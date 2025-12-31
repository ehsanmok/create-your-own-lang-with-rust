# Firstlang: Your First Real Language

In Part I, we built a [calculator](../01_calculator/calc_intro.md) that could evaluate arithmetic expressions. While useful for learning the basics, it was not really a *programming language* - you cannot define functions, store values in variables, or make decisions with conditionals.

In this chapter, we build **Firstlang**, a complete interpreted programming language.

## What Changes from Calculator?

### Grammar: From 18 Lines to 70 Lines

The Calculator grammar was minimal:

```text
Program = _{ SOI ~ Expr ~ EOF }
Expr = { UnaryExpr | BinaryExpr | Term }
Term = _{Int | "(" ~ Expr ~ ")" }
UnaryExpr = { Operator ~ Term }
BinaryExpr = { Term ~ (Operator ~ Term)+ }
Operator = { "+" | "-" }
Int = @{ Operator? ~ ASCII_DIGIT+ }
```

Firstlang adds *statements*, *identifiers*, *functions*, and *control flow*:

```text
// New: Statements instead of just expressions
Program = _{ SOI ~ Stmt* ~ EOI }
Stmt = { Function | Return | Assignment | Expr }

// New: Identifiers for variables and function names
Identifier = @{ !KEYWORD ~ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }

// New: Functions with parameters
Function = { "def" ~ Identifier ~ "(" ~ Params? ~ ")" ~ Block }

// New: Control flow
Conditional = { "if" ~ "(" ~ Expr ~ ")" ~ Block ~ "else" ~ Block }
WhileLoop = { "while" ~ "(" ~ Expr ~ ")" ~ Block }
```

### AST: From 2 Node Types to 8

Calculator had just `Int` and `BinaryExpr`. Firstlang needs:

| Node | Purpose |
|------|---------|
| `Int`, `Bool` | Literal values |
| `Var` | Variable reference |
| `Binary`, `Unary` | Operators |
| `Call` | Function call |
| `If`, `While` | Control flow |
| `Function` | Function definition |
| `Return`, `Assignment` | Statements |

### Execution: From Expression Evaluation to Statement Execution

Calculator evaluated a single expression and returned a value. Firstlang executes a sequence of statements, maintains variable bindings, and handles function calls with a call stack.

## Features

- **Variables and assignment** (`x = 42`)
- **Functions with parameters** (`def add(a, b) { return a + b }`)
- **Conditionals** (`if (condition) { ... } else { ... }`)
- **Loops** (`while (condition) { ... }`)
- **Recursion** - functions that call themselves

Our ultimate goal is to compute the Fibonacci sequence recursively:

```
def fib(n) {
    if (n < 2) {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

fib(10)  # Returns 55
```

## Why an Interpreter First?

Before we dive into compilation (which we do in [Secondlang](../03_secondlang/intro.md)), we build a simple **tree-walking interpreter**. This approach:

1. **Is simpler** - No need to generate machine code
2. **Provides immediate feedback** - Great for a REPL
3. **Teaches the fundamentals** - How programs actually execute

The concepts you learn here (scoping, call stacks, evaluation) apply directly to understanding how compiled languages work.

## What You Will Learn

1. **Grammar Design** - How to specify a Python-like syntax
2. **AST Design** - Representing programs as data structures
3. **Variable Scoping** - How variables are stored and looked up
4. **Function Calls** - Managing the call stack for recursion
5. **Control Flow** - Implementing conditionals and loops

## Project Structure

Navigate to the `firstlang` subdirectory in the repository:

```bash
cd create-your-own-lang-with-rust/firstlang
```

The structure is:

```
firstlang/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Library exports
│   ├── main.rs         # CLI and REPL
│   ├── grammar.pest    # PEG grammar (70 lines)
│   ├── parser.rs       # AST construction
│   ├── ast.rs          # AST definitions
│   └── interpreter.rs  # Tree-walking interpreter
└── examples/
    ├── fibonacci.fl    # Recursive fibonacci
    ├── factorial.fl    # Factorial examples
    └── basics.fl       # Basic examples
```

Compare to Calculator:

```
calculator/
├── src/
│   ├── grammar.pest         # Minimal grammar (18 lines)
│   ├── parser.rs            # Simpler parsing
│   ├── ast.rs               # Just Int and BinaryExpr
│   └── compiler/
│       ├── interpreter.rs   # Expression evaluator
│       ├── jit.rs           # LLVM JIT (explored later)
│       └── vm/              # Bytecode VM (explored later)
```

We focus on a single backend (the interpreter) to deeply understand the new concepts without distraction.

## Running Firstlang

You can run a Firstlang file:

```bash
cargo run -- examples/fibonacci.fl
# Output: 55
```

Or start the REPL:

```bash
cargo run
# Firstlang REPL v0.1.0
# >>> 1 + 2
# 3
# >>> def double(x) { return x * 2 }
# >>> double(21)
# 42
```

Let us dive in.
