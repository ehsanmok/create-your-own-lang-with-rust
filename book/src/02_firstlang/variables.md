# Variables and Assignment

Variables let us store values and refer to them by name. Without variables, we could only work with literal values - every computation would need to repeat its inputs. Variables give us *memory*.

```
x = 42
name = x + 8    # name = 50
```

That second line is the key: we're using `x` by name instead of repeating `42`. If we later change `x`, code that uses `x` automatically uses the new value. Variables make code reusable and readable.

## How Variables Work

Let's trace through what happens when our interpreter runs `x = 42; y = x + 1`:

### Step 1: Parse to AST

The parser turns this into:

```rust,ignore
[
    Stmt::Assignment { name: "x", value: Expr::Int(42) },
    Stmt::Assignment { name: "y", value: Expr::Binary {
        op: Add,
        left: Expr::Var("x"),
        right: Expr::Int(1)
    }},
]
```

Notice `Expr::Var("x")` - that's a reference to a variable, not the variable itself. We'll need to look it up later.

### Step 2: Execute First Assignment

For `x = 42`:

1. **Evaluate the right side** - `Expr::Int(42)` evaluates to `Value::Int(42)`
2. **Store in the environment** - Put `"x" -> 42` in our storage

### Step 3: Execute Second Assignment

For `y = x + 1`:

1. **Evaluate the right side** - This is a binary expression, so:
   - Evaluate left: `Expr::Var("x")` → look up "x" in storage → find `42`
   - Evaluate right: `Expr::Int(1)` → `1`
   - Apply operator: `42 + 1 = 43`
2. **Store in the environment** - Put `"y" -> 43` in our storage

### The Environment (Storage)

Where do variables live? In a `HashMap` inside a "frame":

```rust,ignore
struct Frame {
    locals: HashMap<String, Value>,
}
```

A `HashMap` is a dictionary: given a key (the variable name), it returns a value. When we assign `x = 42`, we insert `("x", 42)` into the map. When we look up `x`, we query the map with `"x"` and get `42` back.

Why call it a "frame"? Because in the [functions](./functions.md) chapter, we'll have multiple frames - one for each function call. Each function gets its own private storage.

## Variable Lookup

When we encounter a variable like `x`, we need to find its value:

```rust,ignore
fn lookup_var(&self, name: &str) -> Result<Value, String> {
    // First, check the current frame
    if let Some(value) = self.current_frame().locals.get(name) {
        return Ok(value.clone());
    }

    // Then, check globals (for functions defined at top level)
    if let Some(value) = self.globals.get(name) {
        return Ok(value.clone());
    }

    // Not found anywhere - error!
    Err(format!("Undefined variable: {}", name))
}
```

This is called **scoping**: we first look in the local scope (current function), then fall back to global scope. If the variable isn't anywhere, that's an error.

## Scoping: Local vs Global

Variables can be *local* (inside a function) or *global* (outside all functions):

```
x = 10          # global - accessible everywhere

def foo() {
    y = 20      # local to foo - only accessible inside foo
    return x + y  # can access global x
}

foo()           # = 30
# y             # ERROR: y is not defined here
```

When `foo` runs, it has access to:

- Its own locals (`y`)
- Global variables (`x`)

But code outside `foo` cannot see `y` - it only existed during the function call.

## Reassignment

Variables can be reassigned:

```
x = 1
x = x + 1   # x is now 2
x = x * 2   # x is now 4
```

The right side is always evaluated *first*, using the current value of `x`. Then the result overwrites `x`. So `x = x + 1` means "take the current `x`, add 1, store it back in `x`."

## Why This Matters

This simple mechanism - storing and looking up names - is the foundation of all programming:

- **Parameters** are just variables that get their values from function calls
- **Loop counters** are variables that change each iteration
- **Object fields** (in Thirdlang) are variables attached to objects

The environment is one of the most important data structures in any interpreter.

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

Trace through this mentally:

- After line 1: `{a: 5}`
- After line 2: `{a: 5, b: 3}`
- After line 3: `{a: 5, b: 3, sum: 8}` (computed 5 + 3)
- And so on...

<div class="checkpoint">

At this point, you should be able to:

- Parse and evaluate `x = 42` and `y = x + 1`
- Look up variables and get their values
- Handle "undefined variable" errors gracefully

</div>

In the [next section](./functions.md), we'll see how functions use these building blocks - parameters are just local variables that get their values from call sites.

<div class="related-topics">
<strong>Related Topics</strong>

- [Functions](./functions.md) - Parameters are just local variables
- [Scoping](./recursion.md) - How each function call gets its own variables
- [Secondlang Variables](../03_secondlang/annotations.md) - Adding types to variables

</div>
