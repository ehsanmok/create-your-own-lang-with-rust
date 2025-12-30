//! Integration tests for Secondlang
//!
//! These tests verify the full compilation pipeline:
//! parsing -> type checking -> LLVM codegen -> JIT execution

use secondlang::{compile_to_ir, parse, run, typecheck};

// =============================================================================
// Type Checking Tests
// =============================================================================

fn typecheck_source(source: &str) -> Result<(), String> {
    let mut program = parse(source)?;
    typecheck(&mut program)
}

#[test]
fn test_typecheck_literals() {
    typecheck_source("42").unwrap();
    typecheck_source("true").unwrap();
    typecheck_source("false").unwrap();
}

#[test]
fn test_typecheck_arithmetic() {
    typecheck_source("1 + 2").unwrap();
    typecheck_source("10 - 5").unwrap();
    typecheck_source("3 * 4").unwrap();
    typecheck_source("20 / 4").unwrap();
}

#[test]
fn test_typecheck_comparison() {
    typecheck_source("1 < 2").unwrap();
    typecheck_source("1 > 2").unwrap();
    typecheck_source("1 == 1").unwrap();
    typecheck_source("1 != 2").unwrap();
}

#[test]
fn test_typecheck_type_error_arithmetic() {
    let result = typecheck_source("1 + true");
    assert!(result.is_err());
}

#[test]
fn test_typecheck_typed_function() {
    let source = r#"
        def add(a: int, b: int) -> int {
            return a + b
        }
        add(1, 2)
    "#;
    typecheck_source(source).unwrap();
}

#[test]
fn test_typecheck_fibonacci() {
    let source = r#"
        def fib(n: int) -> int {
            if (n < 2) {
                return n
            } else {
                return fib(n - 1) + fib(n - 2)
            }
        }
        fib(10)
    "#;
    typecheck_source(source).unwrap();
}

#[test]
fn test_typecheck_wrong_argument_type() {
    let source = r#"
        def add(a: int, b: int) -> int {
            return a + b
        }
        add(1, true)
    "#;
    let result = typecheck_source(source);
    assert!(result.is_err());
}

// =============================================================================
// LLVM IR Generation Tests
// =============================================================================

#[test]
fn test_compile_simple() {
    let source = r#"
        def answer() -> int {
            return 42
        }
        answer()
    "#;
    let ir = compile_to_ir(source).unwrap();
    assert!(ir.contains("define i64 @answer"));
    assert!(ir.contains("ret i64 42"));
}

#[test]
fn test_compile_add() {
    let source = r#"
        def add(a: int, b: int) -> int {
            return a + b
        }
        add(3, 4)
    "#;
    let ir = compile_to_ir(source).unwrap();
    assert!(ir.contains("define i64 @add"));
    assert!(ir.contains("add"));
}

#[test]
fn test_compile_fibonacci() {
    let source = r#"
        def fib(n: int) -> int {
            if (n < 2) {
                return n
            } else {
                return fib(n - 1) + fib(n - 2)
            }
        }
        fib(10)
    "#;
    let ir = compile_to_ir(source).unwrap();
    assert!(ir.contains("define i64 @fib"));
    assert!(ir.contains("call i64 @fib")); // Recursive call
}

// =============================================================================
// JIT Execution Tests
// =============================================================================

#[test]
fn test_jit_simple() {
    let source = r#"
        def answer() -> int {
            return 42
        }
        answer()
    "#;
    assert_eq!(run(source).unwrap(), 42);
}

#[test]
fn test_jit_arithmetic() {
    let source = r#"
        def compute() -> int {
            return 2 + 3 * 4
        }
        compute()
    "#;
    assert_eq!(run(source).unwrap(), 14);
}

#[test]
fn test_jit_add() {
    let source = r#"
        def add(a: int, b: int) -> int {
            return a + b
        }
        add(3, 4)
    "#;
    assert_eq!(run(source).unwrap(), 7);
}

#[test]
fn test_jit_conditional() {
    let source = r#"
        def max(a: int, b: int) -> int {
            if (a > b) {
                return a
            } else {
                return b
            }
        }
        max(10, 20)
    "#;
    assert_eq!(run(source).unwrap(), 20);
}

#[test]
fn test_jit_while_loop() {
    let source = r#"
        def sum_to(n: int) -> int {
            result: int = 0
            i: int = 1
            while (i <= n) {
                result = result + i
                i = i + 1
            }
            return result
        }
        sum_to(10)
    "#;
    assert_eq!(run(source).unwrap(), 55);
}

#[test]
fn test_jit_factorial_recursive() {
    let source = r#"
        def factorial(n: int) -> int {
            if (n <= 1) {
                return 1
            } else {
                return n * factorial(n - 1)
            }
        }
        factorial(5)
    "#;
    assert_eq!(run(source).unwrap(), 120);
}

#[test]
fn test_jit_fibonacci() {
    let source = r#"
        def fib(n: int) -> int {
            if (n < 2) {
                return n
            } else {
                return fib(n - 1) + fib(n - 2)
            }
        }
        fib(10)
    "#;
    assert_eq!(run(source).unwrap(), 55);
}

#[test]
fn test_jit_fibonacci_larger() {
    let source = r#"
        def fib(n: int) -> int {
            if (n < 2) {
                return n
            } else {
                return fib(n - 1) + fib(n - 2)
            }
        }
        fib(20)
    "#;
    assert_eq!(run(source).unwrap(), 6765);
}

#[test]
fn test_jit_multiple_functions() {
    let source = r#"
        def double(x: int) -> int {
            return x * 2
        }
        def quadruple(x: int) -> int {
            return double(double(x))
        }
        quadruple(5)
    "#;
    assert_eq!(run(source).unwrap(), 20);
}

#[test]
fn test_jit_gcd() {
    let source = r#"
        def gcd(a: int, b: int) -> int {
            while (b != 0) {
                temp: int = b
                b = a % b
                a = temp
            }
            return a
        }
        gcd(48, 18)
    "#;
    assert_eq!(run(source).unwrap(), 6);
}

#[test]
fn test_jit_power() {
    let source = r#"
        def power(base: int, exp: int) -> int {
            if (exp == 0) {
                return 1
            } else {
                return base * power(base, exp - 1)
            }
        }
        power(2, 10)
    "#;
    assert_eq!(run(source).unwrap(), 1024);
}
