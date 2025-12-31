//! Integration tests for Thirdlang
//!
//! These tests verify the full compilation pipeline:
//! parsing -> type checking -> LLVM codegen -> JIT execution

use thirdlang::{compile_to_ir, parse, run, run_optimized, typecheck};

// =============================================================================
// Parsing Tests
// =============================================================================

#[test]
fn test_parse_class() {
    let source = r#"
        class Point {
            x: int
            y: int
        }
    "#;
    parse(source).unwrap();
}

#[test]
fn test_parse_class_with_methods() {
    let source = r#"
        class Point {
            x: int

            def __init__(self, x: int) {
                self.x = x
            }

            def get_x(self) -> int {
                return self.x
            }
        }
    "#;
    parse(source).unwrap();
}

#[test]
fn test_parse_new_delete() {
    let source = r#"
        class Point { x: int }
        p = new Point()
        delete p
    "#;
    parse(source).unwrap();
}

// =============================================================================
// Type Checking Tests
// =============================================================================

fn typecheck_source(source: &str) -> Result<(), String> {
    let mut program = parse(source)?;
    typecheck(&mut program)?;
    Ok(())
}

#[test]
fn test_typecheck_class() {
    let source = r#"
        class Point {
            x: int
            y: int

            def __init__(self, x: int, y: int) {
                self.x = x
                self.y = y
            }
        }
    "#;
    typecheck_source(source).unwrap();
}

#[test]
fn test_typecheck_method_call() {
    let source = r#"
        class Counter {
            value: int
            def get(self) -> int { return self.value }
        }
        c = new Counter()
        c.get()
    "#;
    typecheck_source(source).unwrap();
}

#[test]
fn test_typecheck_field_access() {
    let source = r#"
        class Point {
            x: int
            def __init__(self, x: int) { self.x = x }
        }
        p = new Point(42)
        p.x
    "#;
    typecheck_source(source).unwrap();
}

#[test]
fn test_typecheck_wrong_constructor_args() {
    let source = r#"
        class Point {
            x: int
            def __init__(self, x: int) { self.x = x }
        }
        p = new Point(true)
    "#;
    assert!(typecheck_source(source).is_err());
}

#[test]
fn test_typecheck_wrong_field_type() {
    let source = r#"
        class Point {
            x: int
            def __init__(self) { self.x = true }
        }
    "#;
    assert!(typecheck_source(source).is_err());
}

#[test]
fn test_typecheck_unknown_field() {
    let source = r#"
        class Point { x: int }
        p = new Point()
        p.y
    "#;
    assert!(typecheck_source(source).is_err());
}

#[test]
fn test_typecheck_unknown_method() {
    let source = r#"
        class Point { x: int }
        p = new Point()
        p.foo()
    "#;
    assert!(typecheck_source(source).is_err());
}

#[test]
fn test_typecheck_delete_non_class() {
    let source = r#"
        x: int = 42
        delete x
    "#;
    assert!(typecheck_source(source).is_err());
}

// =============================================================================
// Code Generation Tests
// =============================================================================

#[test]
fn test_compile_simple_class() {
    let source = r#"
        class Point {
            x: int
            def __init__(self, x: int) { self.x = x }
            def get_x(self) -> int { return self.x }
        }
        p = new Point(42)
        p.get_x()
    "#;
    let ir = compile_to_ir(source).unwrap();
    assert!(ir.contains("Point____init__"));
    assert!(ir.contains("Point__get_x"));
    assert!(ir.contains("@malloc"));
}

#[test]
fn test_compile_delete() {
    let source = r#"
        class Point { x: int }
        p = new Point()
        delete p
        42
    "#;
    let ir = compile_to_ir(source).unwrap();
    assert!(ir.contains("@free"));
}

// =============================================================================
// JIT Execution Tests
// =============================================================================

// ANCHOR: test_simple_class
#[test]
fn test_jit_simple_class() {
    let source = r#"
        class Point {
            x: int
            def __init__(self, x: int) { self.x = x }
            def get_x(self) -> int { return self.x }
        }
        p = new Point(42)
        result = p.get_x()
        delete p
        result
    "#;
    assert_eq!(run(source).unwrap(), 42);
}
// ANCHOR_END: test_simple_class

#[test]
fn test_jit_counter() {
    let source = r#"
        class Counter {
            value: int

            def __init__(self, start: int) {
                self.value = start
            }

            def increment(self) -> int {
                self.value = self.value + 1
                return self.value
            }

            def get(self) -> int {
                return self.value
            }
        }

        c = new Counter(10)
        c.increment()
        c.increment()
        c.increment()
        result = c.get()
        delete c
        result
    "#;
    assert_eq!(run(source).unwrap(), 13);
}

#[test]
fn test_jit_point_distance() {
    let source = r#"
        class Point {
            x: int
            y: int

            def __init__(self, x: int, y: int) {
                self.x = x
                self.y = y
            }

            def distance_squared(self, other: Point) -> int {
                dx = self.x - other.x
                dy = self.y - other.y
                return dx * dx + dy * dy
            }
        }

        p1 = new Point(0, 0)
        p2 = new Point(3, 4)
        result = p1.distance_squared(p2)
        delete p1
        delete p2
        result
    "#;
    assert_eq!(run(source).unwrap(), 25);
}

#[test]
fn test_jit_multiple_objects() {
    let source = r#"
        class Box {
            value: int
            def __init__(self, v: int) { self.value = v }
            def get(self) -> int { return self.value }
        }

        b1 = new Box(10)
        b2 = new Box(20)
        b3 = new Box(30)

        sum = b1.get() + b2.get() + b3.get()

        delete b1
        delete b2
        delete b3

        sum
    "#;
    assert_eq!(run(source).unwrap(), 60);
}

#[test]
fn test_jit_destructor() {
    let source = r#"
        class Resource {
            id: int

            def __init__(self, id: int) {
                self.id = id
            }

            def get_id(self) -> int {
                return self.id
            }

            def __del__(self) {
                # Destructor called on delete
            }
        }

        r = new Resource(99)
        id = r.get_id()
        delete r
        id
    "#;
    assert_eq!(run(source).unwrap(), 99);
}

#[test]
fn test_jit_method_with_args() {
    let source = r#"
        class Calculator {
            result: int

            def __init__(self) {
                self.result = 0
            }

            def add(self, n: int) -> int {
                self.result = self.result + n
                return self.result
            }

            def multiply(self, n: int) -> int {
                self.result = self.result * n
                return self.result
            }
        }

        calc = new Calculator()
        calc.add(10)
        calc.add(5)
        calc.multiply(2)
        result = calc.result
        delete calc
        result
    "#;
    assert_eq!(run(source).unwrap(), 30);
}

#[test]
fn test_jit_conditional_in_method() {
    let source = r#"
        class Number {
            value: int

            def __init__(self, v: int) {
                self.value = v
            }

            def abs(self) -> int {
                if (self.value < 0) {
                    return 0 - self.value
                } else {
                    return self.value
                }
            }
        }

        n = new Number(0 - 42)
        result = n.abs()
        delete n
        result
    "#;
    assert_eq!(run(source).unwrap(), 42);
}

#[test]
fn test_jit_loop_in_method() {
    let source = r#"
        class Accumulator {
            total: int

            def __init__(self) {
                self.total = 0
            }

            def sum_to(self, n: int) -> int {
                i: int = 1
                while (i <= n) {
                    self.total = self.total + i
                    i = i + 1
                }
                return self.total
            }
        }

        acc = new Accumulator()
        result = acc.sum_to(10)
        delete acc
        result
    "#;
    assert_eq!(run(source).unwrap(), 55);
}

#[test]
fn test_jit_class_with_functions() {
    let source = r#"
        def square(n: int) -> int {
            return n * n
        }

        class Point {
            x: int
            y: int

            def __init__(self, x: int, y: int) {
                self.x = x
                self.y = y
            }

            def magnitude_squared(self) -> int {
                return square(self.x) + square(self.y)
            }
        }

        p = new Point(3, 4)
        result = p.magnitude_squared()
        delete p
        result
    "#;
    assert_eq!(run(source).unwrap(), 25);
}

// ============================================================================
// Optimization Tests
// ============================================================================

#[test]
fn test_run_optimized() {
    let source = r#"
        class Counter {
            value: int

            def __init__(self, start: int) {
                self.value = start
            }

            def increment(self) -> int {
                self.value = self.value + 1
                return self.value
            }

            def get(self) -> int {
                return self.value
            }
        }

        c = new Counter(0)
        c.increment()
        c.increment()
        c.increment()
        result = c.get()
        delete c
        result
    "#;

    // Test without optimization
    assert_eq!(run(source).unwrap(), 3);

    // Test with optimization
    assert_eq!(run_optimized(source, "dce,mem2reg,instcombine").unwrap(), 3);
}

#[test]
fn test_run_optimized_with_dead_code() {
    // This test verifies DCE removes unused computations
    let source = r#"
        class Point {
            x: int
            y: int

            def __init__(self, x: int, y: int) {
                self.x = x
                self.y = y
            }

            def get_x(self) -> int {
                return self.x
            }
        }

        p = new Point(42, 100)
        # The y field is set but never used - DCE should handle this
        unused = p.get_x()
        result = p.get_x()
        delete p
        result
    "#;

    assert_eq!(
        run_optimized(source, "dce,mem2reg,instcombine").unwrap(),
        42
    );
}

// ANCHOR: test_optimization_pipeline
#[test]
fn test_optimization_pipeline() {
    use thirdlang::compile_to_ir_with_opts;

    let source = r#"
        class Simple {
            value: int

            def __init__(self, v: int) {
                self.value = v
            }

            def get(self) -> int {
                return self.value
            }
        }

        s = new Simple(100)
        result = s.get()
        delete s
        result
    "#;

    // Get unoptimized IR
    let unopt_ir = compile_to_ir_with_opts(source, None).unwrap();

    // Get optimized IR
    let opt_ir = compile_to_ir_with_opts(source, Some("mem2reg,dce,instcombine")).unwrap();

    // Optimized IR should be shorter (fewer alloca instructions)
    assert!(
        opt_ir.len() < unopt_ir.len(),
        "Optimized IR should be smaller"
    );

    // Unoptimized should have alloca for parameters
    assert!(
        unopt_ir.contains("alloca"),
        "Unoptimized IR should have allocas"
    );
}
// ANCHOR_END: test_optimization_pipeline

#[test]
fn test_default_o2_pipeline() {
    // Test the standard O2 pipeline
    let source = r#"
        class Box {
            value: int

            def __init__(self, v: int) {
                self.value = v
            }

            def double(self) -> int {
                return self.value * 2
            }
        }

        b = new Box(21)
        result = b.double()
        delete b
        result
    "#;

    assert_eq!(run_optimized(source, "default<O2>").unwrap(), 42);
}
