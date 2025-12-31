# From Calculator to Real Language

You've built a working calculator. It parses expressions, constructs an AST, and evaluates (or compiles) the result. That's a real achievement - you understand the core pipeline that every language uses.

But it's not a *programming language* yet.

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

| Aspect | Calculator | Firstlang |
|--------|------------|-----------|
| Grammar | 18 lines | 70 lines |
| AST nodes | 2 types | 8+ types |
| Execution | Single eval | Statement sequence |
| State | None | Variable environment |
| Functions | None | Call stack |

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

This requires everything: variables, functions, conditionals, recursion.

## Ready?

Let's build a real language.
