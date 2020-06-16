## AST Traversal Patterns

Recall from the previous section that JITing our [add function](./basic_llvm.md) was very detailed and cumbersome to write. Fortunately, there are some useful patterns for traversing complicated ASTs (and IRs)

* **Builder pattern**
* **Visitor pattern** (Will be introduced in chapter 4)

### Builder Pattern

Recall how we have interpreted our AST by traversing recursively and evaluating the nodes

```rust, no_run, noplaypen
{{#include ../../../calculator/src/compiler/interpreter.rs:interpreter_recursive}}
```
<span class="filename">Filename: calculator/src/compiler/interpreter.rs</span>

but instead, we can take advantage of the [inkwell Builder](https://thedan64.github.io/inkwell/inkwell/builder/struct.Builder.html) and recursively traverse our `Calc` AST as follows

```rust, no_run, noplaypen
{{#include ../../../calculator/src/compiler/jit.rs:jit_recursive_builder}}
```

and similar to our addition example, we can JIT the builder output

```rust, no_run, noplaypen
{{#include ../../../calculator/src/compiler/jit.rs:jit_ast}}
```
<span class="filename">Filename: calculator/src/compiler/jit.rs</span>

Finally, we can test it

```rust,ignore
assert_eq!(Jit::from_source("1 + 2").unwrap(), 3)
```

Run such tests locally with

```text
cargo test jit --tests
```
