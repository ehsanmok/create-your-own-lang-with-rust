# Variables and Assignment

Variables let us store values and refer to them by name. In Firstlang, assignment is simple:

```
x = 42
name = x + 8    # name = 50
```

## How Variables Work

### The AST

In our AST, assignment is represented as:

```rust
enum Stmt {
    Assignment { name: String, value: Expr },
    // ...
}
```

### Storage: The Environment

Variables are stored in a `HashMap` inside our interpreter:

```rust
struct Frame {
    locals: HashMap<String, Value>,
}
```

When we encounter an assignment like `x = 42`:

1. Evaluate the right-hand side expression
2. Store the result in the current frame's `locals`

### Variable Lookup

When we encounter a variable reference like `x`:

1. Look in the current frame's locals
2. If not found, look in globals (for functions)
3. If still not found, error: "Undefined variable"

## Scoping

Firstlang uses simple scoping rules:

```
x = 10          # global scope

def foo() {
    y = 20      # local to foo
    return x + y
}

foo()           # = 30 (can access global x)
```

Variables inside functions are local to that function. Global variables (defined outside functions) are visible everywhere.

## Example

```
# Variables and arithmetic
a = 5
b = 3
sum = a + b     # 8
diff = a - b    # 2
prod = a * b    # 15

# Reassignment
x = 1
x = x + 1
x = x * 2
x               # 4
```

<div class="checkpoint">
<strong>Checkpoint</strong>

At this point, you should be able to:

- Parse and evaluate `x = 42` and `y = x + 1`
- Look up variables and get their values
- Handle "undefined variable" errors gracefully

</div>

In the [next section](./functions.md), we'll see how functions use these building blocks.

<div class="related-topics">
<strong>Related Topics</strong>

- [Functions](./functions.md) - Parameters are just local variables
- [Scoping](./recursion.md) - How each function call gets its own variables
- [Secondlang Variables](../03_secondlang/annotations.md) - Adding types to variables

</div>
