//! Secondlang - A typed, compiled programming language
//!
//! Secondlang extends Firstlang with a type system and LLVM code generation.
//! It demonstrates how static types enable compilation to native code.
//!
//! # Features
//!
//! - Same syntax as Firstlang, plus type annotations
//! - Static type checking and type inference
//! - AST optimization passes (constant folding, algebraic simplification)
//! - LLVM IR code generation
//! - JIT compilation to native code
//!
//! # Example
//!
//! ```text
//! # Typed Fibonacci
//! def fib(n: int) -> int {
//!     if (n < 2) {
//!         return n
//!     } else {
//!         return fib(n - 1) + fib(n - 2)
//!     }
//! }
//! fib(10)  # Compiled to native code!
//! ```

extern crate pest;

#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod codegen;
pub mod parser;
pub mod typeck;
pub mod types;
pub mod visitor;

pub use ast::{Expr, Program, Stmt, TypedExpr};
pub use codegen::{jit_run, CodeGen};
pub use parser::parse;
pub use typeck::typecheck;
pub use types::Type;
pub use visitor::{AlgebraicSimplifier, ConstantFolder, ExprVisitor, PrettyPrinter};

/// Convenience function to compile and run source code
pub fn run(source: &str) -> Result<i64, String> {
    run_with_opts(source, false)
}

/// Compile and run with optional optimizations
pub fn run_with_opts(source: &str, optimize: bool) -> Result<i64, String> {
    let mut program = parse(source)?;
    typecheck(&mut program)?;

    if optimize {
        program = optimize_program(program);
    }

    jit_run(&program)
}

/// Apply optimization passes to the program
pub fn optimize_program(program: Program) -> Program {
    // Pass 1: Constant folding
    let program = ConstantFolder::fold_program(&program);
    // Pass 2: Algebraic simplification
    AlgebraicSimplifier::simplify_program(&program)
}

/// Pretty print the AST
pub fn print_ast(source: &str) -> Result<String, String> {
    let mut program = parse(source)?;
    typecheck(&mut program)?;
    Ok(PrettyPrinter::print_program(&program))
}

/// Compile source code and return LLVM IR as a string
pub fn compile_to_ir(source: &str) -> Result<String, String> {
    compile_to_ir_with_opts(source, false)
}

/// Compile to IR with optional optimizations
pub fn compile_to_ir_with_opts(source: &str, optimize: bool) -> Result<String, String> {
    use inkwell::context::Context;

    let mut program = parse(source)?;
    typecheck(&mut program)?;

    if optimize {
        program = optimize_program(program);
    }

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "secondlang");
    codegen.compile(&program)?;

    Ok(codegen.print_ir())
}
