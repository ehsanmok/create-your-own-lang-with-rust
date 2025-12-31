# From Calculator to Real Language

You've built a working calculator. It parses expressions like `(1 + 2) * 3`, constructs an AST, and evaluates (or compiles) the result through three different backends: an interpreter, a bytecode VM, and an LLVM JIT compiler. That's a real achievement - you understand the core pipeline that every language uses.

But something is missing. Try this thought experiment: Can you write a program that computes the first 10 Fibonacci numbers? Can you store intermediate results? Can you abstract repeated patterns into functions?

No - because a calculator is not a *programming language* yet.

## The Realization

A calculator is like a sophisticated pocket calculator: it takes input, computes output, and forgets everything in between. There's no *memory* of previous computations, no way to *name* values for reuse, and no way to *decide* which computation to perform based on conditions.

Think about what you *can't* do:

```
x = 10          # No variables!
if (x > 5) { }  # No conditionals!
def add(a, b) { return a + b }  # No functions!
```

Each of these requires the language to maintain *state* - to remember things across multiple operations. That's the fundamental shift we're about to make.

## What's Missing?

| Feature | Calculator | Real Language |
|---------|------------|---------------|
| **State** | None - each expression is independent | Variables persist across statements |
| **Abstraction** | None - can't name computations | Functions let you reuse code |
| **Decisions** | None - always evaluates everything | Conditionals choose what to run |
| **Repetition** | None - runs once | Loops repeat until done |

## The Key Insight

> Calculator is a *function* - input goes in, output comes out, nothing persists. A real language is a *state machine* - it maintains memory (variables), can branch (conditionals), and can loop (while). The AST becomes a *program* to execute, not just an expression to evaluate.

## What Carries Forward

Everything you learned still applies:

- **Grammar** - We extend it with new rules (statements, functions, control flow)
- **Parser** - Same pattern: grammar rules → AST nodes
- **AST** - Same idea: tree structure representing code
- **Evaluation** - Same recursion, but now we track state

## What Changes

The transition from Calculator to Firstlang is not about rewriting everything - it's about *adding layers*.

| Aspect | Calculator | Firstlang | Growth |
|--------|------------|-----------|--------|
| Grammar | 18 lines | 70 lines | ~4x larger |
| AST nodes | 2 types | 8+ types | ~4x more cases |
| Execution | Single eval | Statement sequence | Iterative instead of one-shot |
| State | None | Variable environment | HashMap of name → value |
| Functions | None | Call stack | Stack frames for recursion |
| Keywords | None | 5 keywords | `def`, `if`, `else`, `while`, `return` |

Each of these changes builds on what you already know:

**Grammar grows** - But it's still pest rules, just more of them.

**AST nodes multiply** - But they're still Rust enums with the same recursive structure.

**Execution becomes stateful** - But it's still recursive tree traversal, now with a HashMap.

**Functions add call stacks** - But each frame is just another HashMap, pushed and popped like any stack.

The *concepts* are the same. The *scale* increases.

## The Goal

By the end of Part II, you'll compute Fibonacci recursively:

```
def fib(n) {
    if (n < 2) {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

fib(10)  # → 55
```

This 8-line program exercises *everything* a real language needs:

- **Variables** (`n`) - Named storage
- **Functions** (`fib`) - Abstraction and reuse
- **Parameters** (passing `n`) - Data flow
- **Conditionals** (`if`/`else`) - Decision making
- **Operators** (`<`, `-`) - Computation
- **Recursion** (`fib` calls `fib`) - Self-reference
- **Call stack** (tracks each frame) - Memory management

If you can implement this, you have a real programming language.

## The Journey Ahead

Each chapter in Part II builds one piece:

1. **Syntax** - Define the Python-like grammar
2. **Variables** - Add state with a HashMap
3. **Functions** - Implement the call stack
4. **Control Flow** - Add `if` and `while`
5. **Recursion** - Put it all together
6. **REPL** - Make it interactive
7. **Fibonacci** - The culmination

By the end, you'll have something you can actually *program* in. Not just evaluate expressions - write real algorithms.

## Ready?

Let's build a real language.
