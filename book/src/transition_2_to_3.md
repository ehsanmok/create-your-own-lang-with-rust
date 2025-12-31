# From Interpreted to Compiled

You've built Firstlang - a complete interpreted language with variables, functions, control flow, and recursion. You can compute Fibonacci!

But there's a problem: **it's slow**.

Try `fib(35)` in Firstlang. It takes seconds. A compiled version takes milliseconds.

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

Same Fibonacci, but now:

```
def fib(n: int) -> int {
    if (n < 2) { return n }
    return fib(n - 1) + fib(n - 2)
}

fib(35)  # Milliseconds, not seconds!
```

## Ready?

Let's add types and compile to native code.
