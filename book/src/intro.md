<div class="license-notice">
<p><em>Materials in this book are distributed under the terms of <a href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/LICENSE">Creative Commons BY-NC-SA 4.0</a></em></p>
<img alt="license" src="./img/by-nc-sa.png">
</div>

This book assumes some basic knowledge of the Rust language. Please take a look at the official [Rust book](https://doc.rust-lang.org/book/).

The accompanying code and materials for this book are available on [GitHub](https://github.com/ehsanmok/create-your-own-lang-with-rust). To follow along, make sure you have

* [Rust toolchain installed](https://www.rust-lang.org/tools/install) (stable 1.70+)
* Cloned the repository

    ```bash
    git clone https://github.com/ehsanmok/create-your-own-lang-with-rust
    ```

    Then navigate to the project directory:

    ```bash
    cd create-your-own-lang-with-rust
    ```

**What works with stable Rust:**

* **Calculator** (interpreter and VM modes) - `cargo run`
* **Firstlang** (interpreter) - `cargo run`

**What requires nightly Rust + LLVM:**

* **Calculator JIT** - `rustup run nightly cargo run --features jit`
* **Secondlang** (compiled to native code) - `rustup run nightly cargo run`
* **Thirdlang** (classes and OOP) - `rustup run nightly cargo run`

To use LLVM features:

* Install nightly: `rustup toolchain install nightly`
* Install LLVM: macOS (`brew install llvm`), Debian/Ubuntu ([apt.llvm.org](https://apt.llvm.org/))
* Check your version: `llvm-config --version`
* Update `Cargo.toml` to match: LLVM 20.x uses `llvm20-1`, LLVM 19.x uses `llvm19-1`, LLVM 18.x uses `llvm18-1`

## Motivations and Goals

This book arises from my frustration of not finding modern, clear, and concise teaching materials that are readily accessible to beginners like me who want to learn how to create their own programming language.

The following are my guidelines:

> "If you don't know how *compilers* work, then you don't know how computers work" <sup>[1](http://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html?)</sup>

> "If you can't explain something in simple terms, you don't understand it" <sup>[2](https://skeptics.stackexchange.com/questions/8742/did-einstein-say-if-you-cant-explain-it-simply-you-dont-understand-it-well-en)</sup>

<span style="font-family:Trebuchet MS">Pedagogically, one of the most effective methods of teaching is co-creating interactively. Introducing the core aspects around the *simplest example* (here, our calculator language) helps a lot to build knowledge and confidence. For that, we will use mature technologies instead of spending tons of time partially reinventing the wheel and boring the reader.</span>

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

This minimal language lets us focus on the fundamentals without distraction: what is a grammar? How does pest generate a parser? What is an AST? We also explore *three different backends* (interpreter, bytecode VM, JIT) to show that the same AST can be executed in multiple ways.

### Part II: Firstlang

With the basics understood, we add the features that make a *real* programming language. The grammar grows to 70 lines, adding:

```text
// New: Statements instead of just expressions
Stmt = { Function | Return | Assignment | Expr }

// New: Functions with parameters
Function = { "def" ~ Identifier ~ "(" ~ Params? ~ ")" ~ Block }

// New: Control flow
Conditional = { "if" ~ "(" ~ Expr ~ ")" ~ Block ~ "else" ~ Block }
WhileLoop = { "while" ~ "(" ~ Expr ~ ")" ~ Block }
```

We focus on a single backend (tree-walking interpreter) to deeply understand scoping, call stacks, and recursion. The culminating example is computing Fibonacci recursively.

### Part III: Secondlang

We add *static types* and compile to native code. The grammar changes are minimal (just 7 more lines), but the compiler grows significantly:

```text
// New: Type annotations
Type = { IntType | BoolType }
TypedParam = { Identifier ~ ":" ~ Type }
ReturnType = { "->" ~ Type }

// Modified: Functions now have types
Function = { "def" ~ Identifier ~ "(" ~ TypedParams? ~ ")" ~ ReturnType? ~ Block }
```

This demonstrates a key insight: types are primarily a *semantic* addition, not a syntactic one. The grammar changes are small, but we need new compiler phases (type checking, type inference) and can now generate efficient native code via LLVM.

### Part IV: Thirdlang

Finally, we add *object-oriented programming* with classes, methods, and memory management. The grammar grows to 140 lines:

```text
// New: Class definitions
ClassDef = { "class" ~ Identifier ~ "{" ~ ClassBody ~ "}" }
FieldDef = { Identifier ~ ":" ~ Type }
MethodDef = { "def" ~ Identifier ~ "(" ~ SelfParam ~ ... ~ ")" ~ ... ~ Block }

// New: Object operations
NewExpr = { "new" ~ Identifier ~ "(" ~ Args? ~ ")" }
Delete = { "delete" ~ Expr }
```

This introduces heap allocation (`malloc`/`free`), struct types in LLVM, and the `self` parameter for methods. We see how OOP features map to lower-level constructs.

## Outline

* [Crash Course on Computing](./crash_course.md) where we briefly set up definitions and foundations
* [**Calculator**](./01_calculator/calc_intro.md): Our first language supporting simple integer addition and subtraction. We use [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) to define our grammar, [pest](https://pest.rs/) to generate the parser, and explore AST interpretation, JIT compilation with [inkwell](https://github.com/TheDan64/inkwell), and a bytecode VM
* [**Firstlang**](./02_firstlang/intro.md): An interpreted language with variables, functions, control flow, and recursion. We implement [Fibonacci](./02_firstlang/fibonacci.md) as the culminating example
* [**Secondlang**](./03_secondlang/intro.md): A statically typed language that compiles to native code via LLVM. We add [type annotations](./03_secondlang/annotations.md), [type inference](./03_secondlang/inference.md), AST [optimization passes](./03_secondlang/optimizations.md) with the visitor pattern, and [JIT compilation](./03_secondlang/jit_fibonacci.md)
* [**Thirdlang**](./04_thirdlang/intro.md): An object-oriented language with [classes](./04_thirdlang/classes_syntax.md), [methods](./04_thirdlang/methods.md), [constructors/destructors](./04_thirdlang/constructors.md), and [explicit memory management](./04_thirdlang/memory.md) via `new` and `delete`

## Donation

If you have found this book useful, please consider donating to any of the organizations below

* [Child Foundation](https://childfoundation.org/)
* [Black Lives Matter](https://blacklivesmatter.com/)
* [Food Bank of Canada](https://www.foodbankscanada.ca/)
