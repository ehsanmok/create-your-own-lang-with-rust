//! End-to-End Integration Tests for Firstlang
//!
//! These tests demonstrate the full capabilities of Firstlang
//! and serve as examples for the book.

use firstlang::{run, Value};

// =============================================================================
// Basic Expressions
// =============================================================================

#[test]
fn test_integer_literals() {
    assert_eq!(run("0").unwrap(), Value::Int(0));
    assert_eq!(run("42").unwrap(), Value::Int(42));
    assert_eq!(run("999999").unwrap(), Value::Int(999999));
}

#[test]
fn test_boolean_literals() {
    assert_eq!(run("true").unwrap(), Value::Bool(true));
    assert_eq!(run("false").unwrap(), Value::Bool(false));
}

#[test]
fn test_arithmetic_operators() {
    assert_eq!(run("2 + 3").unwrap(), Value::Int(5));
    assert_eq!(run("10 - 4").unwrap(), Value::Int(6));
    assert_eq!(run("6 * 7").unwrap(), Value::Int(42));
    assert_eq!(run("20 / 4").unwrap(), Value::Int(5));
    assert_eq!(run("17 % 5").unwrap(), Value::Int(2));
}

#[test]
fn test_operator_precedence() {
    // Multiplication before addition
    assert_eq!(run("2 + 3 * 4").unwrap(), Value::Int(14));
    // Parentheses override precedence
    assert_eq!(run("(2 + 3) * 4").unwrap(), Value::Int(20));
    // Left-to-right for same precedence
    assert_eq!(run("10 - 4 - 2").unwrap(), Value::Int(4));
}

#[test]
fn test_unary_operators() {
    assert_eq!(run("-5").unwrap(), Value::Int(-5));
    assert_eq!(run("--5").unwrap(), Value::Int(5));
    assert_eq!(run("!true").unwrap(), Value::Bool(false));
    assert_eq!(run("!false").unwrap(), Value::Bool(true));
    assert_eq!(run("!!true").unwrap(), Value::Bool(true));
}

#[test]
fn test_comparison_operators() {
    assert_eq!(run("1 < 2").unwrap(), Value::Bool(true));
    assert_eq!(run("2 < 1").unwrap(), Value::Bool(false));
    assert_eq!(run("2 > 1").unwrap(), Value::Bool(true));
    assert_eq!(run("1 > 2").unwrap(), Value::Bool(false));
    assert_eq!(run("1 <= 1").unwrap(), Value::Bool(true));
    assert_eq!(run("1 <= 2").unwrap(), Value::Bool(true));
    assert_eq!(run("2 >= 2").unwrap(), Value::Bool(true));
    assert_eq!(run("1 >= 2").unwrap(), Value::Bool(false));
    assert_eq!(run("42 == 42").unwrap(), Value::Bool(true));
    assert_eq!(run("42 == 43").unwrap(), Value::Bool(false));
    assert_eq!(run("42 != 43").unwrap(), Value::Bool(true));
    assert_eq!(run("true == true").unwrap(), Value::Bool(true));
    assert_eq!(run("true != false").unwrap(), Value::Bool(true));
}

// =============================================================================
// Variables
// =============================================================================

#[test]
fn test_variable_assignment() {
    assert_eq!(run("x = 42\nx").unwrap(), Value::Int(42));
}

#[test]
fn test_variable_reassignment() {
    let source = r#"
        x = 10
        x = 20
        x
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(20));
}

#[test]
fn test_variable_in_expressions() {
    let source = r#"
        a = 10
        b = 20
        a + b * 2
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(50));
}

#[test]
fn test_compound_assignment_pattern() {
    let source = r#"
        x = 1
        x = x + 1
        x = x * 2
        x
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(4));
}

// =============================================================================
// Functions
// =============================================================================

#[test]
fn test_function_no_params() {
    let source = r#"
        def answer() {
            return 42
        }
        answer()
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(42));
}

#[test]
fn test_function_with_parameters() {
    let source = r#"
        def add(a, b) {
            return a + b
        }
        add(3, 4)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(7));
}

#[test]
fn test_function_with_local_variables() {
    let source = r#"
        def compute(x) {
            doubled = x * 2
            tripled = x * 3
            return doubled + tripled
        }
        compute(10)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(50));
}

#[test]
fn test_multiple_functions() {
    let source = r#"
        def square(x) {
            return x * x
        }
        def cube(x) {
            return x * x * x
        }
        square(3) + cube(2)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(17)); // 9 + 8
}

#[test]
fn test_function_calling_function() {
    let source = r#"
        def double(x) {
            return x * 2
        }
        def quadruple(x) {
            return double(double(x))
        }
        quadruple(5)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(20));
}

// =============================================================================
// Control Flow - Conditionals
// =============================================================================

#[test]
fn test_if_true_branch() {
    let source = r#"
        if (true) {
            42
        } else {
            0
        }
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(42));
}

#[test]
fn test_if_false_branch() {
    let source = r#"
        if (false) {
            42
        } else {
            0
        }
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(0));
}

#[test]
fn test_if_with_comparison() {
    let source = r#"
        x = 10
        if (x > 5) {
            1
        } else {
            0
        }
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(1));
}

#[test]
fn test_nested_conditionals() {
    let source = r#"
        def classify(n) {
            if (n < 0) {
                return -1
            } else {
                if (n == 0) {
                    return 0
                } else {
                    return 1
                }
            }
        }
        classify(-5) + classify(0) + classify(10)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(0)); // -1 + 0 + 1
}

#[test]
fn test_max_function() {
    let source = r#"
        def max(a, b) {
            if (a > b) {
                return a
            } else {
                return b
            }
        }
        max(10, 20)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(20));
}

#[test]
fn test_abs_function() {
    let source = r#"
        def abs(x) {
            if (x < 0) {
                return -x
            } else {
                return x
            }
        }
        abs(-42)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(42));
}

// =============================================================================
// Control Flow - Loops
// =============================================================================

#[test]
fn test_while_loop() {
    let source = r#"
        x = 0
        while (x < 5) {
            x = x + 1
        }
        x
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(5));
}

#[test]
fn test_while_loop_sum() {
    let source = r#"
        sum = 0
        i = 1
        while (i <= 10) {
            sum = sum + i
            i = i + 1
        }
        sum
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(55)); // 1+2+...+10
}

#[test]
fn test_while_never_executes() {
    let source = r#"
        x = 0
        while (false) {
            x = x + 1
        }
        x
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(0));
}

#[test]
fn test_countdown() {
    let source = r#"
        def countdown(n) {
            count = 0
            while (n > 0) {
                count = count + 1
                n = n - 1
            }
            return count
        }
        countdown(10)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(10));
}

// =============================================================================
// Recursion
// =============================================================================

#[test]
fn test_factorial_recursive() {
    let source = r#"
        def factorial(n) {
            if (n <= 1) {
                return 1
            } else {
                return n * factorial(n - 1)
            }
        }
        factorial(5)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(120));
}

#[test]
fn test_factorial_iterative() {
    let source = r#"
        def factorial(n) {
            result = 1
            while (n > 1) {
                result = result * n
                n = n - 1
            }
            return result
        }
        factorial(5)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(120));
}

#[test]
fn test_fibonacci_recursive() {
    let source = r#"
        def fib(n) {
            if (n < 2) {
                return n
            } else {
                return fib(n - 1) + fib(n - 2)
            }
        }
        fib(10)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(55));
}

#[test]
fn test_fibonacci_iterative() {
    let source = r#"
        def fib(n) {
            if (n < 2) {
                return n
            } else {
                a = 0
                b = 1
                i = 2
                while (i <= n) {
                    temp = a + b
                    a = b
                    b = temp
                    i = i + 1
                }
                return b
            }
        }
        fib(10)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(55));
}

#[test]
fn test_fibonacci_larger() {
    let source = r#"
        def fib(n) {
            if (n < 2) {
                return n
            } else {
                a = 0
                b = 1
                i = 2
                while (i <= n) {
                    temp = a + b
                    a = b
                    b = temp
                    i = i + 1
                }
                return b
            }
        }
        fib(20)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(6765));
}

#[test]
fn test_sum_to_n_recursive() {
    let source = r#"
        def sum_to(n) {
            if (n <= 0) {
                return 0
            } else {
                return n + sum_to(n - 1)
            }
        }
        sum_to(10)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(55));
}

#[test]
fn test_power_function() {
    let source = r#"
        def power(base, exp) {
            if (exp == 0) {
                return 1
            } else {
                return base * power(base, exp - 1)
            }
        }
        power(2, 10)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(1024));
}

#[test]
fn test_mutual_recursion_even_odd() {
    let source = r#"
        def is_even(n) {
            if (n == 0) {
                return true
            } else {
                return is_odd(n - 1)
            }
        }
        def is_odd(n) {
            if (n == 0) {
                return false
            } else {
                return is_even(n - 1)
            }
        }
        is_even(10)
    "#;
    assert_eq!(run(source).unwrap(), Value::Bool(true));
}

// =============================================================================
// Complex Programs
// =============================================================================

#[test]
fn test_gcd_euclidean() {
    let source = r#"
        def gcd(a, b) {
            while (b != 0) {
                temp = b
                b = a % b
                a = temp
            }
            return a
        }
        gcd(48, 18)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(6));
}

#[test]
fn test_is_prime() {
    let source = r#"
        def is_prime(n) {
            if (n < 2) {
                return false
            } else {
                result = true
                i = 2
                while (i * i <= n) {
                    if (n % i == 0) {
                        result = false
                    } else {
                        result = result
                    }
                    i = i + 1
                }
                return result
            }
        }
        is_prime(17)
    "#;
    assert_eq!(run(source).unwrap(), Value::Bool(true));
}

#[test]
fn test_is_not_prime() {
    let source = r#"
        def is_prime(n) {
            if (n < 2) {
                return false
            } else {
                result = true
                i = 2
                while (i * i <= n) {
                    if (n % i == 0) {
                        result = false
                    } else {
                        result = result
                    }
                    i = i + 1
                }
                return result
            }
        }
        is_prime(15)
    "#;
    assert_eq!(run(source).unwrap(), Value::Bool(false));
}

#[test]
fn test_collatz_steps() {
    // Count steps to reach 1 in Collatz sequence
    let source = r#"
        def collatz_steps(n) {
            steps = 0
            while (n != 1) {
                if (n % 2 == 0) {
                    n = n / 2
                } else {
                    n = n * 3 + 1
                }
                steps = steps + 1
            }
            return steps
        }
        collatz_steps(27)
    "#;
    assert_eq!(run(source).unwrap(), Value::Int(111));
}

// =============================================================================
// Error Cases
// =============================================================================

#[test]
fn test_undefined_variable_error() {
    let result = run("x");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Undefined variable"));
}

#[test]
fn test_undefined_function_error() {
    let result = run("foo()");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Undefined"));
}

#[test]
fn test_division_by_zero_error() {
    let result = run("10 / 0");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Division by zero"));
}

#[test]
fn test_wrong_argument_count_error() {
    let source = r#"
        def add(a, b) {
            return a + b
        }
        add(1)
    "#;
    let result = run(source);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects"));
}

#[test]
fn test_type_error_in_conditional() {
    let source = r#"
        if (42) {
            1
        } else {
            0
        }
    "#;
    let result = run(source);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("boolean"));
}
