# Building the REPL

A REPL (Read-Eval-Print Loop) is an interactive environment for your language. It lets users type expressions and see results immediately.

## The Basic Loop

```rust,ignore
fn repl() {
    let mut interpreter = Interpreter::new();

    loop {
        // Read
        print!(">>> ");
        let line = read_line();

        // Eval
        let program = parse(&line)?;
        let result = interpreter.run(&program)?;

        // Print
        println!("{}", result);

        // Loop (back to Read)
    }
}
```

## Persistent State

The key insight is that we use **one interpreter** across all inputs. This means:

- Variables defined in one line are available in the next
- Functions stay defined for the session

```
>>> x = 42
>>> y = x + 8
>>> y
50
>>> def double(n) { return n * 2 }
>>> double(y)
100
```

## Running the REPL

```bash
cargo run
```

```
Firstlang REPL v0.1.0
Type expressions to evaluate, or 'quit' to exit.

>>> 1 + 2
3
>>> def fib(n) { if (n < 2) { return n } else { return fib(n-1) + fib(n-2) } }
>>> fib(10)
55
>>> quit
Goodbye!
```

## Multi-line Input

The REPL supports multi-line input by detecting unclosed brackets. When you have unclosed `{`, `(`, or `[`, the REPL shows a continuation prompt (`...`) and waits for more input:

```
>>> def factorial(n) {
...     if (n <= 1) {
...         return 1
...     } else {
...         return n * factorial(n - 1)
...     }
... }
>>> factorial(5)
120
```

This is implemented by counting bracket depth:

```rust,ignore
/// Count unmatched opening brackets in a string
fn bracket_depth(s: &str) -> i32 {
    let mut depth = 0;
    let mut in_string = false;

    for c in s.chars() {
        if c == '"' { in_string = !in_string; }
        if !in_string {
            match c {
                '{' | '(' | '[' => depth += 1,
                '}' | ')' | ']' => depth -= 1,
                _ => {}
            }
        }
    }
    depth
}
```

When `bracket_depth` is positive, we keep reading lines until all brackets are closed.

## Error Handling

The REPL catches errors and keeps running:

```
>>> undefined_var
Runtime error: Undefined variable: undefined_var
>>> 1 / 0
Runtime error: Division by zero
>>> 1 + 2
3
```

The REPL is your best friend for experimenting with the language! Try computing [Fibonacci](./fibonacci.md) interactively.

## Running Examples

Firstlang comes with several example programs demonstrating different features.

### Fibonacci Sequence

```bash
cargo run -- examples/fibonacci.fl
```

Expected output: `55`

### Factorial

```bash
cargo run -- examples/factorial.fl
```

Expected output: `120`

### Basics

```bash
cargo run -- examples/basics.fl
```

This demonstrates variables, functions, conditionals, and loops.

### All Examples

```bash
for file in examples/*.fl; do
    echo "Running $file..."
    cargo run -- "$file"
done
```

## Running Tests

Firstlang has comprehensive integration tests covering:

- Variables and assignments
- Functions with parameters
- Recursion (Fibonacci, factorial)
- Control flow (if/else, while)
- Type errors
- Undefined variables and functions

Run all tests:

```bash
cargo test
```

Run a specific test:

```bash
cargo test test_fibonacci_recursive
```

Run tests with verbose output:

```bash
cargo test -- --nocapture
```

The tests are in `tests/integration_tests.rs` and serve as examples of what the language can do.
