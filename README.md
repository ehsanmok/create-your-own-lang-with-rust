# Create your own programming language with Rust

[![CI](https://github.com/ehsanmok/create-your-own-lang-with-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/ehsanmok/create-your-own-lang-with-rust/actions/workflows/ci.yml)
[![Github Pages](https://github.com/ehsanmok/create-your-own-lang-with-rust/actions/workflows/gh-pages.yml/badge.svg)](https://github.com/ehsanmok/create-your-own-lang-with-rust/actions/workflows/gh-pages.yml)

This repository contains the codes and the contents for [createlang.rs](https://createlang.rs)

## Why did I write this book?

The book arose from my frustration of not finding modern, clear and concise teaching materials that are readily accessible to beginners like me who wants to learn a bit on how to create their own programming language.

> "If you don't know how *compilers* work, then you don't know how computers work" <sup>[1](http://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html?)</sup>

> "If you can't explain something in simple terms, you don't understand it" <sup>[2](https://skeptics.stackexchange.com/questions/8742/did-einstein-say-if-you-cant-explain-it-simply-you-dont-understand-it-well-en)</sup>

## Project Structure

The book teaches programming language implementation through four progressively complex languages:

| Language | Type System | Execution | Features | Rust Version |
|----------|-------------|-----------|----------|--------------|
| **Calculator** | None | Interpreter/VM | Arithmetic | stable 1.70+ |
| **Calculator** | None | JIT | Arithmetic | nightly (LLVM) |
| **Firstlang** | Dynamic | Interpreter | Functions, recursion | stable 1.70+ |
| **Secondlang** | Static | LLVM JIT | Type inference | nightly (LLVM) |
| **Thirdlang** | Static | LLVM JIT | Classes, OOP | nightly (LLVM) |

```
├── calculator/     # Simple arithmetic language (see calculator/README.md)
├── firstlang/      # Interpreted language with recursion (see firstlang/README.md)
├── secondlang/     # Typed language with LLVM backend (see secondlang/README.md)
├── thirdlang/      # OOP language with classes (see thirdlang/README.md)
└── book/           # mdbook source
```

Each language has its own README with detailed instructions for running examples and tests.

## Quick Start

### Firstlang (stable Rust)

```bash
cd firstlang
cargo run -- examples/fibonacci.fl
cargo run  # REPL
```

### Secondlang (nightly Rust + LLVM)

```bash
cd secondlang
rustup run nightly cargo run -- examples/fibonacci.sl
rustup run nightly cargo run -- --ir examples/fibonacci.sl
```

### Thirdlang (nightly Rust + LLVM)

```bash
cd thirdlang
rustup run nightly cargo run --bin thirdlang -- examples/point.tl
rustup run nightly cargo run --bin thirdlang -- examples/counter.tl
```

### Calculator (stable Rust, or nightly for JIT)

```bash
cd calculator

# Interpreter (stable Rust)
cargo run --bin main examples/simple.calc

# VM (stable Rust)
cargo run --bin main --features vm examples/simple.calc

# JIT (nightly Rust + LLVM)
rustup run nightly cargo run --bin main --features jit examples/simple.calc
```

## Requirements

* [Rust toolchain](https://www.rust-lang.org/tools/install) (stable 1.70+ for Firstlang)
* Clone this repository

### LLVM (for Calculator JIT and Secondlang)

LLVM is required for JIT compilation. Calculator JIT and Secondlang require **nightly Rust** due to inkwell's dependency on edition 2024.

Install LLVM:

* macOS: `brew install llvm`
* Debian/Ubuntu: [apt.llvm.org](https://apt.llvm.org/)

Install nightly Rust:

```bash
rustup toolchain install nightly
```

Check your LLVM version with `llvm-config --version` and update the `inkwell` feature in `Cargo.toml` accordingly:

* LLVM 20.x: `features = ["llvm20-1"]`
* LLVM 19.x: `features = ["llvm19-1"]`
* LLVM 18.x: `features = ["llvm18-1"]`

### Firstlang Only (stable Rust)

Firstlang is a pure interpreter and works with **stable Rust 1.70+**. No LLVM required.

```bash
cd firstlang
cargo test
cargo run
```

## Running Tests

```bash
# Calculator (stable Rust - interpreter/VM only)
cd calculator && cargo test

# Firstlang (stable Rust)
cd firstlang && cargo test

# Secondlang (nightly Rust)
cd secondlang && rustup run nightly cargo test

# Thirdlang (nightly Rust)
cd thirdlang && rustup run nightly cargo test
```

## Building the Book

```bash
cd book
mdbook serve    # Local preview at http://localhost:3000
mdbook build    # Build static site
```

## Donation

If you have found this project useful, please consider donating to any of the organizations below

* [Child Foundation](https://mycf.childfoundation.org/s/donate)
* [Black Lives Matter](https://blacklivesmatter.com/)
* [Food Bank of Canada](https://www.foodbankscanada.ca/)
