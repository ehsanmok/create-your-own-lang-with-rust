# Firstlang

A dynamically typed, interpreted programming language with Python-like syntax. Firstlang supports variables, functions, control flow, and recursion.

## Features

- Variables and assignments
- Functions with parameters
- Control flow (`if`/`else`, `while`)
- Recursion
- Boolean and integer types
- Comparison and arithmetic operators
- REPL for interactive exploration

## Requirements

- Rust stable 1.70+
- No external dependencies (pure interpreter)

## Running Examples

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

Demonstrates variables, functions, conditionals, and loops.

### All Examples

```bash
for file in examples/*.fl; do
    echo "Running $file..."
    cargo run -- "$file"
done
```

## REPL

Start the interactive REPL:

```bash
cargo run
```

Try these examples:

```python
>>> x = 10
>>> x + 5
15

>>> def double(n) { return n * 2 }
>>> double(21)
42

>>> def fib(n) {
...     if (n <= 1) {
...         return n
...     } else {
...         return fib(n - 1) + fib(n - 2)
...     }
... }
>>> fib(10)
55
```

## Running Tests

Run all unit and integration tests:

```bash
cargo test
```

Run specific test:

```bash
cargo test test_fibonacci_recursive
```

Run with verbose output:

```bash
cargo test -- --nocapture
```

## Project Structure

```
firstlang/
├── src/
│   ├── grammar.pest    # PEG grammar definition
│   ├── parser.rs       # Parser (pest-generated)
│   ├── ast.rs          # Abstract Syntax Tree
│   ├── interpreter.rs  # Tree-walking interpreter
│   └── main.rs         # CLI and REPL
├── examples/           # Example programs
│   ├── basics.fl
│   ├── fibonacci.fl
│   └── factorial.fl
└── tests/              # Integration tests
    └── integration_tests.rs
```

## Language Syntax

### Variables

```python
x = 42
name = x + 10
```

### Functions

```python
def add(a, b) {
    return a + b
}

result = add(10, 20)
```

### Conditionals

```python
if (x > 0) {
    result = 1
} else {
    result = -1
}
```

### Loops

```python
i = 0
while (i < 10) {
    i = i + 1
}
```

### Recursion

```python
def factorial(n) {
    if (n <= 1) {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}
```

## Comparison with Calculator

Firstlang extends the Calculator language with:

- Named variables and assignments
- Function definitions with parameters
- Control flow structures (if/else, while)
- Multiple statement blocks
- Proper operator precedence
- Boolean type and comparison operators

## Next Steps

For a typed version with LLVM compilation, see [Secondlang](../secondlang/).

