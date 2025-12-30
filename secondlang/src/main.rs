//! Secondlang Compiler CLI
//!
//! Usage:
//!   secondlang <file.sl>              JIT compile and run
//!   secondlang --ir <file.sl>         Print LLVM IR
//!   secondlang --ast <file.sl>        Print AST
//!   secondlang --check <file.sl>      Type check only
//!   secondlang -O <file.sl>           Run with optimizations

use std::env;
use std::fs;

use secondlang::{compile_to_ir_with_opts, parse, print_ast, run_with_opts, typecheck};

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

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ir" => show_ir = true,
            "--ast" => show_ast = true,
            "--check" => check_only = true,
            "-O" | "--optimize" => optimize = true,
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

    if check_only {
        // Type check only
        match parse(&source) {
            Ok(mut program) => match typecheck(&mut program) {
                Ok(()) => println!("Type check passed!"),
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
        // Print AST
        match print_ast(&source) {
            Ok(ast) => println!("{}", ast),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else if show_ir {
        // Print LLVM IR
        match compile_to_ir_with_opts(&source, optimize) {
            Ok(ir) => println!("{}", ir),
            Err(e) => {
                eprintln!("Compilation error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // JIT compile and run
        match run_with_opts(&source, optimize) {
            Ok(result) => println!("{}", result),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn print_usage() {
    println!("Secondlang Compiler v0.1.0");
    println!();
    println!("Usage:");
    println!("  secondlang <file.sl>              JIT compile and run");
    println!("  secondlang -O <file.sl>           Run with optimizations");
    println!("  secondlang --ir <file.sl>         Print LLVM IR");
    println!("  secondlang --ir -O <file.sl>      Print optimized LLVM IR");
    println!("  secondlang --ast <file.sl>        Print AST");
    println!("  secondlang --check <file.sl>      Type check only");
    println!("  secondlang --help                 Show this help");
    println!();
    println!("Optimization passes (with -O):");
    println!("  - Constant folding:        1 + 2 * 3 -> 7");
    println!("  - Algebraic simplification: x + 0 -> x, x * 1 -> x");
    println!();
    println!("Example:");
    println!("  secondlang examples/fibonacci.sl");
}
