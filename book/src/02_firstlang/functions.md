# Functions

Functions are the heart of any programming language. They let us:

- **Name** a piece of computation
- **Parameterize** it with inputs
- **Reuse** it multiple times

## Defining Functions

In Firstlang, we define functions with `def`:

```python
def add(a, b) {
    return a + b
}
```

This creates a function named `add` that takes two parameters `a` and `b`, and returns their sum.

## The AST

Functions are represented in our AST as:

```rust
enum Stmt {
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    // ...
}
```

## How Function Calls Work

When we call `add(3, 4)`:

1. **Look up** the function by name
2. **Evaluate** the arguments (3 and 4)
3. **Create a new frame** for this call
4. **Bind parameters** to argument values (`a = 3, b = 4`)
5. **Execute** the function body
6. **Return** the result and pop the frame

Here's the key interpreter code:

```rust
Expr::Call { name, args } => {
    // Look up function
    let func = self.lookup_var(name)?;

    if let Value::Function { params, body } = func {
        // Evaluate arguments
        let arg_values = args.iter()
            .map(|a| self.eval_expr(a))
            .collect();

        // Create new frame with parameter bindings
        let mut frame = Frame::new();
        for (param, arg) in params.iter().zip(arg_values) {
            frame.locals.insert(param.clone(), arg);
        }

        // Push frame, execute, pop frame
        self.call_stack.push(frame);
        let result = self.execute_body(&body)?;
        self.call_stack.pop();

        Ok(result)
    }
}
```

## The Call Stack

Each function call creates a new "frame" on the call stack. This is crucial for:

- **Local variables** - Each call has its own `a` and `b`
- **Recursion** - Multiple calls can be "in flight" simultaneously

```python
def outer() {
    x = 1
    return inner()
}

def inner() {
    y = 2
    return y
}

outer()
```

During execution:

```
Call Stack:
┌─────────────┐
│ inner       │  ← current frame (y = 2)
│ x = 1, y = ?│
├─────────────┤
│ outer       │  ← previous frame (x = 1)
│ x = 1       │
├─────────────┤
│ global      │
└─────────────┘
```

## Examples

### Simple Function

```python
def square(x) {
    return x * x
}

square(5)       # 25
```

### Multiple Parameters

```python
def area(width, height) {
    return width * height
}

area(4, 5)      # 20
```

### Function Composition

```python
def double(x) {
    return x * 2
}

def quadruple(x) {
    return double(double(x))
}

quadruple(5)    # 20
```

Next, we'll add [control flow](./control_flow.md) to make our functions more powerful.
