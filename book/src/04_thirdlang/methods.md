# Methods and Self

> If you have not read about [functions in Firstlang](../02_firstlang/functions.md), review that first. Methods are similar to functions but attached to objects.

Methods are functions that belong to a class. They always receive the object as their first parameter, called `self`.

## Method Definition

Methods are defined inside a class using `def`:

```
class Counter {
    count: int

    def __init__(self) {
        self.count = 0
    }

    def increment(self) -> int {
        self.count = self.count + 1
        return self.count
    }

    def add(self, n: int) -> int {
        self.count = self.count + n
        return self.count
    }

    def get(self) -> int {
        return self.count
    }
}
```

Every method:

1. Starts with `def`
2. Has `self` as its first parameter
3. Can have additional parameters after `self`
4. Can have a return type

## The `self` Parameter

`self` is a reference to the object the method was called on:

```
c = new Counter()
c.increment()   # Inside increment(), self == c
```

When you call `c.increment()`:

1. The object `c` becomes `self`
2. The method body executes with that `self`
3. `self.count` accesses `c`'s `count` field

### Self is Explicit

Unlike Java or C++ (where `this` is implicit), Thirdlang requires explicit `self`:

```
# Thirdlang (explicit self)
def get(self) -> int {
    return self.count    # Must write self.
}

# Java (implicit this)
// int get() {
//     return count;     // 'this.' is optional
// }
```

Explicit `self` is clearer: you always know when you are accessing object state.

## Method Calls

Call a method using dot notation:

```
c = new Counter()
c.increment()       # Call increment on c
c.add(5)            # Call add with argument 5
x = c.get()         # Get returns an int
```

### How Method Calls Work

When you write `c.increment()`:

<p align="center">
</br>
    <a href><img alt="How method works" src="../img/how-method-works.svg"> </a>
</p>

Methods are compiled as regular functions with a mangled name: `ClassName__methodName`.

## Type Checking Methods

The type checker verifies method calls:

```rust,ignore
{{#include ../../../thirdlang/src/typeck.rs:typecheck_expr}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/typeck.rs">thirdlang/src/typeck.rs</a>

For `object.method(args)`:

1. **Type check object** - Get the object's type
2. **Verify it is a class** - Cannot call methods on `int` or `bool`
3. **Find the method** - Look up method name in the class
4. **Check arguments** - Verify argument types match parameters
5. **Return method's return type** - The expression's type

### Example Type Checks

```
class Point {
    x: int
    y: int
    def __init__(self, x: int, y: int) { ... }
    def distance_squared(self, other: Point) -> int { ... }
}

p1 = new Point(0, 0)
p2 = new Point(3, 4)

p1.distance_squared(p2)      # OK: Point method, Point argument
p1.distance_squared(5)       # ERROR: expected Point, got int
p1.nonexistent()             # ERROR: method not found
```

## Methods with Parameters

Methods can take additional parameters after `self`:

```
class Point {
    x: int
    y: int

    def __init__(self, x: int, y: int) {
        self.x = x
        self.y = y
    }

    def distance_squared(self, other: Point) -> int {
        dx = other.x - self.x
        dy = other.y - self.y
        return dx * dx + dy * dy
    }

    def translate(self, dx: int, dy: int) {
        self.x = self.x + dx
        self.y = self.y + dy
    }
}
```

The `distance_squared` method takes another `Point` and returns `int`. The `translate` method takes two integers and has no explicit return type - this means it returns `Unit` (nothing useful), it just modifies the object in place.

### Classes as Parameters

Notice `other: Point` - methods can take objects of any class as parameters:

```
def distance_squared(self, other: Point) -> int { ... }
#                          ^^^^^^^^^^^^
#                          Parameter is a Point!
```

**Important**: When you pass an object as a parameter, you are passing a **pointer** (reference), not a copy. Both `self` and `other` point to actual objects in memory. If you modify `other.x` inside the method, you are modifying the original object!

This is called **reference semantics** and is common in OOP languages (Java, Python, etc.).

## Field Access

Methods can read and write fields using `self.field`:

```
def get_x(self) -> int {
    return self.x        # Read field
}

def set_x(self, x: int) {
    self.x = x           # Write field
}
```

### Field Access Compiles to Struct GEP

In LLVM IR, `self.x` becomes a GEP (Get Element Pointer) instruction. GEP is one of LLVM's most important instructions - it calculates the memory address of a struct field without actually reading memory. Think of it as "pointer arithmetic" that knows about struct layouts:

```
; self.x = 42
%field_ptr = getelementptr %Point, ptr %self, i32 0, i32 0  ; Get pointer to field 0
store i64 42, ptr %field_ptr                                 ; Store value

; return self.x
%field_ptr = getelementptr %Point, ptr %self, i32 0, i32 0
%value = load i64, ptr %field_ptr
ret i64 %value
```

## Method Compilation

Methods compile to regular functions with a special naming convention:

```
class Point {
    def get_x(self) -> int {
        return self.x
    }
}
```

Becomes:

```
define i64 @Point__get_x(ptr %self) {
    %x_ptr = getelementptr %Point, ptr %self, i32 0, i32 0
    %x = load i64, ptr %x_ptr
    ret i64 %x
}
```

The naming pattern `ClassName__methodName`:

- Avoids conflicts between classes
- Makes it clear which class owns the method
- Allows us to find the right function at call sites

## Calling Methods on Self

Inside a method, you can call other methods on `self`:

```
class Calculator {
    value: int

    def __init__(self) {
        self.value = 0
    }

    def add(self, n: int) {
        self.value = self.value + n
    }

    def double(self) {
        self.add(self.value)   # Call add on self
    }
}
```

This is just `self.methodName(args)` - same as any other method call.

## Methods Returning Self's Type

Methods can return their own class type:

```
class Builder {
    value: int

    def __init__(self) { self.value = 0 }

    def set_value(self, v: int) -> Builder {
        self.value = v
        return self   # Return the same object
    }
}

b = new Builder()
b.set_value(10)
```

Returning `self` enables method chaining (though we do not support it syntactically yet).

## Special Methods

### Constructor: `__init__`

Called automatically when you use `new`:

```
def __init__(self, x: int) {
    self.x = x
}
```

### Destructor: `__del__`

Called automatically when you use `delete`:

```
def __del__(self) {
    # Cleanup code (if any)
}
```

We cover destructors in detail in the [memory management](memory.md) chapter.

## Summary

| Concept | Syntax | Description |
|---------|--------|-------------|
| Method definition | `def name(self, ...)` | Function belonging to a class |
| Method call | `obj.method(args)` | Call method on an object |
| Self reference | `self` | The object being operated on |
| Field read | `self.field` | Access object's field |
| Field write | `self.field = x` | Modify object's field |

Methods are the *behavior* half of object-oriented programming. Fields are data; methods are actions.

Next, let us look at [memory management](memory.md) - how objects are allocated and freed.
