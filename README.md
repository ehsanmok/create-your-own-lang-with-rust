# Create your own programming language with Rust

![Github Pages](https://github.com/ehsanmok/create-your-own-lang-with-rust/workflows/Github%20Pages/badge.svg?branch=master)
![CI](https://github.com/ehsanmok/create-your-own-lang-with-rust/workflows/CI/badge.svg?branch=master)

This repository contains the codes and the contents for [createlang.rs](https://createlang.rs)

## Why am I writing this book?

The book arises from my frustration of not finding modern, clear and concise teaching materials that are readily accessible to beginners like me who wants to learn a bit on how to create their own programming language.

The following are my guidelines

> "If you don't know how *compilers* work, then you don't know how computers work" <sup>[1](http://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html?)</sup>


> "If you can’t explain something in simple terms, you don’t understand it" <sup>[2](https://skeptics.stackexchange.com/questions/8742/did-einstein-say-if-you-cant-explain-it-simply-you-dont-understand-it-well-en)</sup>

## Requirements

Make sure you have

1. [Rust toolchain installed](https://www.rust-lang.org/tools/install)
2. Cloned this repository (follow the instructions in each chapter)
3. LLVM installed to run and test locally `cargo test --tests`
    * Easiest option is LLVM v14.0 ([Debian/Ubuntu](https://apt.llvm.org/) or [macOS](https://formulae.brew.sh/formula/llvm))
    * Otherwise, in `Cargo.toml` you'd need to change the `inkwell = { ..., branch = "master", features = ["your-llvm-version"] }` with LLVM version on your system (output of `llvm-config --version`)


To build the book locally, navigate to the `book` subdirectory and follow the instructions in [mdbook](https://github.com/rust-lang/mdBook).

## Roadmap

Checkout the [roadmap to the 1st edition](https://github.com/ehsanmok/create-your-own-lang-with-rust/projects).

## Donation

If you have found this project useful, please consider donating to any of the organizations below

* [Child Foundation](https://www.childfoundation.org/page/donate)
* [Black Lives Matter](https://blacklivesmatter.com/)
* [Food Bank of Canada](https://www.foodbankscanada.ca/)
