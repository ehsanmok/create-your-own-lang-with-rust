#![allow(unused)]

extern crate pest;

#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod syntax;

pub use ast::typed;
pub use ast::untyped;

pub type Result<T> = anyhow::Result<T>;
