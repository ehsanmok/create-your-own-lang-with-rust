pub mod bytecode;
pub mod opcode;
#[allow(clippy::module_inception)]
pub mod vm;

pub use crate::compiler::vm::{
    bytecode::Bytecode,
    opcode::{make_op, OpCode},
};
