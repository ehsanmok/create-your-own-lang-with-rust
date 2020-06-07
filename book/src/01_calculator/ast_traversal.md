## AST Traversal Patterns

Recall from the previous section that JITing our [add function](./basic_llvm.md) was very detailed and cumbersome to write. For traversing complicated ASTs, fortunately, there are some useful patterns

* **Builder pattern**
* **Visitor pattern** (Will introduce it in chapter 4)

### Builder Pattern

Recall how we have interpreted our AST by traversing recursively and evaluating the nodes

```rust, no_run, noplaypen
{{#include ../../../calculator/src/compiler/interpreter.rs:interpreter_recursive}}
```
<span class="filename">Filename: calculator/src/compiler/interpreter.rs</span>

but now, we can take advantage of the [inkwell Builder](https://thedan64.github.io/inkwell/inkwell/builder/struct.Builder.html) and recursively traverse our AST

```rust, no_run, noplaypen
{{#include ../../../calculator/src/compiler/jit.rs:jit_recursive_builder}}
```

and similar to our addition example, we can JIT our AST as follows

```rust, no_run, noplaypen
{{#include ../../../calculator/src/compiler/jit.rs:jit_ast}}
```
<span class="filename">Filename: calculator/src/compiler/jit.rs</span>
