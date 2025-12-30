# Recursion

[Recursion](https://en.wikipedia.org/wiki/Recursion_(computer_science)) is when a function calls itself. It's a powerful technique for solving problems that have a recursive structure.

## The Key Insight

A recursive function has two parts:

1. **Base case** - A condition where we return directly (no recursion)
2. **Recursive case** - We call ourselves with a "smaller" problem

Every recursive function must have both. Without a base case, the recursion never stops (infinite loop). Without a recursive case, there is no recursion at all.

## Classic Example: Factorial

The [factorial function](https://en.wikipedia.org/wiki/Factorial) is the "hello world" of recursion. Mathematically:

$$ n! = n \times (n-1) \times (n-2) \times \cdots \times 1 $$

Or recursively:

$$
n! = \begin{cases}
1 & \text{if } n \leq 1 \\\\
n \times (n-1)! & \text{otherwise}
\end{cases}
$$

```python
def factorial(n) {
    if (n <= 1) {
        return 1        # Base case
    } else {
        return n * factorial(n - 1)  # Recursive case
    }
}

factorial(5)  # 5 * 4 * 3 * 2 * 1 = 120
```

## How It Works

Let's trace `factorial(4)`:

```
factorial(4)
  → 4 * factorial(3)
      → 3 * factorial(2)
          → 2 * factorial(1)
              → 1  (base case!)
          ← 2 * 1 = 2
      ← 3 * 2 = 6
  ← 4 * 6 = 24
```

## The Call Stack

Each recursive call creates a new [stack frame](https://en.wikipedia.org/wiki/Call_stack):

```
factorial(4) → frame: {n: 4}
factorial(3) → frame: {n: 3}
factorial(2) → frame: {n: 2}
factorial(1) → frame: {n: 1}  ← hits base case, starts returning
```

This is why our interpreter needs a call stack - to keep track of each call's local variables.

## Why Recursion Works in Firstlang

Our interpreter properly handles recursion because:

1. **Functions are stored globally** - `factorial` can look itself up
2. **Each call gets its own frame** - Local `n` is different in each call
3. **Return propagates values** - Results bubble up through the stack

## More Examples

### Sum of Numbers

```python
def sum_to(n) {
    if (n <= 0) {
        return 0
    } else {
        return n + sum_to(n - 1)
    }
}

sum_to(5)  # 5 + 4 + 3 + 2 + 1 + 0 = 15
```

### Power Function

```python
def power(base, exp) {
    if (exp == 0) {
        return 1
    } else {
        return base * power(base, exp - 1)
    }
}

power(2, 10)  # 1024
```

### Mutual Recursion

Functions can call each other recursively. This is called [mutual recursion](https://en.wikipedia.org/wiki/Mutual_recursion):

```python
def is_even(n) {
    if (n == 0) {
        return true
    } else {
        return is_odd(n - 1)
    }
}

def is_odd(n) {
    if (n == 0) {
        return false
    } else {
        return is_even(n - 1)
    }
}

is_even(4)  # true
is_odd(7)   # true
```

## A Note on Stack Overflow

Recursion uses memory for each call. Very deep recursion can cause a [stack overflow](https://en.wikipedia.org/wiki/Stack_overflow):

```python
factorial(100000)  # Too deep! Runs out of stack space.
```

The iterative version using a `while` loop (from the [control flow](./control_flow.md) chapter) does not have this problem. Some languages optimize [tail calls](https://en.wikipedia.org/wiki/Tail_call) to avoid this issue.

## The Ultimate Test: Fibonacci

Now we're ready for [Fibonacci](./fibonacci.md) - the culmination of everything we've built!
