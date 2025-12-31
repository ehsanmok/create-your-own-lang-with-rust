//! Thirdlang Compiler CLI
//!
//! Usage:
//!   thirdlang <file.tl>                    JIT compile and run
//!   thirdlang -O <file.tl>                 Run with default optimizations
//!   thirdlang --passes <passes> <file.tl>  Run with custom optimization passes
//!   thirdlang --ir <file.tl>               Print LLVM IR (unoptimized)
//!   thirdlang --ir -O <file.tl>            Print LLVM IR (optimized)
//!   thirdlang --ast <file.tl>              Print AST
//!   thirdlang --check <file.tl>            Type check only

use std::env;
use std::fs;

use thirdlang::{compile_to_ir_with_opts, parse, print_ast, run, run_optimized, typecheck};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let mut filename = None;
    let mut show_ir = false;
    let mut show_ast = false;
    let mut check_only = false;
    let mut optimize = false;
    let mut custom_passes: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ir" => show_ir = true,
            "--ast" => show_ast = true,
            "--check" => check_only = true,
            "-O" | "--optimize" => optimize = true,
            "--passes" => {
                i += 1;
                if i < args.len() {
                    custom_passes = Some(args[i].clone());
                    optimize = true;
                } else {
                    eprintln!("Error: --passes requires a pass pipeline argument");
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            arg if !arg.starts_with('-') => {
                filename = Some(arg.to_string());
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let filename = match filename {
        Some(f) => f,
        None => {
            eprintln!("Error: No input file specified");
            print_usage();
            std::process::exit(1);
        }
    };

    let source = match fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    // Determine the pass pipeline
    let passes = if let Some(p) = custom_passes {
        Some(p)
    } else if optimize {
        // Default optimization pipeline for teaching
        Some("dce,mem2reg,instcombine,simplifycfg".to_string())
    } else {
        None
    };

    if check_only {
        match parse(&source) {
            Ok(mut program) => match typecheck(&mut program) {
                Ok(_) => println!("Type check passed!"),
                Err(e) => {
                    eprintln!("Type error: {}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Parse error: {}", e);
                std::process::exit(1);
            }
        }
    } else if show_ast {
        match print_ast(&source) {
            Ok(ast) => println!("{}", ast),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else if show_ir {
        match compile_to_ir_with_opts(&source, passes.as_deref()) {
            Ok(ir) => println!("{}", ir),
            Err(e) => {
                eprintln!("Compilation error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let result = if let Some(p) = passes {
            run_optimized(&source, &p)
        } else {
            run(&source)
        };

        match result {
            Ok(result) => println!("{}", result),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn print_usage() {
    println!("Thirdlang Compiler v0.1.0");
    println!();
    println!("An object-oriented language with explicit memory management.");
    println!();
    println!("Usage:");
    println!("  thirdlang <file.tl>                    JIT compile and run");
    println!("  thirdlang -O <file.tl>                 Run with optimizations");
    println!("  thirdlang --passes <passes> <file.tl>  Run with custom passes");
    println!("  thirdlang --ir <file.tl>               Print LLVM IR");
    println!("  thirdlang --ir -O <file.tl>            Print optimized LLVM IR");
    println!("  thirdlang --ast <file.tl>              Print AST");
    println!("  thirdlang --check <file.tl>            Type check only");
    println!("  thirdlang --help                       Show this help");
    println!();
    println!("Optimization Passes (for --passes):");
    println!("  dce           Dead Code Elimination");
    println!("  mem2reg       Promote allocas to SSA registers");
    println!("  instcombine   Combine redundant instructions");
    println!("  simplifycfg   Simplify control flow graph");
    println!("  gvn           Global Value Numbering");
    println!("  default<O0>   No optimization (verify only)");
    println!("  default<O1>   Light optimization");
    println!("  default<O2>   Standard optimization (recommended)");
    println!("  default<O3>   Aggressive optimization");
    println!();
    println!("Features:");
    println!("  - Classes with fields and methods");
    println!("  - Constructors (__init__) and destructors (__del__)");
    println!("  - Object creation (new) and deletion (delete)");
    println!("  - Static type checking with inference");
    println!("  - JIT compilation via LLVM");
    println!("  - LLVM New Pass Manager for optimization");
    println!();
    println!("Examples:");
    println!("  thirdlang examples/point.tl");
    println!("  thirdlang -O examples/counter.tl");
    println!("  thirdlang --ir -O examples/point.tl");
    println!("  thirdlang --passes \"dce,mem2reg\" examples/point.tl");
}
