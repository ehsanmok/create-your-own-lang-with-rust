# Secondlang

In Part II, we built [Firstlang](../02_firstlang/intro.md), an interpreted language with functions, recursion, and control flow. Now we take the next step: adding a *type system* and compiling to native code using LLVM.

## What Changes from Firstlang?

The transition from Firstlang to Secondlang illustrates a key insight in language design: **types are primarily a semantic addition, not a syntactic one**. The grammar changes are minimal, but the compiler architecture changes significantly.

### Grammar: 7 Lines Added

Firstlang's function definition:

```text
Function = { "def" ~ Identifier ~ "(" ~ Params? ~ ")" ~ Block }
Params = _{ Identifier ~ ("," ~ Identifier)* }
```

Secondlang's function definition:

```text
Function = { "def" ~ Identifier ~ "(" ~ TypedParams? ~ ")" ~ ReturnType? ~ Block }
TypedParams = _{ TypedParam ~ ("," ~ TypedParam)* }
TypedParam = { Identifier ~ ":" ~ Type }
ReturnType = { "->" ~ Type }
Type = { IntType | BoolType }
IntType = { "int" }
BoolType = { "bool" }
```

The only syntactic change is `param` becomes `param: type` and we add `-> return_type`. Everything else (expressions, statements, control flow) remains identical.

### Compiler: Two New Phases

| Phase | Firstlang | Secondlang |
|-------|-----------|------------|
| Parsing | Source → AST | Source → Typed AST |
| Type Checking | None | AST → Typed AST (types resolved) |
| Execution | Tree-walking interpreter | LLVM code generation → JIT |

The type checker is the major new component. It walks the AST, infers types for expressions, and catches errors like `1 + true` at compile time instead of runtime.

## Why Types Enable Compilation

Without types, the interpreter must check types at runtime for every operation:

```
# Firstlang interpreter
def eval_binary(left, right, op):
    if type(left) != type(right):
        raise TypeError("...")
    if op == "+" and isinstance(left, int):
        return left + right
    # ... many more checks
```

With static types, we know at compile time that `a` is an `int` and `b` is an `int`, so `a + b` is a single CPU instruction:

```
%add = add i64 %a, %b
```

No runtime checks, no type dispatch, just a direct machine instruction.

## Feature Comparison

| Feature | Firstlang | Secondlang |
|---------|-----------|------------|
| Type System | Dynamic (runtime) | Static (compile-time) |
| Type Annotations | None | `x: int`, `-> int` |
| Type Errors | Runtime crash | Compile-time error |
| Execution | Tree-walking interpreter | LLVM JIT native code |
| Optimizations | None | Constant folding, algebraic simplification |

## Syntax Comparison

Firstlang (untyped):

```
def fib(n) {
    if (n < 2) {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}
fib(10)
```

Secondlang (typed):

```rust,ignore
def fib(n: int) -> int {
    if (n < 2) {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}
fib(10)
```

The programs are nearly identical. Type annotations are the only difference, but they unlock compile-time safety and native code generation.

## Project Structure

```
secondlang/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Library exports
│   ├── main.rs          # CLI entry point
│   ├── grammar.pest     # PEG grammar with types
│   ├── parser.rs        # Parser → Typed AST
│   ├── ast.rs           # Typed AST definitions
│   ├── types.rs         # Type system
│   ├── typeck.rs        # Type checker and inference
│   ├── visitor.rs       # AST visitors and optimizations
│   └── codegen.rs       # LLVM code generation
├── examples/
│   ├── fibonacci.sl
│   └── factorial.sl
└── tests/
    └── integration_tests.rs
```

Compare to [Firstlang's structure](../02_firstlang/intro.md#project-structure):

```
firstlang/
├── src/
│   ├── grammar.pest     # Same structure, fewer rules
│   ├── parser.rs        # Simpler: no type handling
│   ├── ast.rs           # Simpler: no TypedExpr
│   └── interpreter.rs   # Tree-walking, no codegen
```

The new modules (`types.rs`, `typeck.rs`, `visitor.rs`, `codegen.rs`) represent the additional complexity that types bring, but also the power they unlock.

## Prerequisites

LLVM is required. Check your version with `llvm-config --version` and update `Cargo.toml` accordingly:

* LLVM 20.x: `features = ["llvm20-1"]`
* LLVM 19.x: `features = ["llvm19-1"]`
* LLVM 18.x: `features = ["llvm18-1"]`

Secondlang also requires Rust nightly due to inkwell's dependency on edition 2024.

```bash
rustup toolchain install nightly
```

## Quick Start

```bash
cd secondlang

# Run Fibonacci
rustup run nightly cargo run -- examples/fibonacci.sl

# Show LLVM IR
rustup run nightly cargo run -- --ir examples/fibonacci.sl

# Type check only
rustup run nightly cargo run -- --check examples/fibonacci.sl
```

## Outline

In the following chapters, we build Secondlang step by step:

1. [Why Types Matter](why_types.md) - Benefits of static typing
2. [Type Annotations](annotations.md) - Grammar and parsing changes
3. [Type Inference](inference.md) - Deducing types automatically
4. [AST Optimizations](optimizations.md) - Visitor pattern and optimization passes
5. [From AST to IR](ir.md) - LLVM intermediate representation
6. [LLVM Code Generation](codegen.md) - Native code generation
7. [JIT Compiling Fibonacci](jit_fibonacci.md) - Putting it all together
