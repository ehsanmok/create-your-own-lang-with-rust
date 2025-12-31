//! Thirdlang - An object-oriented programming language with explicit memory management
//!
//! Thirdlang extends Secondlang with:
//! - Classes with fields and methods
//! - Constructors (`__init__`) and destructors (`__del__`)
//! - Object creation (`new`) and deletion (`delete`)
//! - Method calls (`obj.method(args)`)
//! - Field access (`obj.field`)
//!
//! # Example
//!
//! ```text
//! class Point {
//!     x: int
//!     y: int
//!
//!     def __init__(self, x: int, y: int) {
//!         self.x = x
//!         self.y = y
//!     }
//!
//!     def distance_squared(self, other: Point) -> int {
//!         dx = self.x - other.x
//!         dy = self.y - other.y
//!         return dx * dx + dy * dy
//!     }
//!
//!     def __del__(self) {
//!         # Cleanup if needed
//!     }
//! }
//!
//! p1 = new Point(0, 0)
//! p2 = new Point(3, 4)
//! dist = p1.distance_squared(p2)  # 25
//! delete p1
//! delete p2
//! dist
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

pub use ast::{ClassDef, Expr, Program, Stmt, TopLevel, TypedExpr};
pub use codegen::{jit_run, jit_run_with_opts, CodeGen};
pub use parser::parse;
pub use typeck::typecheck;
pub use types::{ClassInfo, ClassRegistry, Type};
pub use visitor::{ConstantFolder, ExprVisitor, PrettyPrinter};

/// Convenience function to compile and run source code
pub fn run(source: &str) -> Result<i64, String> {
    let mut program = parse(source)?;
    let classes = typecheck(&mut program)?;
    jit_run(&program, classes)
}

/// Compile and run source code with optimization passes
///
/// # Arguments
/// * `source` - The source code to compile
/// * `passes` - Optimization passes (e.g., "dce,mem2reg,instcombine" or "default<O2>")
pub fn run_optimized(source: &str, passes: &str) -> Result<i64, String> {
    let mut program = parse(source)?;
    let classes = typecheck(&mut program)?;
    jit_run_with_opts(&program, classes, Some(passes))
}

/// Compile source code and return LLVM IR as a string
pub fn compile_to_ir(source: &str) -> Result<String, String> {
    compile_to_ir_with_opts(source, None)
}

/// Compile source code and return LLVM IR, optionally optimized
///
/// # Arguments
/// * `source` - The source code to compile
/// * `passes` - Optional optimization passes
pub fn compile_to_ir_with_opts(source: &str, passes: Option<&str>) -> Result<String, String> {
    use inkwell::context::Context;

    let mut program = parse(source)?;
    let classes = typecheck(&mut program)?;

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "thirdlang", classes);
    codegen.compile(&program)?;

    // Run optimization passes if specified
    if let Some(pass_pipeline) = passes {
        codegen.run_passes(pass_pipeline)?;
    }

    Ok(codegen.print_ir())
}

/// Pretty print the AST
pub fn print_ast(source: &str) -> Result<String, String> {
    let mut program = parse(source)?;
    typecheck(&mut program)?;
    Ok(PrettyPrinter::print_program(&program))
}
