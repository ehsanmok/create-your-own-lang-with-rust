<div class="license-notice">
<p><em>Materials in this book are distributed under the terms of <a href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/LICENSE">Creative Commons BY-NC-SA 4.0</a></em></p>
<img alt="license" src="./img/by-nc-sa.png">
<img src="./img/logo_light_mode.png" alt="CreateLang.rs Logo" class="logo-img logo-light" width="320">
<img src="./img/logo_dark_mode.png" alt="CreateLang.rs Logo" class="logo-img logo-dark" width="320">
</div>

---

## Motivations and Goals

This book arises from my frustration of not finding modern, clear, and concise teaching materials that are readily accessible to beginners like me who want to learn how to create their own programming language.

> *"If you don't know how compilers work, then you don't know how computers work"* <sup>[1](http://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html)</sup>

> *"If you can't explain something in simple terms, you don't understand it"* <sup>[2](https://skeptics.stackexchange.com/questions/8742/did-einstein-say-if-you-cant-explain-it-simply-you-dont-understand-it-well-en)</sup>

<span style="font-family:Trebuchet MS">Pedagogically, one of the most effective methods of teaching is co-creating interactively. Introducing the core aspects around the *simplest example* (here, our calculator language) helps build knowledge and confidence. We use mature technologies instead of reinventing the wheel.</span>

---

## Getting Started

This book assumes basic knowledge of Rust. If you're new to Rust, start with the official [Rust book](https://doc.rust-lang.org/book/).

The code and materials are available on [GitHub](https://github.com/ehsanmok/create-your-own-lang-with-rust). To follow along:

```bash
git clone https://github.com/ehsanmok/create-your-own-lang-with-rust
cd create-your-own-lang-with-rust
```

### Calculator and Firstlang (stable Rust)

These projects work with stable Rust 1.70+ and require no external dependencies:

```bash
# Calculator - interpreter mode
cd calculator
cargo run --bin main examples/simple.calc

# Calculator - VM mode
cargo run --bin main --features vm examples/simple.calc

# Firstlang - interpreter
cd firstlang
cargo run -- examples/fibonacci.fl
cargo run  # REPL
```

### Secondlang and Thirdlang (nightly Rust + LLVM)

These projects require nightly Rust and LLVM for JIT compilation:

```bash
# Install nightly Rust
rustup toolchain install nightly

# Install LLVM (macOS)
brew install llvm

# Install LLVM (Debian/Ubuntu) - see https://apt.llvm.org/
```

Check your LLVM version with `llvm-config --version` and update the `inkwell` dependency in `Cargo.toml` to match:

| LLVM Version | inkwell feature |
|--------------|-----------------|
| 20.x | `llvm20-1` |
| 19.x | `llvm19-1` |
| 18.x | `llvm18-1` |

For example, with LLVM 20:

```toml
inkwell = { version = "0.7", features = ["llvm20-1"] }
```

```bash
# Secondlang
cd secondlang
rustup run nightly cargo run -- examples/fibonacci.sl
rustup run nightly cargo run -- --ir examples/fibonacci.sl  # view LLVM IR

# Thirdlang
cd thirdlang
rustup run nightly cargo run --bin thirdlang -- examples/point.tl
rustup run nightly cargo run --bin thirdlang -- examples/counter.tl
```

---

## Learning Progression

We build four languages, each building on concepts from the previous:

| Language | Grammar | New Concepts | Execution |
|----------|---------|--------------|-----------|
| **Calculator** | 18 lines | PEG basics, AST, operators | Interpreter, VM, JIT |
| **Firstlang** | 70 lines | Variables, functions, control flow, recursion | Tree-walking interpreter |
| **Secondlang** | 77 lines | Types, type inference, optimization passes | LLVM JIT compilation |
| **Thirdlang** | 140 lines | Classes, methods, constructors, memory management | LLVM JIT compilation |

### Part I: Calculator

We start with the *simplest possible language*: integer arithmetic with `+` and `-`. The grammar fits in 18 lines:

```text
Program = _{ SOI ~ Expr ~ EOF }
Expr = { UnaryExpr | BinaryExpr | Term }
Term = _{Int | "(" ~ Expr ~ ")" }
...
```

This minimal language lets us focus on the fundamentals: what is a grammar? How does pest generate a parser? What is an AST? We explore *three different backends* (interpreter, bytecode VM, JIT) to show that the same AST can be executed in multiple ways.

### Part II: Firstlang

With the basics understood, we add features that make a *real* programming language. The grammar grows to 70 lines:

```text
// Statements instead of just expressions
Stmt = { Function | Return | Assignment | Expr }

// Functions with parameters
Function = { "def" ~ Identifier ~ "(" ~ Params? ~ ")" ~ Block }

// Control flow
Conditional = { "if" ~ "(" ~ Expr ~ ")" ~ Block ~ "else" ~ Block }
WhileLoop = { "while" ~ "(" ~ Expr ~ ")" ~ Block }
```

We focus on a single backend (tree-walking interpreter) to deeply understand scoping, call stacks, and recursion. The culminating example is computing Fibonacci recursively.

### Part III: Secondlang

We add *static types* and compile to native code. The grammar changes are minimal (just 7 more lines), but the compiler grows significantly:

```text
// Type annotations
Type = { IntType | BoolType }
TypedParam = { Identifier ~ ":" ~ Type }
ReturnType = { "->" ~ Type }

// Functions now have types
Function = { "def" ~ Identifier ~ "(" ~ TypedParams? ~ ")" ~ ReturnType? ~ Block }
```

Types are primarily a *semantic* addition, not a syntactic one. The grammar changes are small, but we need new compiler phases (type checking, type inference) and can now generate efficient native code via LLVM.

### Part IV: Thirdlang

Finally, we add *object-oriented programming* with classes, methods, and memory management. The grammar grows to 140 lines:

```text
// Class definitions
ClassDef = { "class" ~ Identifier ~ "{" ~ ClassBody ~ "}" }
FieldDef = { Identifier ~ ":" ~ Type }
MethodDef = { "def" ~ Identifier ~ "(" ~ SelfParam ~ ... ~ ")" ~ ... ~ Block }

// Object operations
NewExpr = { "new" ~ Identifier ~ "(" ~ Args? ~ ")" }
Delete = { "delete" ~ Expr }
```

This introduces heap allocation (`malloc`/`free`), struct types in LLVM, and the `self` parameter for methods. We see how OOP features map to lower-level constructs.

---

## Outline

- [Crash Course on Computing](./crash_course.md) - definitions and foundations
- [**Calculator**](./01_calculator/calc_intro.md) - integer arithmetic with PEG, pest, AST interpretation, JIT compilation, and bytecode VM
- [**Firstlang**](./02_firstlang/intro.md) - variables, functions, control flow, recursion, culminating in [Fibonacci](./02_firstlang/fibonacci.md)
- [**Secondlang**](./03_secondlang/intro.md) - [type annotations](./03_secondlang/annotations.md), [type inference](./03_secondlang/inference.md), [optimization passes](./03_secondlang/optimizations.md), and [JIT compilation](./03_secondlang/jit_fibonacci.md)
- [**Thirdlang**](./04_thirdlang/intro.md) - [classes](./04_thirdlang/classes_syntax.md), [methods](./04_thirdlang/methods.md), [constructors/destructors](./04_thirdlang/constructors.md), and [memory management](./04_thirdlang/memory.md)

---

<p align="center">
  <strong>Support</strong><br>
  If you found this book useful, please consider donating to:
</p>

<p align="center">
  <a href="https://mycf.childfoundation.org/s/donate">Child Foundation</a> &bull;
  <a href="https://blacklivesmatter.com/">Black Lives Matter</a> &bull;
  <a href="https://www.foodbankscanada.ca/">Food Bank of Canada</a>
</p>
