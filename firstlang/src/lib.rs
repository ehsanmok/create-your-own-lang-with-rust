//! Firstlang - A simple interpreted programming language
//!
//! Firstlang is a Python-like language that supports:
//! - Integer and boolean values
//! - Variables and assignment
//! - Functions with recursion
//! - Control flow (if/else, while)
//!
//! This is an educational language designed to teach programming language
//! implementation concepts without the complexity of type systems or compilation.
//!
//! # Example
//!
//! ```text
//! # Recursive Fibonacci
//! def fib(n) {
//!     if (n < 2) {
//!         return n
//!     } else {
//!         return fib(n - 1) + fib(n - 2)
//!     }
//! }
//! fib(10)  # Returns 55
//! ```

extern crate pest;

#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod interpreter;
pub mod parser;

pub use ast::{Expr, Program, Stmt};
pub use interpreter::{Interpreter, Value};
pub use parser::parse;

/// Convenience function to run source code and get the result
pub fn run(source: &str) -> Result<Value, String> {
    let program = parse(source)?;
    let mut interpreter = Interpreter::new();
    interpreter.run(&program)
}
