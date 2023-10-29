use std::collections::BTreeMap;
use std::fmt;

use crate::ast::typed::Type;
use crate::ir::basic_block::{BasicBlock, BasicBlockId};
use crate::ir::statement::Terminator;
use crate::ir::symbol::Symbol;

pub type FunctionId = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    pub id: FunctionId,
    pub params: BTreeMap<Symbol, Type>,
    pub locals: BTreeMap<Symbol, Type>,
    pub blocks: Vec<BasicBlock>,
    pub ret_ty: Type,
}

impl Function {
    /// Add a new basic block and return its block ID.
    pub fn add_block(&mut self) -> BasicBlockId {
        let block = BasicBlock {
            id: self.blocks.len(),
            statements: vec![],
            terminator: Terminator::Crash,
        };
        self.blocks.push(block);
        self.blocks.len() - 1
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "F{}:", self.id)?;
        for (sym, ty) in &self.params {
            writeln!(f, "  param {} {}", sym, ty)?;
        }
        for (sym, ty) in &self.locals {
            writeln!(f, "  local {} {}", sym, ty)?;
        }
        for block in &self.blocks {
            writeln!(f, "{}", block)?;
        }
        writeln!(f, "  ret {}", self.ret_ty)?;
        Ok(())
    }
}
