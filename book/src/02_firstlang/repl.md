# Building the REPL

A REPL (Read-Eval-Print Loop) is an interactive environment for your language. It lets users type expressions and see results immediately.

## The Basic Loop

```rust
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

```rust
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
