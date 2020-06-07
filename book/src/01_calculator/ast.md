## Abstract Syntax Tree (AST)

AST comes into picture when we want to go from the string representation of our program like `"-1"` or `"1 + 2"` to something more manageable and easier to work with. Since our program is not a random string (we have a grammar), we can use the structure within `"-1"` and `"1 + 2"` expressions to our own advantage and come up with a *new representation* like a [tree](https://en.wikipedia.org/wiki/Tree_structure)

<p align="center">
  </br>
    <a href><img  alt="ast" src="../img/ast.svg"> </a>
</p>

One thing to note here is the *kind* of the nodes in our tree is not the same (we don't want to be the same actually) i.e. `+` node is a different from `1` node. In fact, `+` has an **Operator** type and `1` is an integer **Int** type


<p align="center">
  </br>
    <a href><img  alt="ast" src="../img/ast_typed.svg"> </a>
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

Referring back to our grammar, we actually have different kinds of *recursive* expressions;

* **unary** with grammar
  ```
  UnaryExpr = { Operator ~ Term }
  ```
* **binary**: with grammar
  ```
  BinaryExpr = { Term ~ (Operator ~ Term)* }
  ```

For example in `"-1 + (2 + 3)"`

<p align="center">
</br>
    <a href><img alt="compiler" src="../img/ast_recursive.svg"> </a>
</p>

So we need to include those structures in our AST to make it an actual [tree data structure](https://en.wikipedia.org/wiki/Binary_tree). Notice the *inherent recursive structure in our grammar* that translates into


```rust
{{#include ../../../calculator/src/ast.rs:operator}}

{{#include ../../../calculator/src/ast.rs:node}}
```
<span class="filename">Filename: calculator/src/ast.rs</span>

After defining our AST, we can use the `pest` generated `CalcParser::parse` to map the Rules of our `Calc` language string to the AST.

```rust,ignore

{{#include ../../../calculator/src/parser.rs:parse_source}}
```
Checkout [calculator/src/parser.rs](../../../calculator/src/parser.rs).


Note that `CalcParser::parse` takes care of the AST traversal and correctly linearizes it in `Vec<Node>` so we can easily feed it to other stages of compilation.


## Interpreter

CPU is the ultimate interpreter that is it executes opcodes as it goes. When going from our source code `&str` to AST `Node`, we changed the representation (*lowered* the representation). A basic interpreter (recursively) looks and each node of the AST (via any [tree traversal methods](https://en.wikipedia.org/wiki/Tree_traversal)) and simply **evaluates** it *recursively*

```rust,ignore
{{#include ../../../calculator/src/compiler/interpreter.rs:interpreter_eval}}
```

To sum up, we define a `Compile` trait

```rust,ignore
{{#include ../../../calculator/src/lib.rs:compile_trait}}
```

and implement our interpreter

```rust,ignore
{{#include ../../../calculator/src/compiler/interpreter.rs:interpreter}}
```
<span class="filename">Filename: calculator/src/compiler/interpreter.rs</span>
