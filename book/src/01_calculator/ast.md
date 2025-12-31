## Abstract Syntax Tree (AST)

> Think of source code as a *sentence* and the AST as its *diagram*. Just like "The cat sat on the mat" can be diagrammed into subject/verb/object, `1 + 2` can be diagrammed into left/operator/right. The AST is that diagram - it captures *structure*, not just text.

AST comes into the picture when we want to go from the string representation of our program like `"-1"` or `"1 + 2"` to something more manageable and easier to work with. Since our program is not a random string (that's what the grammar ensures), we can use the structure within the expressions `"-1"` and `"1 + 2"` to our advantage and come up with a *new representation* like a [tree](https://en.wikipedia.org/wiki/Tree_structure):

<p align="center">
  </br>
    <a href><img  alt="ast" src="../img/ast.svg"> </a>
</p>

One thing to note here is that the *kinds* of nodes in our tree are not the same—the `+` node is different from the `1` node. In fact, `+` has an **Operator** type and `1` is an integer **Int** type:

<p align="center">
  </br>
    <a href><img  alt="ast" src="../img/ast-typed.svg"> </a>
</p>

so we define our AST nodes as

```rust
pub enum Operator {
    Plus,
    Minus,
}

pub enum Node {
    Int(i32),
}
```

Referring back to our grammar, we actually have different kinds of *recursive* expressions:

* **unary** grammar

  ```
  UnaryExpr = { Operator ~ Term }
  ```

* **binary** grammar

  ```
  BinaryExpr = { Term ~ (Operator ~ Term)* }
  ```

So for example, the expression `"-1 + (2 + 3)"` has this recursive structure

<p align="center">
</br>
    <a href><img alt="compiler" src="../img/ast-recursive.svg"> </a>
</p>

To include those into our AST to make it an actual [tree data structure](https://en.wikipedia.org/wiki/Binary_tree),
we complete our AST definition as follows

```rust
{{#include ../../../calculator/src/ast.rs:operator}}

{{#include ../../../calculator/src/ast.rs:node}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/src/ast.rs">calculator/src/ast.rs</a>

Now, we can use the `pest` generated `CalcParser::parse` to map the Rules of our `Calc` language string to our AST.

```rust,ignore

{{#include ../../../calculator/src/parser.rs:parse_source}}
```

Checkout [calculator/src/parser.rs](https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/src/parser.rs).

Note that `CalcParser::parse` takes care of the AST traversal and correctly maps it to `Vec<Node>` for easier access
in later stages of compilation.

## Interpreter

The CPU is the *ultimate interpreter*—it executes opcodes as it goes. After we have changed the representation (a.k.a. *lowered*) of our source code `&str` to AST `Node`, a basic interpreter looks at each node of the AST (via any [tree traversal method](https://en.wikipedia.org/wiki/Tree_traversal)) and simply **evaluates** it *recursively*:

```rust,ignore
{{#include ../../../calculator/src/compiler/interpreter.rs:interpreter_eval}}
```

To sum up, we define a `Compile` trait that we will use throughout this chapter

```rust,ignore
{{#include ../../../calculator/src/lib.rs:compile_trait}}
```

and we can now implement our interpreter

```rust,ignore
{{#include ../../../calculator/src/compiler/interpreter.rs:interpreter}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/src/compiler/interpreter.rs">calculator/src/compiler/interpreter.rs</a>

and test

```rust,ignore
assert_eq!(Interpreter::from_source("1 + 2").unwrap(), 3);
```

Run such tests locally with

```bash
cargo test interpreter --tests
```

<div class="checkpoint">
<strong>Checkpoint</strong>

At this point, you should be able to:

* Parse `1 + 2` and build an AST with a `Binary` node
* Evaluate `1 + 2 - 3` and get `0`
* Handle nested expressions like `-1 + (2 + 3)`

</div>

<div class="related-topics">
<strong>Related Topics</strong>

* [Grammar and Parser](./grammar_lexer_parser.md) - How input becomes tokens
* [JIT Compilation](./jit_intro.md) - Compile AST to machine code
* [Virtual Machine](./vm.md) - Compile AST to bytecode
* [Firstlang AST](../02_firstlang/functions.md) - AST for a full language

</div>
