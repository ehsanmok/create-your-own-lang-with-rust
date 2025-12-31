# Constructors and Object Creation

> This chapter assumes familiarity with [function calls](../02_firstlang/functions.md) and [type checking](../03_secondlang/inference.md) from previous parts.

When you write `p = new Point(10, 20)`, several things happen:

1. Memory is allocated on the heap
2. The constructor (`__init__`) is called
3. The object is initialized with the given values
4. A pointer to the object is returned

Let us understand each step.

## The `__init__` Constructor

In Thirdlang (like Python), the constructor is named `__init__`:

```
class Point {
    x: int
    y: int

    def __init__(self, x: int, y: int) {
        self.x = x
        self.y = y
    }
}
```

The constructor:

- Always named `__init__`
- First parameter is always `self`
- Has no explicit return type (implicitly returns nothing)
- Responsible for initializing all fields

### Constructor Parameters

The constructor parameters (after `self`) become the arguments to `new`:

```
def __init__(self, x: int, y: int) { ... }
#                  ^^^^^^^^^^^^^ These become:
p = new Point(10, 20)
#             ^^^^^^ Constructor arguments
```

When you call `new Point(10, 20)`:

1. `self` is the newly allocated object
2. `x` is `10`
3. `y` is `20`

### Initializing Fields

Inside `__init__`, we use `self.field = value` to set fields:

```
def __init__(self, x: int, y: int) {
    self.x = x    # Set the 'x' field to parameter 'x'
    self.y = y    # Set the 'y' field to parameter 'y'
}
```

Fields *must* be initialized before the object can be used. Accessing an uninitialized field is undefined behavior (like uninitialized variables in C).

## The `new` Expression

The `new` keyword creates objects:

```
p = new Point(10, 20)
```

This is an expression that:

1. **Allocates memory** - Enough bytes for all fields
2. **Calls `__init__`** - Passing the new object as `self`
3. **Returns a pointer** - To the newly created object

### How `new` Works

<p align="center">
</br>
    <a href><img alt="How new works" src="../img/how-new-works.svg"> </a>
</p>

The result is a pointer to initialized memory.

## Type Checking Constructors

The type checker verifies constructor calls:

```rust,ignore
{{#include ../../../thirdlang/src/typeck.rs:typecheck}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/typeck.rs">thirdlang/src/typeck.rs</a>

For `new ClassName(args)`:

1. **Check class exists** - Is there a class named `ClassName`?
2. **Check constructor exists** - Does it have `__init__`?
3. **Check argument count** - Right number of arguments (excluding `self`)?
4. **Check argument types** - Do types match the constructor parameters?
5. **Return class type** - The expression has type `Class("ClassName")`

### Example Type Check

```
class Point {
    x: int
    y: int
    def __init__(self, x: int, y: int) { ... }
}

p = new Point(10, 20)      # OK: 2 args match (int, int)
q = new Point(10)          # ERROR: expected 2 args, got 1
r = new Point(10, true)    # ERROR: expected int, got bool
```

## Code Generation for `new`

Here is how we generate LLVM IR for object creation:

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:compile_new}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/codegen.rs">thirdlang/src/codegen.rs</a>

The generated LLVM IR looks like:

```
; new Point(10, 20)
%size = call i64 @llvm.sizeof.s_Point()
%raw = call ptr @malloc(i64 %size)
call void @Point__init(ptr %raw, i64 10, i64 20)
; %raw is now a pointer to an initialized Point
```

## Memory Layout

Objects are laid out in memory as LLVM structs:

```
class Point {
    x: int    # offset 0, 8 bytes
    y: int    # offset 8, 8 bytes
}             # total: 16 bytes
```

In LLVM IR:

```
%Point = type { i64, i64 }
;              ^^^  ^^^
;               x    y
```

Field order matters! We use `field_order` in `ClassInfo` to maintain consistent layout.

## Constructors Without Parameters

Some classes have zero-parameter constructors:

```
class Counter {
    count: int

    def __init__(self) {
        self.count = 0
    }
}

c = new Counter()   # No arguments
```

The constructor still receives `self`, but no other arguments.

## Multi-Field Initialization

For classes with many fields, the constructor initializes them all:

```
class Rectangle {
    x: int
    y: int
    width: int
    height: int

    def __init__(self, x: int, y: int, w: int, h: int) {
        self.x = x
        self.y = y
        self.width = w
        self.height = h
    }
}

r = new Rectangle(0, 0, 100, 50)
```

Each field gets initialized in the constructor body.

## Common Patterns

### Default Values in Constructor

```
class Config {
    value: int
    enabled: bool

    def __init__(self) {
        self.value = 42      # Default value
        self.enabled = true  # Default enabled
    }
}
```

### Computed Initialization

```
class Square {
    side: int
    area: int

    def __init__(self, side: int) {
        self.side = side
        self.area = side * side  # Computed from input
    }
}
```

### Validation (sort of)

Since we do not have exceptions, validation is limited:

```
class PositiveInt {
    value: int

    def __init__(self, v: int) {
        # Cannot truly validate, but can clamp
        if (v < 0) {
            self.value = 0
        } else {
            self.value = v
        }
    }
}
```

## What About Failure?

Our constructors cannot fail. In real languages, constructors might:

- Throw exceptions (Java, Python)
- Return `Result` or `Option` (Rust)
- Use factory methods instead

We keep things simple: constructors always succeed.

## Summary

| Concept | Syntax | Purpose |
|---------|--------|---------|
| Constructor | `def __init__(self, ...)` | Initialize new objects |
| Object creation | `new ClassName(args)` | Allocate and initialize |
| Field assignment | `self.field = value` | Set field values |
| Return type | Implicit `Unit` | Constructors don't return values |

In the next chapter, we look at [methods and the `self` parameter](methods.md).
