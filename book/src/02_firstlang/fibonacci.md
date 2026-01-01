# Computing Fibonacci

This is it - the culmination of everything we've built! Let's compute the [Fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_number).

## The Fibonacci Sequence

The Fibonacci sequence starts with 0 and 1, and each subsequent number is the sum of the previous two:

```
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, ...
```

Mathematically:

$$
\text{fib}(n) = \begin{cases}
0 & \text{if } n = 0 \\\\
1 & \text{if } n = 1 \\\\
\text{fib}(n-1) + \text{fib}(n-2) & \text{if } n > 1
\end{cases}
$$

This sequence appears throughout nature - in flower petals, pine cones, and the spiral of shells. It is closely related to the [golden ratio](https://en.wikipedia.org/wiki/Golden_ratio).

## Recursive Implementation

The mathematical definition translates directly to code:

```
def fib(n) {
    if (n < 2) {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

fib(10)  # = 55
```

This is beautiful in its simplicity, but let's trace through what happens when we call `fib(4)`:

<p align="center">
</br>
    <a href><img alt="fib(4)" src="../img/fib4.svg"> </a>
</p>

Each call creates a new stack frame in our interpreter, and when the function returns, we pop the frame and continue with the result.

## Why This Works

Our interpreter properly handles recursion because:

1. **Call Stack** - Each function call pushes a new environment frame
2. **Local Variables** - Parameters `n` are local to each call
3. **Return Values** - We propagate return values up the call stack

## Iterative Alternative

For comparison, here's an iterative version (also works in Firstlang):

```
def fib_iter(n) {
    if (n < 2) {
        return n
    } else {
        a = 0
        b = 1
        i = 2
        while (i <= n) {
            temp = a + b
            a = b
            b = temp
            i = i + 1
        }
        return b
    }
}

fib_iter(10)  # = 55
```

The iterative version is more efficient ($O(n)$ vs $O(2^n)$ - see [Big O notation](https://en.wikipedia.org/wiki/Big_O_notation)), but the recursive version is more elegant and demonstrates the power of our language.

## Running It

Save to `examples/fibonacci.fl` and run:

```bash
cargo run -- examples/fibonacci.fl
55
```

Or in the REPL:

```
>>> def fib(n) {
...     if (n < 2) { return n } else { return fib(n - 1) + fib(n - 2) }
... }
>>> fib(10)
55
>>> fib(20)
6765
```

## Performance Note

Our [tree-walking interpreter](https://en.wikipedia.org/wiki/Interpreter_(computing)#Abstract_syntax_tree_interpreters) is not optimized. Computing `fib(35)` might take several seconds due to exponential complexity and interpretation overhead.

In the next part of the book ([Secondlang](../03_secondlang/intro.md)), we'll add types and compile to native code via LLVM - making `fib(35)` nearly instant!

## What We've Built

Congratulations! You've built a complete interpreted programming language that can:

- Parse Python-like syntax
- Handle variables and scoping
- Define and call functions
- Support recursion with proper stack frames
- Execute control flow (if/else, while)
- Compute Fibonacci recursively

The Firstlang interpreter is about **500 lines of Rust** - not bad for a real programming language!

## Next Steps

Ready for more? In [Part III: Secondlang](../03_secondlang/intro.md), we'll add:

- [Type annotations](../03_secondlang/annotations.md) (`def fib(n: int) -> int`)
- [Type inference](../03_secondlang/inference.md)
- [LLVM code generation](../03_secondlang/codegen.md)
- [JIT compilation](../03_secondlang/jit_fibonacci.md) to native code

The same Fibonacci function, but compiled to machine code!
