> Materials in this book are distributed under the terms of [Creative Commons BY-NC-SA 4.0](../../LICENSE)
> <p align="center">
>    <a href><img alt="license" src="./img/by-nc-sa.png" width="250" height="100"> </a>
> </p>

This book assumes some basic knowledge of Rust language. Please take a look  at the official [Rust book](https://doc.rust-lang.org/book/).

The accompanying codes and materials for this book are available in [GitHub](https://github.com/ehsanmok/create-your-own-lang-with-rust). To follow along, clone the repository

```text
git clone https://github.com/ehsanmok/create-your-own-lang-with-rust
cd create-your-own-lang-with-rust
```

## Motivations and Goals

This book arises from my frustration of not finding modern, clear and concise teaching materials that are readily accessible to beginners like me who wants to learn a bit on how to create their own programming language.

The following are my guidelines

> "If you don't know how *compilers* work, then you don't know how computers work" <sup>[1]((http://steve-yegge.blogspot.com/2007/06/rich-programmer-food.html?))</sup>


> "If you can’t explain something in simple terms, you don’t understand it" <sup>[2](https://skeptics.stackexchange.com/questions/8742/did-einstein-say-if-you-cant-explain-it-simply-you-dont-understand-it-well-en)</sup>

<span style="font-family:Trebuchet MS"> Pedagogically, one of the most effect methods of teaching is co-creating interactively. Introducing the core aspects around the *simplest example* (here, our calculator language) helps a lot to build knowledge and confidence. For that, we will use mature technologies instead of spending tone of time on partially reinventing-the-wheel and bore the reader.</span>

Here is an outline of the contents in this book

* [Crash Course on Computing](./crash_course.md) which we briefly set up the definitions and foundations
* We create our first programming language `Calc` that supports simple integer addition and subtraction. The simplicity allows us to cover a lot of important topics concisely. We will use [pest.rs](https://pest.rs) to define our grammar and generate our `CalcParser`, explain what AST is and interpreting the AST means. Next, we will introduce JIT compilation and use [inkwell: Safe Rust wrapper around LLVM](https://github.com/TheDan64/inkwell) to JIT compile our `Calc` language. We will continue by creating a REPL for our `Calc` language and finally we will create a Virtual Machine and runtime environment and discuss its features.
* TODO: We will introduce `Jeslang` (a statically typed, JIT compiled language) and gradually work our way up from `Calc` language to create `Jeslang` together
* TODO: Object system and object oriented programming
* TODO: Functional language
* TENTATIVE: Module system and packaging
* TENTATIVE: Create a mini standard library
* TODO: Resources

## Donation

If you have found this book useful, please consider donating any amount to any of the organizations below

* [Child Foundation](https://www.childfoundation.org/page/donate)
* [Black Lives Matters](https://blacklivesmatter.com/)
* [Food Bank of Canada](https://www.foodbankscanada.ca/)
