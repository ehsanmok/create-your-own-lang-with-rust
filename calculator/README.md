# Calculator

A simple arithmetic expression language supporting addition and subtraction. This is the first language in the book, designed to introduce fundamental compiler concepts.

## Features

- Integer arithmetic (addition, subtraction)
- Unary operators (`-1`, `+2`)
- Parentheses for grouping
- Three execution backends:
  - **Interpreter**: Direct AST evaluation
  - **VM**: Bytecode compilation and stack-based VM
  - **JIT**: LLVM-based just-in-time compilation

## Requirements

### Interpreter and VM (stable Rust)

- Rust stable 1.70+
- No external dependencies

### JIT (nightly Rust + LLVM)

- Rust nightly
- LLVM 20.x (check with `llvm-config --version`)

## Running Examples

### Interpreter (stable Rust)

```bash
cargo run --bin main examples/simple.calc
```

### VM (stable Rust)

```bash
cargo run --bin main --features vm examples/simple.calc
```

### JIT (nightly Rust + LLVM)

```bash
rustup run nightly cargo run --bin main --features jit examples/simple.calc
```

## REPL

Start the interactive REPL with different backends:

### Interpreter REPL

```bash
cargo run --bin repl --features interpreter
```

### VM REPL

```bash
cargo run --bin repl --features vm
```

### JIT REPL

```bash
rustup run nightly cargo run --bin repl --features jit
```

### Example Session

```text
Calculator prompt. Expressions are line evaluated.
>> 1 + 2
3
>> -2 + 5
3
>> (10 - 3) + 5
12
>> -(5 - 2)
-3
>> CTRL-C
```

## Running Tests

```bash
# All tests (interpreter and VM only, stable Rust)
cargo test

# With specific features
cargo test --features interpreter
cargo test --features vm
```

## Project Structure

```
calculator/
├── src/
│   ├── grammar.pest        # PEG grammar
│   ├── parser.rs           # Parser
│   ├── ast.rs              # Abstract Syntax Tree
│   ├── lib.rs              # Library interface
│   ├── main.rs             # File execution CLI
│   ├── bin/
│   │   └── repl.rs         # REPL
│   └── compiler/
│       ├── interpreter.rs  # Direct AST interpreter
│       ├── jit.rs          # LLVM JIT compiler
│       └── vm/
│           ├── bytecode.rs # Bytecode compiler
│           ├── opcode.rs   # Bytecode operations
│           └── vm.rs       # Stack-based VM
└── examples/
    └── simple.calc         # Example expressions
```

## Grammar

The calculator grammar in PEG format:

```pest
Program = _{ SOI ~ Expr ~ EOF }

Expr = { BinaryExpr | UnaryExpr | Term }

Term = _{ Int | "(" ~ Expr ~ ")" }

UnaryExpr = { Operator ~ Term }

BinaryExpr = { (UnaryExpr | Term) ~ (Operator ~ Term)+ }

Operator = { "+" | "-" }

Int = @{ ASCII_DIGIT+ }

WHITESPACE = _{ " " | "\t" | "\r" | "\n" }

EOF = _{ EOI | ";" }
```

## Language Syntax

### Basic Arithmetic

```
1 + 2        # => 3
10 - 5       # => 5
```

### Unary Operators

```
-1           # => -1
+5           # => 5
-2 + 5       # => 3
```

### Parentheses

```
(1 + 2)      # => 3
(10 - 3) + 5 # => 12
-(5 - 2)     # => -3
```

### Multiple Operations

```
1 + 2 + 3    # => 6
10 - 5 - 2   # => 3
```

## Execution Backends

### Interpreter

Direct evaluation of the AST. Simple but slower.

- ✅ Fast compilation
- ❌ Slower execution
- ✅ Easy to understand
- ✅ Stable Rust

### VM (Virtual Machine)

Compiles to bytecode, executes on a stack-based VM.

- ✅ Compact bytecode
- ✅ Faster than interpreter
- ✅ Portable
- ✅ Stable Rust

### JIT (Just-In-Time)

Compiles to native machine code via LLVM.

## Bytecode VM

The VM uses a stack-based architecture with these opcodes:

- `OpConstant(index)`: Push constant onto stack
- `OpAdd`: Pop two values, push sum
- `OpSub`: Pop two values, push difference
- `OpPop`: Pop and discard top value

Example bytecode for `1 + 2`:

```
OpConstant(0)  # Push 1
OpConstant(1)  # Push 2
OpAdd          # Pop 2, 1, push 3
```

## LLVM JIT

The JIT compiler generates LLVM IR:

```llvm
define i32 @jit() {
entry:
  ret i32 3
}
```

Then uses LLVM's JIT engine to compile to native code.

## Performance Comparison

For `(1 + 2) + (3 + 4)`:

| Backend     | Compile Time | Execution Time |
|-------------|--------------|----------------|
| Interpreter | Fast         | Slow           |
| VM          | Medium       | Medium         |
| JIT         | Slow         | Fast           |

## Known Issues

- **Issue #13**: Parser treats linefeeds as whitespace ✅ Fixed
- **Issue #17**: Negative first number support ✅ Fixed

## Next Steps

For a full programming language with variables, functions, and control flow, see [Firstlang](../firstlang/).
