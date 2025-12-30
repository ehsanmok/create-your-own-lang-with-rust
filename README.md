# Create your own programming language with Rust

![Github Pages](https://github.com/ehsanmok/create-your-own-lang-with-rust/workflows/Github%20Pages/badge.svg?branch=master)
![CI](https://github.com/ehsanmok/create-your-own-lang-with-rust/workflows/CI/badge.svg?branch=master)

This repository contains the codes and the contents for [createlang.rs](https://createlang.rs)

## Project Structure

The book teaches programming language implementation through three progressively complex languages:

| Language | Type System | Execution | Rust Version |
|----------|-------------|-----------|--------------|
| **Calculator** | None | Interpreter/VM | stable 1.70+ |
| **Calculator** | None | JIT | nightly (LLVM) |
| **Firstlang** | Dynamic | Interpreter | stable 1.70+ |
| **Secondlang** | Static | LLVM JIT | nightly (LLVM) |

```
├── calculator/     # Simple arithmetic language
├── firstlang/      # Interpreted language with recursion
├── secondlang/     # Typed language with LLVM backend
└── book/           # mdbook source
```

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
```

## Why am I writing this book?

The book arises from my frustration of not finding modern, clear and concise teaching materials that are readily accessible to beginners like me who wants to learn a bit on how to create their own programming language.

> "If you don't know how *compilers* work, then you don't know how computers work" <sup>[1](http://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html?)</sup>

> "If you can't explain something in simple terms, you don't understand it" <sup>[2](https://skeptics.stackexchange.com/questions/8742/did-einstein-say-if-you-cant-explain-it-simply-you-dont-understand-it-well-en)</sup>

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
