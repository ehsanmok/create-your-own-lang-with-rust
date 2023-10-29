use std::collections::BTreeMap;
use std::fmt;

use crate::ast::typed::{Parameter, Type};
use crate::ir::basic_block::BasicBlockId;
use crate::ir::function::{Function, FunctionId};
use crate::ir::statement::{Statement, StatementKind};
use crate::ir::symbol::{Symbol, SymbolGenerator};

#[derive(Debug)]
pub struct Program {
    /// funcs[0] is the entry function
    pub funcs: Vec<Function>,
    pub ret_ty: Type,
    pub top_params: Vec<Parameter>,
    sym_gen: SymbolGenerator,
}

impl Program {
    pub fn new(ret_type: &Type, top_params: &[Parameter]) -> Program {
        let mut prog = Program {
            funcs: vec![],
            ret_ty: ret_type.clone(),
            top_params: top_params.to_vec(),
            sym_gen: SymbolGenerator::new(),
        };
        // Add the main function.
        prog.add_func();
        prog
    }

    pub fn add_func(&mut self) -> FunctionId {
        let func = Function {
            id: self.funcs.len(),
            params: BTreeMap::new(),
            locals: BTreeMap::new(),
            blocks: vec![],
            ret_ty: Type::Unknown,
        };
        self.funcs.push(func);
        self.funcs.len() - 1
    }

    /// Add a local variable of the given type and return a symbol for it.
    pub fn add_local(&mut self, func: FunctionId, ty: &Type) -> Symbol {
        let sym = self.sym_gen.new_symbol(format!("fn{}_tmp", func).as_str());
        self.funcs[func].locals.insert(sym.clone(), ty.clone());
        sym
    }

    /// Add a local variable of the given type and name
    pub fn add_local_named(&mut self, sym: &Symbol, func: FunctionId, ty: &Type) {
        self.funcs[func].locals.insert(sym.clone(), ty.clone());
    }

    /// Add block
    pub fn add_block(&mut self, func: FunctionId) -> BasicBlockId {
        self.funcs[func].add_block()
    }

    /// Add a statement to the given basic block and return a symbol for it.
    /// The statement is added to the end of the block.
    pub fn add_statement(
        &mut self,
        func: FunctionId,
        block: BasicBlockId,
        stmt: StatementKind,
    ) -> Symbol {
        let sym = self.sym_gen.new_symbol(format!("fn{}_tmp", func).as_str());
        self.funcs[func].blocks[block]
            .statements
            .push(Statement::new(stmt, Some(sym.clone())));
        sym
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display top-level parameters if they exist
        if !self.top_params.is_empty() {
            writeln!(f, "Top Parameters:")?;
            for param in &self.top_params {
                writeln!(f, "  {}", param)?;
            }
            writeln!(f)?;
        }

        // Display each function in the program
        for func in &self.funcs {
            write!(f, "{}", func)?;
        }

        // Display the return type of the program
        writeln!(f, "Return Type: {}", self.ret_ty)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::typed as ast;
    use crate::ast::typed::{Parameter, Type};
    use crate::ir::basic_block::BasicBlock;
    use crate::ir::function::Function;
    use crate::ir::statement::{Statement, StatementKind, Terminator};
    use crate::ir::symbol::Symbol;

    #[test]
    fn test_program_ir() {
        // 1. Create a new program with some top-level parameters
        let mut program = Program::new(
            &Type::Unknown,
            &[
                Parameter::new(
                    ast::Identifier::new("a".to_string()),
                    ast::Type::Scalar(ast::ScalarKind::Int),
                ),
                Parameter::new(
                    ast::Identifier::new("b".to_string()),
                    ast::Type::Scalar(ast::ScalarKind::Int),
                ),
            ],
        );

        // 2. Add a function to the program
        let func_id = program.add_func();

        // 3. Add some local variables to the function
        let x = program.add_local(func_id, &Type::Unknown);
        let y = program.add_local(func_id, &Type::Unknown);

        // 4. Create a basic block with some statements and add it to the function
        let mut block = BasicBlock {
            id: 0,
            statements: vec![
                Statement::new(
                    StatementKind::Unary {
                        op: ast::UnaryOp::Minus,
                        child: Symbol::new("a", 0),
                    },
                    Some(x.clone()),
                ),
                Statement::new(
                    StatementKind::Binary {
                        op: ast::BinaryOp::Add,
                        lhs: x.clone(),
                        rhs: Symbol::new("b", 0),
                    },
                    Some(y.clone()),
                ),
            ],
            terminator: Terminator::ProgramReturn(x),
        };

        program.funcs[func_id].blocks.push(block);

        // 5. Check the formatted output of the program
        let expected_output = "\
Top Parameters:
  a: int
  b: int

F0:
  ret unknown
F1:
  local fn1_tmp unknown
  local fn1_tmp__1 unknown
B0:
  fn1_tmp = -a
  fn1_tmp__1 = fn1_tmp + b
  -> return fn1_tmp

  ret unknown
Return Type: unknown
";
        assert_eq!(format!("{}", program), expected_output);
    }

    #[test]
    fn test_program_fib_ir() {
        let mut program = Program::new(&Type::Scalar(ast::ScalarKind::Int), &[]);

        // Define the Fibonacci function
        let fib_func = program.add_func();
        let block_id = program.add_block(fib_func);

        // Add parameter n
        let n = program.add_local(fib_func, &Type::Scalar(ast::ScalarKind::Int));

        // Define the base case: if n <= 1, return n;
        let rhs_value = program.add_local(fib_func, &Type::Scalar(ast::ScalarKind::Int));
        let cond_check = program.add_statement(
            fib_func,
            block_id,
            StatementKind::Binary {
                op: ast::BinaryOp::LessThan,
                lhs: n.clone(),
                rhs: rhs_value,
            },
        );

        // Recursive calls
        let sub_rhs_1 = program.add_local(fib_func, &Type::Scalar(ast::ScalarKind::Int));
        let arg_value_1 = program.add_statement(
            fib_func,
            block_id,
            StatementKind::Binary {
                op: ast::BinaryOp::Sub,
                lhs: n.clone(),
                rhs: sub_rhs_1,
            },
        );

        let fib_n_minus_1 = program.add_statement(
            fib_func,
            block_id,
            StatementKind::Call {
                func: fib_func.into(),
                args: vec![arg_value_1],
            },
        );

        let sub_rhs_2 = program.add_local(fib_func, &Type::Scalar(ast::ScalarKind::Int));
        let arg_value_2 = program.add_statement(
            fib_func,
            block_id,
            StatementKind::Binary {
                op: ast::BinaryOp::Sub,
                lhs: n.clone(),
                rhs: sub_rhs_2,
            },
        );

        let fib_n_minus_2 = program.add_statement(
            fib_func,
            block_id,
            StatementKind::Call {
                func: fib_func.into(),
                args: vec![arg_value_2],
            },
        );

        // Add the sum of the recursive calls to the function body
        let result = program.add_statement(
            fib_func,
            block_id,
            StatementKind::Binary {
                op: ast::BinaryOp::Add,
                lhs: fib_n_minus_1,
                rhs: fib_n_minus_2,
            },
        );

        // Change the terminator to return the computed Fibonacci value
        program.funcs[fib_func].blocks[block_id].terminator = Terminator::ProgramReturn(result);

        let expected_ir = "\
F0:
  ret unknown
F1:
  local fn1_tmp int
  local fn1_tmp__1 int
  local fn1_tmp__3 int
  local fn1_tmp__6 int
B0:
  fn1_tmp__2 = fn1_tmp < fn1_tmp__1
  fn1_tmp__4 = fn1_tmp - fn1_tmp__3
  fn1_tmp__5 = F1__1(fn1_tmp__4)
  fn1_tmp__7 = fn1_tmp - fn1_tmp__6
  fn1_tmp__8 = F1__1(fn1_tmp__7)
  fn1_tmp__9 = fn1_tmp__5 + fn1_tmp__8
  -> return fn1_tmp__9

  ret unknown
Return Type: int
";
        assert_eq!(format!("{}", program), expected_ir);
    }
}
