## Grammar-Lexer-Praser Pipeline

Every language needs a (formal) grammar to describe its syntax and semantics. Once a program adheres to the rules of the grammar in *Source Code* (for example as input string or file format), it is tokenized then lexer adds some metadata to each token, for example where each token starts and finishes in the original Source Code. Finally parsing (reshaping or restructuring) the lexer output into our [Abstract Syntax Tree (AST)](./ast.md) for further analysis.

The following is a compiler *frontend* pipeline

<p align="center">
</br>
    <a href><img alt="grammar, lexer, parser" src="../img/grammar_lexer_parser.svg"> </a>
</p>

## Grammar

While there are varieties of ways to define the grammar, in this book we will use the [Parsing Expression Grammar (PEG)](https://en.wikipedia.org/wiki/Parsing_expression_grammar).

Here is how our simple calculator language `calc` (supporting addition and subtraction) looks like in PEG

```text
{{ #include ../../../calculator/src/grammar.pest }}
```
<span class="filename">Filename: calculator/src/grammar.pest</span>

This grammar basically defines the syntax and semantics where

* each `Program` consists of expressions (`Expr`)
* expressions are either unary (`-1`) or binary (`1 + 2`)
* unary or binary expressions are made of `Term` and `Operator` (`"+"` and `"-"`)
* the only *atom* is integer `Int`

Given a PEG grammar, luckily we can use [pest.rs](https://pest.rs/) which is powerful *parser generator* for PEG grammars. (For more details on pest, checkout the [pest book](https://pest.rs/book/))

`pest` *generates* the parser `CalcParser::parse` from our grammar via

```rust,ignore
{{#include ../../../calculator/src/parser.rs:parser}}
```
<span class="filename">Filename: calculator/src/parser.rs</span>

and does all the steps in the frontend pipeline so that we can start parsing a `Calc` source code (`source: &str`) via the `Rule`s of our grammar

```rust,ignore
CalcParser::parse(Rule::Program, source)
```
