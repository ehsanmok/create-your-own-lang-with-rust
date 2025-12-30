# Secondlang

A statically typed programming language with Python-like syntax, featuring type inference and LLVM-based JIT compilation. Secondlang builds on Firstlang by adding a type system and compiling to native code.

## Features

- Static typing with type inference
- Type annotations for functions and variables
- LLVM IR code generation
- JIT (Just-In-Time) compilation
- AST optimizations (constant folding, algebraic simplification)
- All Firstlang features (variables, functions, control flow, recursion)

## Requirements

- Rust nightly (required by inkwell)
- LLVM 20.x (check with `llvm-config --version`)

### Installation

```bash
# Install nightly Rust
rustup toolchain install nightly

# Install LLVM 20
# macOS:
brew install llvm

# Debian/Ubuntu:
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 20
```

## Running Examples

All commands require nightly Rust:

### Fibonacci Sequence

```bash
rustup run nightly cargo run -- examples/fibonacci.sl
```

Expected output: `55`

### Factorial

```bash
rustup run nightly cargo run -- examples/factorial.sl
```

Expected output: `120`

### Type Inference Showcase

```bash
rustup run nightly cargo run -- examples/inference.sl
```

Demonstrates type inference with mixed types.

### Basics with Types

```bash
rustup run nightly cargo run -- examples/basics.sl
```

Shows typed variables, functions, and control flow.

### View Generated LLVM IR

```bash
rustup run nightly cargo run -- --ir examples/fibonacci.sl
```

This shows the LLVM intermediate representation before JIT compilation.

### All Examples

```bash
for file in examples/*.sl; do
    echo "Running $file..."
    rustup run nightly cargo run -- "$file"
done
```

## Running Tests

Run all tests:

```bash
rustup run nightly cargo test
```

Run specific test:

```bash
rustup run nightly cargo test test_jit_fibonacci
```

Run with verbose output:

```bash
rustup run nightly cargo test -- --nocapture
```

## Project Structure

```
secondlang/
├── src/
│   ├── grammar.pest    # PEG grammar with types
│   ├── parser.rs       # Parser
│   ├── ast.rs          # Typed AST
│   ├── types.rs        # Type system
│   ├── typeck.rs       # Type checker/inference
│   ├── visitor.rs      # AST visitors (optimizations)
│   ├── codegen.rs      # LLVM IR code generation
│   └── main.rs         # CLI
├── examples/           # Example programs
│   ├── basics.sl
│   ├── fibonacci.sl
│   ├── factorial.sl
│   └── inference.sl
└── tests/              # Integration tests
    └── integration_tests.rs
```

## Language Syntax

### Type Annotations

```python
# Variable with type
x: int = 42

# Function with parameter and return types
def add(a: int, b: int) -> int {
    return a + b
}
```

### Type Inference

```python
# Type is inferred from the right-hand side
x = 42              # inferred as int
y = x + 10          # inferred as int
flag = x > 0        # inferred as bool

# Function with inferred return type
def double(n: int) {
    return n * 2    # return type inferred as int
}
```

### Full Example

```python
def fibonacci(n: int) -> int {
    if (n <= 1) {
        return n
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2)
    }
}

result = fibonacci(10)
```

## Type System

Secondlang supports:

- **Primitive types**: `int`, `bool`
- **Function types**: Inferred from parameters and return values
- **Type inference**: Local type inference with unification
- **Type checking**: Static verification at compile time

### Type Inference Example

```python
# All types are inferred automatically:
def is_even(n: int) {
    return n % 2 == 0    # return type: bool (inferred)
}

x = 10                   # int (inferred)
result = is_even(x)      # bool (inferred)
```

## Compilation Pipeline

1. **Parse**: Source code → AST
2. **Type Check**: Infer and verify types
3. **Optimize**: Constant folding, algebraic simplification
4. **CodeGen**: AST → LLVM IR
5. **JIT**: LLVM IR → Native code
6. **Execute**: Run compiled code

## Optimizations

Secondlang performs AST-level optimizations:

### Constant Folding

```python
# Before: 1 + 2 * 3
# After:  7
```

### Algebraic Simplification

```python
# Before: x + 0
# After:  x

# Before: x * 1
# After:  x

# Before: x - x
# After:  0
```

## Comparison with Firstlang

Secondlang extends Firstlang with:

- **Static typing**: Type safety at compile time
- **Type inference**: Automatic type deduction
- **LLVM compilation**: Native code generation
- **JIT execution**: Fast runtime performance
- **AST optimizations**: Compile-time optimizations

## Performance

Secondlang compiles to native code via LLVM, making it significantly faster than Firstlang's interpreter for compute-intensive tasks like recursive Fibonacci.

## Debugging

### View Generated IR

```bash
rustup run nightly cargo run -- --ir examples/fibonacci.sl
```

### View AST with Types

Modify `main.rs` to print the typed AST after type checking.

### Enable LLVM Verification

The code generator performs LLVM module verification automatically, catching any malformed IR.

## Further Reading

- [LLVM Language Reference](https://llvm.org/docs/LangRef.html)
- [Inkwell Documentation](https://thedan64.github.io/inkwell/)
- [Type Inference Algorithms](https://en.wikipedia.org/wiki/Type_inference)

## Previous Step

For the interpreted version without types, see [Firstlang](../firstlang/).
