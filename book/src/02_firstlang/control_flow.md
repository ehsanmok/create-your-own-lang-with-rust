# Control Flow: If/Else and While

So far, our programs execute every statement in order, from top to bottom. That's like following a recipe that says "do step 1, then step 2, then step 3" - no thinking required, just execute.

But real programs need to *think*. They need to make decisions: "if the user is logged in, show the dashboard; otherwise, show the login page." They need to repeat: "while there are items in the cart, add up their prices."

Control flow gives our language these abilities. Without it, we can only write straight-line code. With it, we can write programs that respond to their inputs.

## Conditionals: If/Else

The `if` expression evaluates a condition and chooses which code to run:

```
if (x > 0) {
    return 1
} else {
    return 0
}
```

This reads naturally: "if x is greater than 0, return 1; else, return 0."

### How It Works

The interpreter needs to:

1. **Evaluate the condition** - Compute `x > 0` to get `true` or `false`
2. **Choose a branch** - If true, execute the `then` branch; if false, execute the `else` branch
3. **Return the result** - Whatever the chosen branch produces

Here's the implementation:

```rust,ignore
Expr::If { cond, then_branch, else_branch } => {
    // Step 1: Evaluate the condition
    let cond_val = self.eval_expr(cond)?;

    // Step 2: The condition must be a boolean
    if let Value::Bool(b) = cond_val {
        // Step 3: Choose and execute the appropriate branch
        let branch = if b { then_branch } else { else_branch };
        for stmt in branch {
            self.execute_stmt(stmt)?;
        }
    } else {
        return Err("Condition must be a boolean".to_string());
    }
}
```

Notice we require the condition to be a boolean. In some languages, `if (0)` is valid (0 is "falsy"). In Firstlang, we're explicit: `if (x == 0)` is the way to check for zero.

### Examples

**Absolute value** - Returns the non-negative version of a number:

```
def abs(x) {
    if (x < 0) {
        return x * -1
    } else {
        return x
    }
}

abs(-5)     # 5
abs(3)      # 3
```

When `x = -5`, the condition `x < 0` is `true`, so we return `-5 * -1 = 5`.
When `x = 3`, the condition is `false`, so we return `3` as-is.

**Maximum of two values** - Returns the larger one:

```
def max(a, b) {
    if (a > b) {
        return a
    } else {
        return b
    }
}

max(10, 20)  # 20
max(20, 10)  # 20
```

The condition `a > b` determines which value to return. Simple, but powerful.

## Loops: While

A `while` loop repeats its body as long as a condition is true:

```
x = 0
while (x < 5) {
    x = x + 1
}
x           # 5
```

This reads as: "while x is less than 5, increment x." The loop runs 5 times: when x is 0, 1, 2, 3, and 4. When x becomes 5, the condition `x < 5` is false, and the loop stops.

### How It Works

The interpreter uses an actual loop - Rust's `loop` construct - and checks the condition at the start of each iteration:

```rust,ignore
Expr::While { cond, body } => {
    loop {
        // Step 1: Evaluate the condition
        let cond_val = self.eval_expr(cond)?;

        if let Value::Bool(b) = cond_val {
            // Step 2: If false, exit the loop
            if !b { break; }

            // Step 3: If true, execute the body
            for stmt in body {
                self.execute_stmt(stmt)?;
            }
            // Step 4: Go back to step 1
        } else {
            return Err("While condition must be a boolean".to_string());
        }
    }

    // Loops don't return a meaningful value
    Ok(Value::Unit)
}
```

The key insight: after executing the body, we go back to the top and check the condition again. This creates the repetition.

### Examples

**Sum of 1 to 10** - Classic loop example:

```
sum = 0
i = 1
while (i <= 10) {
    sum = sum + i
    i = i + 1
}
sum         # 55
```

Let's trace through the first few iterations:

| Iteration | `i` | `sum` before | `sum` after |
|-----------|-----|--------------|-------------|
| 1 | 1 | 0 | 1 |
| 2 | 2 | 1 | 3 |
| 3 | 3 | 3 | 6 |
| ... | ... | ... | ... |
| 10 | 10 | 45 | 55 |

After iteration 10, `i` becomes 11, the condition `i <= 10` is false, and the loop exits.

**Iterative factorial** - Computing 5! = 120:

```
def factorial(n) {
    result = 1
    while (n > 1) {
        result = result * n
        n = n - 1
    }
    return result
}

factorial(5)  # 120
```

This computes `1 * 5 * 4 * 3 * 2 = 120`. The loop counts down from `n` to `2`, multiplying as it goes.

Compare this to the recursive version we'll see later:

```
def factorial_recursive(n) {
    if (n <= 1) {
        return 1
    } else {
        return n * factorial_recursive(n - 1)
    }
}
```

Same result, different approach. Loops and recursion are often interchangeable.

## Control Flow in Functions

The real power comes from combining everything. Here's a more complex example:

```
def countdown(n) {
    while (n > 0) {
        n = n - 1
    }
    return n
}
```

The function takes `n`, decrements it until it hits 0, then returns 0.

**Nested conditionals** - When one condition isn't enough:

```
def classify(n) {
    if (n < 0) {
        return -1    # Negative
    } else {
        if (n == 0) {
            return 0    # Zero
        } else {
            return 1    # Positive
        }
    }
}
```

The outer `if` checks for negative numbers. If not negative, we go to the `else` branch, which contains another `if` to distinguish zero from positive.

## Return in Loops

`return` exits the function immediately, even from deep inside a loop. This is useful for "search" patterns:

```
def find_first_even(n) {
    i = 1
    while (i <= n) {
        if (i % 2 == 0) {
            return i    # Found one! Exit immediately
        }
        i = i + 1
    }
    return 0    # Checked all numbers, none were even (only reached if n < 2)
}

find_first_even(5)  # 2
```

When `i = 2`, the condition `i % 2 == 0` is true, and we return immediately. The loop doesn't continue to `i = 3, 4, 5`. This "early return" pattern is common and efficient.

## What Happens Under the Hood

Both `if` and `while` are about *changing the flow of execution*. Without them, we execute line by line. With them, we can:

- **Skip code** (the branch not taken in `if`)
- **Repeat code** (the body of `while`)
- **Exit early** (`return` from inside a loop)

In compiled languages, these become **branch instructions** - the CPU actually jumps to different locations in memory. We'll see this when we compile to LLVM IR in Secondlang, where `if` becomes `br` (branch) instructions.

<div class="checkpoint">

At this point, you should be able to:

- Run `if (true) { 1 } else { 2 }` and get `1`
- Run a `while` loop that counts to 10
- Combine if/else with functions

</div>

<div class="related-topics">
<strong>Related Topics</strong>

- [Recursion](./recursion.md) - Functions calling themselves
- [Fibonacci](./fibonacci.md) - Using all constructs together
- [Secondlang Control Flow](../03_secondlang/codegen.md) - Compiling conditionals to IR

</div>

With control flow in place, we're ready for the ultimate test: [recursion](./recursion.md) - functions that call themselves!
