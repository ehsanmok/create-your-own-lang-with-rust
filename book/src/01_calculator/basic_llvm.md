##  Addition


### Setup

The code is available in [`calculator/examples/llvm/src/main.rs`](https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/examples/llvm/src/main.rs). Because my `llvm-config --version` shows `14.0.6` so I'm using `features = ["llvm14-0"]` in inkwell

```text
inkwell = { git = "https://github.com/TheDan64/inkwell", branch = "master", features = ["llvm14-0"] }
```

Go to [`calculator/examples/llvm`](https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/examples/llvm/) sub-crate and `cargo run`.


### Add Function

We want to define an add function like

```
add(x: i32, y: i32) -> i32 { x + y }
```

but using the **LLVM language** and JIT it. For that, we need to define *every* bit of what makes up a function through LLVM basic constructs such as context, module, function signature setups, argument types, basic block, etc.

Here is how to *stitch* our add function in LLVM

1. We start by creating a `context`, adding the `addition` module and setting up the data type we want to use `i32_type` of type [`IntType`](https://thedan64.github.io/inkwell/inkwell/types/struct.IntType.html)

```rust,ignore
{{#include ../../../calculator/examples/llvm/src/main.rs:first}}
```

2. We define the signature of `add(i32, i32) -> i32`, add the function to our module, create a [basic block](https://thedan64.github.io/inkwell/inkwell/basic_block/index.html) entry point and a builder to add later parts

```rust,ignore

{{#include ../../../calculator/examples/llvm/src/main.rs:second}}
```

3. We create the arguments `x` and `y` and add them to the `builder` to make up the return instruction

```rust,ignore
{{#include ../../../calculator/examples/llvm/src/main.rs:third}}
```

4. Finally, we create a JIT execution engine (with no optimization for now) and let LLVM handle rest of the work for us

```rust,ignore
{{#include ../../../calculator/examples/llvm/src/main.rs:fourth}}
```

Yes! all of this just to add two integers.
