# Recursion

[Recursion](https://en.wikipedia.org/wiki/Recursion_(computer_science)) is when a function calls itself. At first, this seems like a paradox - how can a function call itself when it's still running? But with the call stack we built in the [functions](./functions.md) chapter, it works beautifully.

Think of recursion like Russian nesting dolls (matryoshka). Each doll contains a smaller version of itself, until you reach the smallest doll that contains nothing. To solve a problem recursively, you solve a smaller version of the same problem, until you reach a problem so small it's trivial to solve.

## The Key Insight

A recursive function has two essential parts:

1. **Base case** - A condition where we return directly, without recursion. This is our "smallest doll" - the problem we can solve without any more work.

2. **Recursive case** - We call ourselves with a "smaller" problem, then combine the result with our current work.

Every recursive function must have both. Without a base case, the recursion never stops - you keep opening dolls forever, eventually crashing (stack overflow). Without a recursive case, there's no recursion at all - just a regular function.

## Classic Example: Factorial

The [factorial function](https://en.wikipedia.org/wiki/Factorial) is the "hello world" of recursion. You've seen it in math:

$$ n! = n \times (n-1) \times (n-2) \times \cdots \times 1 $$

So \(5! = 5 \times 4 \times 3 \times 2 \times 1 = 120\).

Here's the beautiful insight: \(5! = 5 \times 4!\), and \(4! = 4 \times 3!\), and so on. We can define factorial in terms of itself:

$$
n! = \begin{cases}
1 & \text{if } n \leq 1 \\\\
n \times (n-1)! & \text{otherwise}
\end{cases}
$$

In Firstlang:

```
def factorial(n) {
    if (n <= 1) {
        return 1        # Base case: 0! = 1! = 1
    } else {
        return n * factorial(n - 1)  # Recursive case
    }
}

factorial(5)  # 5 * 4 * 3 * 2 * 1 = 120
```

The function calls itself with a smaller `n` until `n` reaches 1. Then the results bubble back up, multiplied together.

## The Call Stack in Action

Here's where the call stack from [Functions](./functions.md) becomes crucial. Each recursive call creates a new stack frame with its own `n`. When we call `factorial(4)`:

1. `factorial(4)` starts, pushing a frame with `n = 4`
2. It calls `factorial(3)`, pushing another frame with `n = 3`
3. This continues until `factorial(1)` hits the base case and returns `1`
4. Then frames pop off one by one, each multiplying its `n` by the returned value

The stack grows as we dive deeper, then shrinks as we return. Each frame has its own `n`, so `n` in `factorial(4)` is completely separate from `n` in `factorial(3)`. This is exactly why we built the call stack the way we did.

## Why Recursion Works in Firstlang

Our interpreter properly handles recursion because of three design decisions:

1. **Functions are stored globally** - When `factorial` runs, it can look up `factorial` by name and find itself. This lookup returns the function definition, which we can then call.

2. **Each call gets its own frame** - When we call `factorial(3)` from inside `factorial(4)`, we push a new frame. The `n` in the new frame is 3, completely independent of the `n = 4` in the outer frame.

3. **Return propagates values correctly** - When `factorial(1)` returns `1`, that value goes back to `factorial(2)`, which uses it in `2 * 1 = 2`, and so on up the stack.

If we had used a single global `n` variable instead of stack frames, recursion would fail miserably - each call would overwrite `n`.

## More Recursive Examples

### Sum of Numbers

Add up all numbers from 1 to n:

```
def sum_to(n) {
    if (n <= 0) {
        return 0        # Base: sum to 0 is 0
    } else {
        return n + sum_to(n - 1)
    }
}

sum_to(5)  # 5 + 4 + 3 + 2 + 1 + 0 = 15
```

The insight: `sum(5) = 5 + sum(4)`. Each call adds one number and delegates the rest.

### Power Function

Calculate `base^exp`:

```
def power(base, exp) {
    if (exp == 0) {
        return 1        # Base: x^0 = 1
    } else {
        return base * power(base, exp - 1)
    }
}

power(2, 10)  # 1024
```

The insight: `2^10 = 2 * 2^9`. We multiply by the base once and compute a smaller power.

### Mutual Recursion

Functions can call each other recursively. This is called [mutual recursion](https://en.wikipedia.org/wiki/Mutual_recursion):

```
def is_even(n) {
    if (n == 0) {
        return true     # 0 is even
    } else {
        return is_odd(n - 1)
    }
}

def is_odd(n) {
    if (n == 0) {
        return false    # 0 is not odd
    } else {
        return is_even(n - 1)
    }
}

is_even(4)  # true
is_odd(7)   # true
```

How does `is_even(4)` work? It asks "is 3 odd?" which asks "is 2 even?" which asks "is 1 odd?" which asks "is 0 even?" which returns `true`. The logic ping-pongs between the two functions, but each call reduces `n` by 1, eventually reaching 0.

## A Warning: Stack Overflow

Every recursive call uses memory for its stack frame. Very deep recursion exhausts available stack space:

```
factorial(100000)  # Too deep! Crashes with stack overflow.
```

The iterative version using a `while` loop (from [control flow](./control_flow.md)) doesn't have this problem - it uses constant memory regardless of how many iterations:

```
def factorial_iter(n) {
    result = 1
    while (n > 1) {
        result = result * n
        n = n - 1
    }
    return result
}
```

Some languages implement [tail call optimization](https://en.wikipedia.org/wiki/Tail_call) to make certain recursive functions use constant stack space. We won't implement that here, but it's a fascinating optimization.

## When to Use Recursion

Recursion is natural for problems with recursive structure:

- Trees (each subtree is a smaller tree)
- Mathematical sequences (Fibonacci, factorial)
- Divide-and-conquer algorithms (merge sort, quicksort)
- Parsing nested structures (JSON, HTML, our AST!)

For simple loops, iteration is usually clearer. But for inherently recursive problems, recursion can be elegant and expressive.

## The Ultimate Test: Fibonacci

Now we're ready for [Fibonacci](./fibonacci.md) - the culmination of everything we've built! It's more complex than factorial (two recursive calls!), but our interpreter handles it beautifully.
