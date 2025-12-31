# Control Flow: If/Else and While

Control flow lets our programs make decisions and repeat operations.

## Conditionals: If/Else

The `if` expression evaluates a condition and executes one of two branches:

```
if (x > 0) {
    return 1
} else {
    return 0
}
```

### Implementation

In our interpreter:

```rust
Expr::If { cond, then_branch, else_branch } => {
    let cond_val = self.eval_expr(cond)?;
    if let Value::Bool(b) = cond_val {
        let branch = if b { then_branch } else { else_branch };
        // Execute the chosen branch
        for stmt in branch {
            // ...
        }
    }
}
```

### Examples

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

```
def max(a, b) {
    if (a > b) {
        return a
    } else {
        return b
    }
}

max(10, 20)  # 20
```

## Loops: While

The `while` loop repeats while a condition is true:

```
x = 0
while (x < 5) {
    x = x + 1
}
x           # 5
```

### Implementation

```rust
Expr::While { cond, body } => {
    loop {
        let cond_val = self.eval_expr(cond)?;
        if let Value::Bool(b) = cond_val {
            if !b { break; }
            // Execute body
            for stmt in body {
                // ...
            }
        }
    }
    Ok(Value::Unit)
}
```

### Examples

```
# Sum 1 to 10
sum = 0
i = 1
while (i <= 10) {
    sum = sum + i
    i = i + 1
}
sum         # 55
```

```
# Iterative factorial
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

## Control Flow in Functions

Control flow becomes powerful when combined with functions:

```
def countdown(n) {
    while (n > 0) {
        n = n - 1
    }
    return n
}

def fizzbuzz_check(n) {
    if (n % 3 == 0) {
        return 1    # Divisible by 3
    } else {
        if (n % 5 == 0) {
            return 2    # Divisible by 5
        } else {
            return 0    # Neither
        }
    }
}
```

## Return in Loops

`return` exits the function immediately, even from inside a loop:

```
def find_first_even(n) {
    i = 1
    while (i <= n) {
        if (i % 2 == 0) {
            return i    # Exit immediately
        }
        i = i + 1
    }
    return 0    # Not found
}

find_first_even(5)  # 2
```

<div class="checkpoint">
<strong>Checkpoint</strong>

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

With control flow in place, we're ready for the ultimate test: [recursion](./recursion.md)!
