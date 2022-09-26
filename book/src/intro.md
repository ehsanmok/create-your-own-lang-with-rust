> Materials in this book are distributed under the terms of [Creative Commons BY-NC-SA 4.0](https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/LICENSE)
> <p align="center">
>    <a href><img alt="license" src="./img/by-nc-sa.png" width="250" height="100"> </a>
> </p>

This book assumes some basic knowledge of Rust language. Please take a look at the official [Rust book](https://doc.rust-lang.org/book/).

The accompanying codes and materials for this book are available in [GitHub](https://github.com/ehsanmok/create-your-own-lang-with-rust). To follow along, make sure you have

* [Rust toolchain installed](https://www.rust-lang.org/tools/install)
* Cloned the repository

    ```text
    git clone https://github.com/ehsanmok/create-your-own-lang-with-rust
    cd create-your-own-lang-with-rust
    ```

* LLVM installed to run and test locally `cargo test --tests`
  * Easiest option is LLVM v14.0 ([Debian/Ubuntu](https://apt.llvm.org/) or [macOS](https://formulae.brew.sh/formula/llvm))
  * Otherwise, in `Cargo.toml` you'd need to change the `inkwell = { ..., branch = "master", features = ["your-llvm-version"] }` with LLVM version on your system (output of `llvm-config --version`)

## Motivations and Goals

This book arises from my frustration of not finding modern, clear and concise teaching materials that are readily accessible to beginners like me who wants to learn a bit on how to create their own programming language.

The following are my guidelines

> "If you don't know how *compilers* work, then you don't know how computers work" <sup>[1](http://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html?)</sup>


> "If you can’t explain something in simple terms, you don’t understand it" <sup>[2](https://skeptics.stackexchange.com/questions/8742/did-einstein-say-if-you-cant-explain-it-simply-you-dont-understand-it-well-en)</sup>

<span style="font-family:Trebuchet MS"> Pedagogically, one of the most effective methods of teaching is co-creating interactively. Introducing the core aspects around the *simplest example* (here, our calculator language) helps a lot to build knowledge and confidence. For that, we will use mature technologies instead of spending tone of time on partially reinventing-the-wheel and bore the reader.</span>

Here is the outline of the contents

* [Crash Course on Computing](./crash_course.md) which we briefly set up the definitions and foundations
* We create our first programming language `Calc` that supports simple integer addition and subtraction. The simplicity allows us to touch a lot of important topics. We will use [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) to define our grammar, [pest](https://bitbegin.github.io/pest-rs/) to generate our `CalcParser` and explain what AST is and interpreting the AST means. Next, we will introduce JIT compilation and use [inkwell](https://github.com/TheDan64/inkwell) to JIT compile our `Calc` language from its AST. To show an alternative compilation approach, we will create a Virtual Machine and a Runtime environment and discuss its features. Finally, we will write a simple REPL for our `Calc` language and test out different execution paths.
* TODO: We will create `Firstlang`, a statically typed language, by gradually working our way up from our `Calc`
* TODO: Object system and minimal object oriented programming support
* TENTATIVE: Create a mini standard library
* TODO: Resources

## Donation

If you have found this book useful, please consider donating to any of the organizations below

* [Child Foundation](https://www.childfoundation.org/page/donate)
* [Black Lives Matter](https://blacklivesmatter.com/)
* [Food Bank of Canada](https://www.foodbankscanada.ca/)
