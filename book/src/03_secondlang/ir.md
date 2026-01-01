# From AST to IR

Now we have a typed, [optimized](./optimizations.md) AST. The next step is to convert it to **[LLVM IR](https://en.wikipedia.org/wiki/LLVM#Intermediate_representation)** (Intermediate Representation). But before we write the [code generator](./codegen.md), let us understand what LLVM IR looks like.

## What is LLVM IR?

[LLVM](https://llvm.org/) is a compiler infrastructure project that provides a set of reusable compiler and toolchain technologies. The key idea: you compile your language to LLVM IR, and LLVM handles everything else - optimization, code generation for different platforms, etc.

> LLVM is like a universal translator for CPUs. You speak LLVM IR, and LLVM translates it to x86, ARM, WebAssembly - whatever you need. You write your compiler once; LLVM gives you every platform for free.

As discussed in the [Crash Course](../crash_course.md#intermediate-representation-ir), an [IR](https://en.wikipedia.org/wiki/Intermediate_representation) is any representation between source and assembly:

<p align="center">
</br>
    <a href><img alt="simple llvm flow" src="../img/simple-llvm-flow.svg"> </a>
</p>

Think of IR as a *universal assembly language*. It is low-level (close to the machine) but not tied to any specific CPU. LLVM takes IR and produces optimized machine code for whatever platform you are on.

Many languages use LLVM: Rust, Swift, Julia, Kotlin/Native, and more. By using LLVM, we get world-class optimizations for free.

## A Simple Example

Let us see what LLVM IR looks like. Here is a Secondlang function:

```rust,ignore
def answer() -> int {
    return 42
}
```

And here is the LLVM IR it compiles to:

```
define i64 @answer() {
entry:
  ret i64 42
}
```

Let us break this down piece by piece:

- `define i64 @answer()` - We are defining a function called `answer` that returns a 64-bit integer (`i64`) and takes no parameters
- `entry:` - This is a **label** marking the start of a **[basic block](https://en.wikipedia.org/wiki/Basic_block)** (a sequence of instructions with no branches in the middle)
- `ret i64 42` - Return the 64-bit integer value 42

That is it. Our function just returns 42.

## IR for Arithmetic

Here is a function that adds two numbers:

```rust,ignore
def add(a: int, b: int) -> int {
    return a + b
}
```

LLVM IR:

```
define i64 @add(i64 %a, i64 %b) {
entry:
  %a.addr = alloca i64           ; allocate stack space for a
  store i64 %a, ptr %a.addr      ; store parameter a
  %b.addr = alloca i64           ; allocate stack space for b
  store i64 %b, ptr %b.addr      ; store parameter b
  %0 = load i64, ptr %a.addr     ; load a
  %1 = load i64, ptr %b.addr     ; load b
  %add = add i64 %0, %1          ; add them
  ret i64 %add                   ; return result
}
```

This looks more complex. Let us understand it:

1. **Parameters** (`%a`, `%b`) come in as values
2. **Allocate stack space** with `alloca` - we create local variables `%a.addr` and `%b.addr`
3. **Store parameters** into the stack slots with `store`
4. **Load values** back from stack with `load`
5. **Add** the loaded values with `add i64`
6. **Return** the result

### Why All the Loading and Storing?

This pattern looks wasteful. Why not just use `%a` and `%b` directly?

The answer is **mutability**. In LLVM IR, values like `%a` are immutable - you cannot change them. But in most languages, variables *can* change:

```rust,ignore
x = 5
x = x + 1    // x is now 6
```

By storing variables in stack slots (`alloca`), we can modify them:

```
%x.addr = alloca i64
store i64 5, ptr %x.addr           ; x = 5
%tmp = load i64, ptr %x.addr       ; load x
%tmp2 = add i64 %tmp, 1            ; x + 1
store i64 %tmp2, ptr %x.addr       ; x = x + 1
```

This is called the **alloca/load/store pattern**. It is simple to generate and LLVM optimizes it away (the [mem2reg pass](https://llvm.org/docs/Passes.html#mem2reg-promote-memory-to-register) promotes stack slots to registers) when possible.

## IR for Conditionals

Conditionals require **branching** - jumping to different code based on a condition:

```rust,ignore
def max(a: int, b: int) -> int {
    if (a > b) {
        return a
    } else {
        return b
    }
}
```

LLVM IR:

```
define i64 @max(i64 %a, i64 %b) {
entry:
  %cmp = icmp sgt i64 %a, %b     ; compare: is a > b? (signed greater than)
  br i1 %cmp, label %then, label %else  ; branch based on result

then:
  ret i64 %a                     ; return a

else:
  ret i64 %b                     ; return b
}
```

Key instructions:

- `icmp sgt` - Integer compare, signed greater than. Returns an `i1` (1-bit integer, a boolean)
- `br i1 %cmp, label %then, label %else` - [Branch](https://en.wikipedia.org/wiki/Branch_(computer_science)): if `%cmp` is true, go to `then`, else go to `else`

Notice we have multiple **basic blocks** now: `entry`, `then`, and `else`. Each block ends with a **terminator** (like `ret` or `br`) that says where to go next.

## SSA Form

LLVM IR uses **[Static Single Assignment (SSA)](https://en.wikipedia.org/wiki/Static_single-assignment_form)** form. This means every variable is assigned exactly once.

Consider this code:

```rust,ignore
x = 1
x = 2
x = 3
```

In SSA form, we cannot reuse `x`. Instead, we create new names:

```
%x.1 = ...   ; first assignment
%x.2 = ...   ; second assignment
%x.3 = ...   ; third assignment
```

Each name appears on the left side of exactly one assignment.

Why SSA? It makes optimization easier. The compiler always knows exactly where each value was defined. This enables powerful optimizations like [dead code elimination](https://en.wikipedia.org/wiki/Dead_code_elimination), [constant propagation](https://en.wikipedia.org/wiki/Constant_folding#Constant_propagation), and [common subexpression elimination](https://en.wikipedia.org/wiki/Common_subexpression_elimination).

## Phi Nodes: Merging Values from Different Paths

SSA creates a problem with conditionals. Consider:

```rust,ignore
def pick(cond: bool, a: int, b: int) -> int {
    if (cond) {
        x = a
    } else {
        x = b
    }
    return x
}
```

After the if/else, what is `x`? It depends on which branch we took. In SSA, we need different names:

```
then:
  %x.then = ...
  br label %merge

else:
  %x.else = ...
  br label %merge

merge:
  ; what goes here? We need x, but is it %x.then or %x.else?
```

The answer is a **[phi node](https://en.wikipedia.org/wiki/Static_single-assignment_form#Converting_to_SSA)** (φ). A phi node selects a value based on which block we came from:

```
merge:
  %x = phi i64 [ %x.then, %then ], [ %x.else, %else ]
  ret i64 %x
```

This reads as: "If we came from `%then`, use `%x.then`. If we came from `%else`, use `%x.else`."

Phi nodes are the only way to merge values from different [control flow](https://en.wikipedia.org/wiki/Control_flow) paths in SSA.

## IR for Recursion

[Recursive](https://en.wikipedia.org/wiki/Recursion_(computer_science)) functions use the `call` instruction:

```rust,ignore
def fib(n: int) -> int {
    if (n < 2) {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}
```

LLVM IR (simplified):

```
define i64 @fib(i64 %n) {
entry:
  %cmp = icmp slt i64 %n, 2      ; is n < 2?
  br i1 %cmp, label %then, label %else

then:
  ret i64 %n                     ; return n

else:
  %n1 = sub i64 %n, 1            ; n - 1
  %fib1 = call i64 @fib(i64 %n1) ; fib(n - 1)
  %n2 = sub i64 %n, 2            ; n - 2
  %fib2 = call i64 @fib(i64 %n2) ; fib(n - 2)
  %result = add i64 %fib1, %fib2 ; fib(n-1) + fib(n-2)
  ret i64 %result
}
```

The `call` instruction calls a function and returns its result. Recursion is just calling the same function from within itself.

## Type Mapping

Our types map to LLVM types:

| Secondlang | LLVM IR | Notes |
|------------|---------|-------|
| `int` | `i64` | 64-bit signed integer |
| `bool` | `i1` or `i64` | 1-bit for branches, 64-bit for storage |

We use `i64` for booleans in most places (simpler), and truncate to `i1` only when needed for branches.

LLVM supports many integer sizes: `i1`, `i8`, `i16`, `i32`, `i64`, `i128`, etc. The number is the bit width.

## Viewing Generated IR

You can see the IR your programs compile to:

```bash
rustup run nightly cargo run -- --ir examples/fibonacci.sl
```

Output:

```
; ModuleID = 'secondlang'
source_filename = "secondlang"

define i64 @fib(i64 %n) {
entry:
  ; ... the generated IR
}

define i64 @__main() {
entry:
  %call = call i64 @fib(i64 10)
  ret i64 %call
}
```

Notice the `@__main` function. This is a wrapper we generate for top-level expressions. When you write `fib(10)` at the top level, we wrap it in `__main` so the JIT has something to call.

## Further Reading

- [LLVM Language Reference](https://llvm.org/docs/LangRef.html) - the official specification
- [SSA on Wikipedia](https://en.wikipedia.org/wiki/Static_single-assignment_form) - the theory behind SSA
- [LLVM Tutorial: Kaleidoscope](https://llvm.org/docs/tutorial/) - LLVM's own tutorial (in C++)
- [inkwell](https://github.com/TheDan64/inkwell) - the Rust bindings we use

## Summary

LLVM IR key concepts:

- **Low-level** - close to assembly, explicit about memory operations
- **Typed** - every value has a type (`i64`, `i1`, `ptr`, etc.)
- **SSA form** - each variable is assigned exactly once
- **Basic blocks** - sequences of instructions ending with a terminator (`ret`, `br`)
- **Phi nodes** - merge values from different control flow paths
- **Platform-independent** - LLVM handles the CPU-specific details

<div class="checkpoint">

At this point, you should understand:

- Why LLVM IR exists (portable, optimizable)
- The alloca/load/store pattern for variables
- How conditionals use basic blocks and phi nodes
- What SSA form means

</div>

<div class="related-topics">
<strong>Related Topics</strong>

- [LLVM Code Generation](./codegen.md) - Generating IR from AST
- [Basic LLVM Example](../01_calculator/basic_llvm.md) - Simple JIT introduction
- [Thirdlang IR](../04_thirdlang/optimization.md) - Optimizing IR

</div>

In the next chapter, we write the [code generator](./codegen.md) that produces this IR from our typed AST.
