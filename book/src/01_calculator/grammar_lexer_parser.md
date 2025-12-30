## Grammar-Lexer-Parser Pipeline

Here is a high-level view of a compiler *frontend* pipeline

<p align="center">
</br>
    <a href><img alt="grammar, lexer, parser" src="../img/grammar-lexer-parser.svg"> </a>
</p>

Every language needs a (formal) grammar to describe its syntax and semantics. Once a program adheres to the rules of the grammar in *Source Code* (for example, as an input string or file), it is *tokenized*. The *lexer* then adds metadata to each token—for example, where each token starts and finishes in the original source code. Lastly, the *parser* reshapes or restructures the lexed outputs into an [Abstract Syntax Tree](./ast.md).

## Grammar

While there are varieties of ways to define the grammar, in this book we will use the [Parsing Expression Grammar (PEG)](https://en.wikipedia.org/wiki/Parsing_expression_grammar).

Here is how the grammar for our simple calculator language `Calc` (supporting addition and subtraction) looks in PEG:

```text
{{ #include ../../../calculator/src/grammar.pest }}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/src/grammar.pest">calculator/src/grammar.pest</a>

This grammar basically defines the syntax and semantics where

* each `Program` consists of expressions (`Expr`)
* expressions are either unary (`-1`), binary (`1 + 2`), or a simple term
* binary expressions can start with a unary expression (e.g., `-1 + 2`)
* unary and binary expressions are made of `Term` and `Operator` (`"+"` and `"-"`)
* the only *atom* is integer `Int`
* `WHITESPACE` includes spaces, tabs, and newlines for flexible formatting

Given our grammar, we will use [pest](https://pest.rs/), which is a powerful *parser generator* for PEG grammars. (For more details on pest, check out the [pest book](https://pest.rs/book/).)

`pest` *derives* the parser `CalcParser::parse` from our grammar

```rust,ignore
{{#include ../../../calculator/src/parser.rs:parser}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/src/parser.rs">calculator/src/parser.rs</a>

and does all the steps of the frontend pipeline that we mentioned so that we can start parsing any `Calc` source code (`source: &str`) via the `Rule`s of our grammar

```rust,ignore
CalcParser::parse(Rule::Program, source)
```

Before doing that, we need to define our Abstract Syntax Tree (AST) in the [next section](./ast.md).
