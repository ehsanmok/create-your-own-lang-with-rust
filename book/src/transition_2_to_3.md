# From Interpreted to Compiled

You've built Firstlang - a complete interpreted language with variables, functions, control flow, and recursion. You can compute Fibonacci recursively, implement factorial, and even write a greatest common divisor algorithm. It works, it's correct, and you understand exactly how it executes.

But there's a problem: **it's painfully slow**.

Try computing `fib(35)` in Firstlang. On a modern CPU, it takes several seconds - maybe 5-10 seconds depending on your machine. That's for a function that makes millions of recursive calls, but each call just does a few additions and comparisons.

Now try the same computation in a compiled language like C or Rust. It takes *milliseconds* - roughly 1000x faster.

## Why Is Interpretation So Slow?

Let's trace what happens when Firstlang evaluates `fib(n - 1)`:

1. **Parse** the subtraction expression from the AST
2. **Look up** `n` in the current environment (HashMap lookup)
3. **Pattern match** on the left operand type to get its value
4. **Pattern match** on the right operand type (`Int(1)`)
5. **Check** that both are integers (runtime type check)
6. **Perform** the subtraction
7. **Wrap** the result back into an `Int` enum
8. **Parse** the function call expression
9. **Look up** `fib` in the global environment
10. **Check** it's a function (runtime check)
11. **Create** a new stack frame (allocate HashMap)
12. **Bind** the parameter
13. **Recurse** into the function body

That's 13 steps *per operation*, and most involve HashMap lookups, pattern matching, and heap allocations. For `fib(35)`, which does roughly 30 million function calls, that's hundreds of millions of operations.

Compare to compiled code:

```assembly
sub $1, %rdi      ; n - 1 directly in register
call fib          ; Direct jump, no lookup
```

Two CPU instructions. No lookups, no checks, no allocations.

## Why Interpretation is Slow

> Imagine reading a recipe in French. An *interpreter* translates each word as you cook - slow, but you can start immediately. A *compiler* translates the whole recipe to English first, then you cook from the translation - upfront cost, but much faster execution.

The tree-walking interpreter:

1. Reads an AST node
2. Decides what to do (match on node type)
3. Recursively processes children
4. Returns the result

This "decide what to do" step happens *every time* the code runs. A compiled program decides once, at compile time.

## What Types Enable

Adding types isn't just about catching errors (though that's valuable). Types tell us *what operations to generate*.

```
x + y   # What instruction? Depends on types!

# If int + int → generate integer add
# If float + float → generate floating add
# If string + string → generate concatenation
```

Without types, we'd need runtime checks. With types, we generate the right code directly.

## The Compilation Pipeline

```
Source → Parse → TypeCheck → Codegen → LLVM IR → Machine Code
```

New stages:

- **TypeCheck** - Verify types match, infer unknowns
- **Codegen** - Generate LLVM IR from typed AST
- **LLVM** - Optimize and compile to native code

## What Carries Forward

| From Firstlang | In Secondlang |
|----------------|---------------|
| Grammar | Extended with type annotations |
| AST | Same structure + type fields |
| Semantics | Same behavior, now type-checked |

The *meaning* of programs stays the same. We're changing *how* they execute.

## What's New

| Concept | Purpose |
|---------|---------|
| Type annotations | `def add(a: int, b: int) -> int` |
| Type inference | `x = 5` → `x` is `int` |
| LLVM IR | Portable low-level code |
| JIT compilation | Native speed |

## The Payoff

The exact same Fibonacci algorithm, but with type annotations:

```
def fib(n: int) -> int {
    if (n < 2) { return n }
    return fib(n - 1) + fib(n - 2)
}

fib(35)  # Milliseconds instead of seconds!
```

Adding `int` types and `->` return types might seem like a small syntactic change - just a few extra characters. But they unlock an entirely different execution model.

**The numbers:**

| Implementation | `fib(35)` Time | Speedup |
|----------------|----------------|---------|
| Firstlang interpreter | ~8 seconds | 1x (baseline) |
| Secondlang JIT | ~8 milliseconds | ~1000x faster |

Same algorithm. Same logic. Different execution model.

## What You'll Learn

The journey from Firstlang to Secondlang teaches you how modern languages work:

1. **Type Systems** - How types catch errors at compile time
2. **Type Inference** - Deducing types automatically (like Rust, TypeScript)
3. **LLVM IR** - The intermediate representation powering Swift, Rust, Julia
4. **Code Generation** - Transforming high-level concepts to machine instructions
5. **Optimization** - Constant folding, algebraic simplification
6. **JIT Compilation** - Compiling and running code on the fly

These are the same techniques used in production compilers. After Secondlang, you'll understand how `rustc`, `swiftc`, and `clang` actually work under the hood.

## Ready?

Let's add types and compile to native code.
