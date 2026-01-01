# Testing Your Language

A language implementation has many moving parts. Testing each layer ensures correctness and catches regressions. This chapter covers practical testing strategies.

## The Testing Pyramid

Test at multiple levels:

- **Unit tests**: Individual functions (parser, type checker)
- **Integration tests**: Multiple components together
- **End-to-end tests**: Full programs, source to result

## Unit Testing the Parser

Test that source code produces the expected AST:

```rust,ignore
#[test]
fn test_parse_addition() {
    let ast = parse("1 + 2").unwrap();
    assert!(matches!(
        ast,
        Expr::Binary { op: BinaryOp::Add, .. }
    ));
}

#[test]
fn test_parse_error() {
    let result = parse("1 +");
    assert!(result.is_err());
}
```

**What to test:**

- Valid syntax parses correctly
- Invalid syntax produces errors
- Edge cases: empty input, deeply nested expressions
- Operator precedence: `1 + 2 * 3` parses as `1 + (2 * 3)`

## Unit Testing the Type Checker

Test type inference and error detection:

```rust,ignore
#[test]
fn test_type_inference() {
    let mut program = parse("x = 5").unwrap();
    typecheck(&mut program).unwrap();
    // x should be inferred as int
}

#[test]
fn test_type_error() {
    let mut program = parse("x = 5 + true").unwrap();
    let result = typecheck(&mut program);
    assert!(result.is_err());
}
```

**What to test:**

- Correct types are inferred
- Type mismatches are caught
- Function signatures match calls
- Class field access is type-safe

## Integration Testing

Test the full pipeline from source to result:

```rust,ignore
#[test]
fn test_fibonacci() {
    let source = r#"
        def fib(n: int) -> int {
            if (n < 2) { return n }
            return fib(n - 1) + fib(n - 2)
        }
        fib(10)
    "#;
    assert_eq!(run(source).unwrap(), 55);
}
```

**What to test:**

- Complete programs produce correct output
- Edge cases in language features
- Combinations of features work together

## Snapshot Testing

Compare output against saved "golden" files:

```rust,ignore
#[test]
fn test_ir_output() {
    let ir = compile_to_ir("def answer() -> int { return 42 }").unwrap();
    insta::assert_snapshot!(ir);
}
```

Use [insta](https://insta.rs/) for snapshot testing in Rust. When output changes, review and accept or reject.

**Good for:**

- IR output (changes are visible in diffs)
- Error messages
- Pretty-printed AST

## Property-Based Testing

Generate random inputs and check properties:

```rust,ignore
use quickcheck::quickcheck;

quickcheck! {
    fn parse_roundtrip(expr: ArbitraryExpr) -> bool {
        let source = expr.to_source();
        let parsed = parse(&source);
        parsed.is_ok()
    }
}
```

**Properties to test:**

- Parsing never panics on any input
- Type checking is deterministic
- Optimization preserves semantics

## Fuzzing

Automatically find inputs that crash or hang:

```bash
cargo install cargo-fuzz
cargo fuzz init
cargo fuzz run parse_fuzz
```

Create a fuzz target:

```rust,ignore
// fuzz/fuzz_targets/parse_fuzz.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = myparser::parse(s);  // Should never panic
    }
});
```

**Fuzzing finds:**

- Parser crashes on malformed input
- Infinite loops
- Stack overflows from deep recursion
- Memory issues

Run fuzzing for hours/days on CI. Even 10 minutes often finds bugs.

## Testing Error Messages

Errors should be helpful. Test them:

```rust,ignore
#[test]
fn test_undefined_variable_error() {
    let result = run("x + 1");
    let err = result.unwrap_err();
    assert!(err.contains("undefined"));
    assert!(err.contains("x"));
}
```

## Regression Tests

When you fix a bug, add a test:

```rust,ignore
#[test]
fn test_issue_42_nested_if() {
    // This used to crash due to incorrect phi node generation
    let source = r#"
        if (true) {
            if (true) { 1 } else { 2 }
        } else { 3 }
    "#;
    assert_eq!(run(source).unwrap(), 1);
}
```

## Test Organization

Structure your tests:

```
tests/
  integration_tests.rs    # Full pipeline tests
  parser_tests.rs         # Parser unit tests
  typeck_tests.rs         # Type checker tests
  codegen_tests.rs        # Code generation tests
  examples/               # Example programs that should work
    fibonacci.sl
    factorial.sl
```

## Continuous Integration

Run tests automatically on every commit:

```yaml
# .github/workflows/ci.yml
- name: Run tests
  run: cargo test --all

- name: Run examples
  run: |
    cargo run -- examples/fibonacci.sl
    cargo run -- examples/factorial.sl
```

## Quick Reference

| Layer | What to Test | Tools |
|-------|--------------|-------|
| Parser | Syntax, precedence, errors | `#[test]`, quickcheck |
| Type Checker | Inference, error detection | `#[test]` |
| Codegen | IR correctness | Snapshot tests |
| Full Pipeline | End-to-end behavior | Integration tests |
| Robustness | Crashes, hangs | Fuzzing |

## Summary

1. **Start with integration tests** - Verify full programs work
2. **Add unit tests** for complex logic - Parser, type checker
3. **Use snapshots** for IR and error messages
4. **Fuzz your parser** - Find edge cases automatically
5. **Add regression tests** for every bug fix

Testing takes effort but prevents countless hours of debugging.
