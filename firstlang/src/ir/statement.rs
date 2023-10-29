// 1. in IR starts from this

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::typed as ast;
use crate::ast::typed::Type;
use crate::ir::basic_block::BasicBlockId;
use crate::ir::function::FunctionId;
use crate::ir::program::Program;
use crate::ir::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatementKind {
    Identifier(Symbol),
    Literal(ast::LiteralKind),
    Unary {
        op: ast::UnaryOp,
        child: Symbol,
    },
    Binary {
        op: ast::BinaryOp,
        lhs: Symbol,
        rhs: Symbol,
    },
    Return(Symbol),
    Assignment {
        ident: Symbol,
        value: Symbol,
    },
    Conditional {
        cond: Symbol,
        on_true: Symbol,
        on_false: Symbol,
    },
    Loop {
        cond: Symbol,
        body: Symbol,
    },
    Function {
        name: Symbol,
        params: Vec<Symbol>,
        body: Vec<Symbol>,
    },
    Call {
        func: Symbol,
        args: Vec<Symbol>,
    },
    Block {
        statements: Vec<Symbol>,
    },
}

impl StatementKind {
    pub fn children(&self) -> Vec<&Symbol> {
        use self::StatementKind::*;
        let mut vars = vec![];
        match self {
            Identifier(ref sym) => vars.push(sym),
            Literal(_) => (),
            Unary { ref child, .. } => vars.push(child),
            Binary {
                ref lhs, ref rhs, ..
            } => {
                vars.push(lhs);
                vars.push(rhs);
            }
            Return(ref sym) => vars.push(sym),
            Assignment { ref ident, .. } => vars.push(ident),
            Conditional {
                ref cond,
                ref on_true,
                ref on_false,
            } => {
                vars.push(cond);
                vars.push(on_true);
                vars.push(on_false);
            }
            Loop { ref cond, ref body } => {
                vars.push(cond);
                vars.push(body);
            }
            Function {
                ref name,
                ref params,
                ref body,
            } => {
                vars.push(name);
                vars.extend(params);
                vars.extend(body);
            }
            Call { ref func, ref args } => {
                vars.push(func);
                vars.extend(args);
            }
            Block { ref statements } => vars.extend(statements),
        }
        vars
    }

    pub fn children_mut(&mut self) -> Vec<&mut Symbol> {
        use self::StatementKind::*;
        let mut vars = vec![];
        match self {
            Identifier(ref mut sym) => vars.push(sym),
            Literal(_) => (),
            Unary { ref mut child, .. } => vars.push(child),
            Binary {
                ref mut lhs,
                ref mut rhs,
                ..
            } => {
                vars.push(lhs);
                vars.push(rhs);
            }
            Return(ref mut sym) => vars.push(sym),
            Assignment {
                ref mut ident,
                ref mut value,
            } => {
                vars.push(ident);
                vars.push(value);
            }
            Conditional {
                ref mut cond,
                ref mut on_true,
                ref mut on_false,
            } => {
                vars.push(cond);
                vars.push(on_true);
                vars.push(on_false);
            }
            Loop {
                ref mut cond,
                ref mut body,
            } => {
                vars.push(cond);
                vars.push(body);
            }
            Function {
                ref mut name,
                ref mut params,
                ref mut body,
            } => {
                vars.push(name);
                vars.extend(params);
                vars.extend(body);
            }
            Call {
                ref mut func,
                ref mut args,
            } => {
                vars.push(func);
                vars.extend(args);
            }
            Block { ref mut statements } => vars.extend(statements),
        }
        vars
    }
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::StatementKind::*;
        match *self {
            Identifier(ref sym) => write!(f, "{}", sym),
            Literal(ref lit) => write!(f, "{}", lit),
            Unary { ref op, ref child } => write!(f, "{}{}", op, child),
            Binary {
                ref op,
                ref lhs,
                ref rhs,
            } => write!(f, "{} {} {}", lhs, op, rhs),
            Return(ref sym) => write!(f, "return {}", sym),
            Assignment {
                ref ident,
                ref value,
            } => write!(f, "{} = {}", ident, value),
            Conditional {
                ref cond,
                ref on_true,
                ref on_false,
            } => write!(f, "if ({}) {} else {}", cond, on_true, on_false),
            Loop { ref cond, ref body } => write!(f, "while ({}) {}", cond, body),
            Function {
                ref name,
                ref params,
                ref body,
            } => {
                write!(f, "def {}(", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") {{")?;
                for (i, stmt) in body.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
            Call { ref func, ref args } => {
                write!(f, "{}(", func)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Block { ref statements } => {
                write!(f, "{{")?;
                for (i, stmt) in statements.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// A single statement in the IR, with a RHS statement kind and an optional LHS output Symbol.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Statement {
    pub kind: StatementKind,
    pub output: Option<Symbol>,
}

impl Statement {
    pub fn new(kind: StatementKind, output: Option<Symbol>) -> Statement {
        Statement { kind, output }
    }

    /// Substitutes the symbol `target` with the symbol `with` in this statement.
    ///
    /// This does not substitute the output.
    pub fn substitute_symbol(&mut self, target: &Symbol, with: &Symbol) {
        for child in self.kind.children_mut() {
            if child == target {
                *child = with.clone();
            }
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref sym) = self.output {
            write!(f, "{} = {}", sym, self.kind)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

/// Wrapper type to add statements into a program. This object prevents statements from being
/// produced more than once.

/// A site in the program, identified via a `FunctionId` and `BasicBlockId`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProgramSite(FunctionId, BasicBlockId);

type SiteSymbolMap = HashMap<StatementKind, Symbol>;

pub struct StatementTracker {
    generated: HashMap<ProgramSite, SiteSymbolMap>,
}

impl StatementTracker {
    pub fn new() -> StatementTracker {
        StatementTracker {
            generated: HashMap::default(),
        }
    }
    // TODO: is this used?
    /// Returns a symbol holding the value of the given `StatementKind` in `(func, block)`. If a
    /// symbol representing this statement does not exist, the statement is added to the program
    /// and a new `Symbol` is returned.
    ///
    /// This function should not be used for statements with _named_ parameters (e.g., identifiers,
    /// parameters in a `Lambda`, or names bound using a `Let` statement.)!
    pub fn symbol_for_statement(
        &mut self,
        prog: &mut Program,
        func: FunctionId,
        block: BasicBlockId,
        sym_ty: &Type,
        kind: StatementKind,
    ) -> Symbol {
        let site = ProgramSite(func, block);
        let map = self.generated.entry(site).or_insert_with(HashMap::default);
        // Return the symbol to use.
        match map.entry(kind.clone()) {
            Entry::Occupied(ent) => ent.get().clone(),
            Entry::Vacant(ent) => {
                let res_sym = prog.add_local(func, sym_ty);
                prog.funcs[func].blocks[block]
                    .add_statement(Statement::new(kind, Some((res_sym.clone()))));
                ent.insert(res_sym.clone());
                res_sym
            }
        }
    }

    /// Adds a Statement with a named statement.
    pub fn named_symbol_for_statement(
        &mut self,
        prog: &mut Program,
        func: FunctionId,
        block: BasicBlockId,
        sym_ty: &Type,
        kind: StatementKind,
        named_sym: Symbol,
    ) {
        let site = ProgramSite(func, block);
        let map = self.generated.entry(site).or_insert_with(HashMap::default);

        prog.add_local_named(&named_sym, func, sym_ty);
        prog.funcs[func].blocks[block]
            .add_statement(Statement::new(kind.clone(), Some((named_sym.clone()))));
        map.insert(kind, named_sym);
    }
}

// A terminating statement inside a basic block.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Terminator {
    Branch {
        cond: Symbol,
        on_true: BasicBlockId,
        on_false: BasicBlockId,
    },
    JumpBlock(BasicBlockId),
    ProgramReturn(Symbol),
    EndFunction(Symbol),
    Crash,
}

impl Terminator {
    /// Returns Symbols that the `Terminator` depends on.
    pub fn children(&self) -> Vec<&Symbol> {
        use self::Terminator::*;
        let mut vars = vec![];
        match *self {
            Branch { ref cond, .. } => {
                vars.push(cond);
            }
            ProgramReturn(ref sym) => {
                vars.push(sym);
            }
            EndFunction(ref sym) => vars.push(&sym),
            Crash => (),
            JumpBlock(_) => (),
        };
        vars
    }

    /// Returns mutable references to symbols that the `Terminator` depends on.
    pub fn children_mut(&mut self) -> Vec<&mut Symbol> {
        use self::Terminator::*;
        let mut vars = vec![];
        match *self {
            Branch { ref mut cond, .. } => {
                vars.push(cond);
            }
            ProgramReturn(ref mut sym) => {
                vars.push(sym);
            }
            EndFunction(ref mut sym) => vars.push(sym),
            Crash => (),
            JumpBlock(_) => (),
        };
        vars
    }
}

impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminator::Branch {
                cond,
                on_true,
                on_false,
            } => {
                write!(f, "if {} then B{} else B{}", cond, on_true, on_false)
            }
            Terminator::JumpBlock(block) => write!(f, "jump B{}", block),
            Terminator::ProgramReturn(sym) => write!(f, "return {}", sym),
            Terminator::EndFunction(sym) => write!(f, "end {}", sym),
            Terminator::Crash => write!(f, "crash"),
        }
    }
}
