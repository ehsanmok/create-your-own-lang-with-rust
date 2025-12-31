## Addition

So far, we've interpreted the AST directly - walking the tree and computing as we go. That works, but it's slow. Every time we evaluate `1 + 2`, we have to:

1. Look at the node type (it's a BinaryExpr)
2. Recursively evaluate the left side
3. Recursively evaluate the right side
4. Check the operator (it's Plus)
5. Add the numbers

For `fib(35)`, which does millions of additions, all this checking adds up.

What if, instead, we generated native machine code? The CPU could just execute `ADD` instructions directly - no interpretation overhead. That's what **JIT (Just-In-Time) compilation** gives us.

We'll use **[LLVM](https://llvm.org/)** as our code generation backend. LLVM is a compiler infrastructure used by Rust, Swift, and many other languages. We write LLVM IR (an intermediate representation), and LLVM compiles it to native code for whatever CPU we're running on.

### Setup

The code is available in [`calculator/examples/llvm/src/main.rs`](https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/examples/llvm/src/main.rs).

First, make sure the `inkwell` feature flag matches your LLVM version. Check with `llvm-config --version`, then update:

```toml
inkwell = { version = "0.7.1", features = ["llvm20-1"] }  # For LLVM 20.x
```

This example requires nightly Rust. Navigate to the [`calculator/examples/llvm`](https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/examples/llvm/) sub-crate and run:

```bash
rustup run nightly cargo run
```

### Our Goal: A Simple Add Function

Let's start with the simplest possible example - a function that adds two numbers:

```
add(x: i32, y: i32) -> i32 { x + y }
```

In LLVM IR, this looks like:

```llvm
define i32 @add(i32 %x, i32 %y) {
entry:
  %result = add i32 %x, %y
  ret i32 %result
}
```

Our job is to construct this IR using inkwell (Rust bindings for LLVM), then JIT compile and execute it.

### Step 1: Create the Foundation

LLVM organizes code in a hierarchy: **Context** contains **Modules**, which contain **Functions**, which contain **Basic Blocks**. Think of it like a project structure: the context is your workspace, modules are files, functions are... functions.

```rust,ignore
{{#include ../../../calculator/examples/llvm/src/main.rs:first}}
```

Let's break this down:

- **`Context::create()`** - The top-level container. All LLVM objects belong to a context. It manages memory and ensures thread safety.
- **`context.create_module("addition")`** - Creates a module (like a compilation unit). Our `add` function will live here.
- **`context.i32_type()`** - Gets the 32-bit integer type. LLVM is explicitly typed - we need to declare that our function works with `i32`.

### Step 2: Define the Function Signature

Now we tell LLVM what our function looks like - its name, parameter types, and return type:

```rust,ignore

{{#include ../../../calculator/examples/llvm/src/main.rs:second}}
```

Walking through this:

- **`i32_type.fn_type(&[i32_type.into(), i32_type.into()], false)`** - Creates a function type: returns `i32`, takes two `i32` parameters. The `false` means it's not variadic (doesn't take variable arguments like `printf`).
- **`module.add_function("add", fn_type, None)`** - Adds a function called "add" with this signature to our module.
- **`context.append_basic_block(add_fn, "entry")`** - Creates a basic block named "entry". A basic block is a sequence of instructions with no branches in the middle - execution flows straight through.
- **`context.create_builder()`** - The builder is our "cursor" for adding instructions. We position it at a basic block, then build instructions there.
- **`builder.position_at_end(entry)`** - Point the builder at our entry block. New instructions will go here.

### Step 3: Build the Function Body

Now the fun part - generating the actual addition:

```rust,ignore
{{#include ../../../calculator/examples/llvm/src/main.rs:third}}
```

Here's what's happening:

- **`add_fn.get_nth_param(0)`** - Get the first parameter. LLVM functions have an array of parameters, indexed from 0.
- **`.unwrap().into_int_value()`** - Parameters come as generic "basic values." We know ours are integers, so we convert them.
- **`builder.build_int_add(x, y, "result")`** - The magic! This emits an `add` instruction. The `"result"` is just a name for the output (helps when reading IR).
- **`builder.build_return(Some(&sum))`** - Emit a `ret` instruction to return our sum.

That's it! We've built a complete LLVM function. But it's just IR in memory - we haven't run it yet.

### Step 4: JIT Compile and Execute

Time to turn our IR into machine code and run it:

```rust,ignore
{{#include ../../../calculator/examples/llvm/src/main.rs:fourth}}
```

This is where it gets real:

- **`module.create_jit_execution_engine(OptimizationLevel::None)`** - Creates a JIT compiler. LLVM takes our IR and compiles it to native x86/ARM code *right now*, in memory.
- **`execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("add")`** - Look up our compiled function. The type signature tells Rust how to call it.
- **`add.call(1, 2)`** - Call the native function! This jumps directly to machine code - no interpretation, no overhead.

**The result: `3`.**

### Why All This Complexity?

You might be thinking: "That's a lot of code just to add two numbers!" And you're right. For simple addition, the interpreter is simpler.

But consider what we just built:

1. **Native speed** - That `add` instruction is real x86 `ADD`. No interpretation overhead.
2. **Platform portable** - We wrote generic IR. LLVM handles x86, ARM, WebAssembly, whatever.
3. **Optimizable** - Set `OptimizationLevel::Aggressive` and LLVM will optimize our code for free.
4. **Foundation for more** - This same pattern scales to full languages. Rust uses LLVM. Swift uses LLVM. Now we do too.

In the [next section](./ast_traversal.md), we'll compile our full calculator AST to LLVM - not just a hardcoded function, but dynamic code generation from any input expression.

<div class="checkpoint">

At this point, you should be able to:

- Run the LLVM example with `rustup run nightly cargo run`
- See the JIT execute and return `3` (for `1 + 2`)
- Understand the context → module → function → basic block hierarchy

</div>

<div class="related-topics">
<strong>Related Topics</strong>

- [AST Traversal Patterns](./ast_traversal.md) - Compile full AST to LLVM
- [LLVM IR Basics](../03_secondlang/ir.md) - Understanding LLVM IR syntax
- [Secondlang Codegen](../03_secondlang/codegen.md) - Full compiler to LLVM

</div>
