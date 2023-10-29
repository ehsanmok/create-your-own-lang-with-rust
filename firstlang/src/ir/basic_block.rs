use std::fmt;

use crate::ir::statement::{Statement, Terminator};

pub type BasicBlockId = usize;

/// A basic block in IR
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub statements: Vec<Statement>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "B{}:", self.id)?;
        for stmt in &self.statements {
            writeln!(f, "  {}", stmt)?;
        }
        writeln!(f, "  -> {}", self.terminator)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::typed as ast;
    use crate::ir::statement::{Statement, StatementKind, Terminator};
    use crate::ir::symbol::Symbol;

    #[test]
    fn test_basic_block_empty() {
        let block = BasicBlock {
            id: 0,
            statements: vec![],
            terminator: Terminator::Crash,
        };
        assert_eq!(format!("{}", block), "B0:\n  -> crash\n");
    }

    #[test]
    fn test_basic_block() {
        let mut block = BasicBlock {
            id: 0,
            statements: vec![],
            terminator: Terminator::Crash,
        };
        block.add_statement(Statement::new(
            StatementKind::Unary {
                op: ast::UnaryOp::Minus,
                child: Symbol::new("x", 0),
            },
            None,
        ));
        block.add_statement(Statement::new(
            StatementKind::Binary {
                op: ast::BinaryOp::Add,
                lhs: Symbol::new("x", 0),
                rhs: Symbol::new("y", 0),
            },
            None,
        ));
        block.terminator = Terminator::ProgramReturn(Symbol::new("x", 0));
        let expected_output = "\
B0:
  -x
  x + y
  -> return x
";
        assert_eq!(format!("{}", block), expected_output);
    }
}
