# Memory Management

> This chapter introduces heap allocation. If you are new to the stack vs heap distinction, this is a fundamental concept in systems programming.

In Thirdlang, objects live on the **heap** and must be explicitly freed. This is different from garbage-collected languages like Java or Python, and similar to C++ or Rust (without RAII). Compare this to [Secondlang](../03_secondlang/intro.md) where all values lived on the stack.

## Stack vs Heap

> The stack is like a stack of cafeteria trays - you can only add or remove from the top, and they're all the same size. The heap is like a parking lot - you can park (allocate) anywhere there's space, leave your car as long as you want, but you must remember to retrieve it (free) or it stays forever (memory leak).

Let us review the two types of memory:

### Stack Memory

- **Automatic** - Allocated/freed with function calls
- **Fast** - Just move a pointer
- **Limited size** - Typically a few MB
- **LIFO** - Last in, first out

In Secondlang, all values live on the stack:

```
def foo() {
    x = 10      # x is on the stack
    y = 20      # y is on the stack
    return x + y
}               # x, y automatically freed
```

### Heap Memory

- **Manual** - You allocate and free
- **Slower** - System call to OS
- **Large** - Can use all available RAM
- **Flexible** - Allocate any time, free any time

In Thirdlang, objects live on the heap:

```
p = new Point(1, 2)   # Allocate on heap
# ... use p ...
delete p              # Must free manually!
```

**Why the heap?** We chose heap allocation because:

1. Objects can outlive the function that created them
2. Multiple variables can reference the same object
3. It mirrors how most OOP languages work (Java, Python, etc.)

Note: Some languages (like Rust or C++) can place objects on the stack for efficiency. We keep things simple with heap-only allocation.

## The `new` Operator

`new` allocates heap memory:

```
p = new Point(10, 20)
```

Behind the scenes:

1. **Calculate size** - How many bytes for a Point?
2. **Call malloc** - Ask OS for memory
3. **Call constructor** - Initialize the memory
4. **Return pointer** - Give caller the address

### Size Calculation

```
class Point {
    x: int    # 8 bytes (i64)
    y: int    # 8 bytes (i64)
}             # Total: 16 bytes
```

LLVM calculates this for us using `sizeof`.

### Malloc

We declare `malloc` from the C library:

```
declare ptr @malloc(i64)   ; Takes size, returns pointer
```

Then call it:

```
%ptr = call ptr @malloc(i64 16)   ; Allocate 16 bytes
```

## The `delete` Operator

`delete` frees heap memory:

```
delete p
```

Behind the scenes:

1. **Call destructor** (if exists) - Run `__del__`
2. **Call free** - Return memory to OS

### Free

We declare `free` from the C library:

```
declare void @free(ptr)   ; Takes pointer, returns nothing
```

Then call it:

```
call void @free(ptr %p)   ; Free the memory
```

## Destructors: `__del__`

The destructor is called automatically when you `delete` an object:

```
class Resource {
    id: int

    def __init__(self, id: int) {
        self.id = id
    }

    def __del__(self) {
        # Cleanup code here
        # (In a real language, might close files, release handles, etc.)
    }
}

r = new Resource(42)
delete r   # Calls __del__, then free()
```

### Destructor Rules

- Named `__del__`
- Only parameter is `self`
- No return type
- Called before memory is freed
- Optional - if not defined, just free memory

### Use Cases

In real languages, destructors:

- Close file handles
- Release network connections
- Free nested allocations
- Log cleanup events

In Thirdlang, we keep it simple - destructors can run any code.

## Code Generation for delete

Here is how we compile `delete`:

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:compile_delete}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/codegen.rs">thirdlang/src/codegen.rs</a>

Generated LLVM IR:

```
; delete p  (where p is a Point with destructor)
call void @Point__del(ptr %p)   ; Destructor
call void @free(ptr %p)          ; Free memory
```

## Memory Safety Issues

Without automatic memory management, several bugs become possible:

### Memory Leak

Forgetting to `delete`:

```
def leak() {
    p = new Point(1, 2)
    # Oops, forgot delete p!
}   # Memory is lost forever
```

The memory stays allocated until the program exits.

### Use After Free

Using an object after deleting it:

```
p = new Point(1, 2)
delete p
p.x   # BUG! Memory already freed
```

This is **[undefined behavior](https://en.wikipedia.org/wiki/Undefined_behavior)** - anything can happen.

### Double Free

Deleting the same object twice:

```
p = new Point(1, 2)
delete p
delete p   # BUG! Already freed
```

Also undefined behavior - might crash, might corrupt memory.

### Dangling Pointer

Multiple variables pointing to freed memory:

```
p = new Point(1, 2)
q = p              # Both point to same object
delete p
q.x               # BUG! q is now dangling
```

## Why Manual Memory Management?

We chose explicit `new`/`delete` for educational purposes:

| Approach | Pros | Cons |
|----------|------|------|
| **Manual (C, C++)** | Fast, predictable, teaches fundamentals | Error-prone |
| **Garbage Collection (Java, Python)** | Safe, convenient | Overhead, pauses |
| **RAII (Rust, C++)** | Safe, no runtime cost | Complex ownership rules |
| **Reference Counting (Swift, Python)** | Predictable cleanup | Cycles, overhead |

Understanding manual management helps you appreciate what other approaches solve.

## Memory Layout Example

Let us trace through a complete example:

```
class Point {
    x: int
    y: int

    def __init__(self, x: int, y: int) {
        self.x = x
        self.y = y
    }

    def __del__(self) {
        # Cleanup
    }
}

p = new Point(10, 20)
delete p
```

### Step 1: new Point(10, 20)

<p align="center">
</br>
    <a href><img alt="new Point(10, 20)" src="../img/new-point.svg"> </a>
</p>

1. `malloc(16)` returns `0x1000`
2. `Point__init(0x1000, 10, 20)` initializes fields
3. `p` holds the pointer `0x1000`

### Step 2: delete p

<p align="center">
</br>
    <a href><img alt="delete ptr" src="../img/delete-ptr.svg"> </a>
</p>

1. `Point__del(0x1000)` runs destructor
2. `free(0x1000)` returns memory to OS
3. `p` still holds `0x1000` but it is invalid now!

## Best Practices

Even though our language is simple, good habits help:

### 1. Delete What You Allocate

```
p = new Point(1, 2)
# ... use p ...
delete p   # Always clean up
```

### 2. Set to "null" After Delete (If We Had Null)

In real languages:

```
delete p
p = null   # Mark as invalid
```

### 3. One Owner

Have a clear owner responsible for deletion:

```
# Function creates and returns - caller owns it
def make_point() -> Point {
    return new Point(1, 2)
}

p = make_point()   # Caller is responsible
# ... use p ...
delete p           # Caller deletes
```

## Summary

| Operation | What It Does | When |
|-----------|--------------|------|
| `new Class(args)` | Allocate + initialize | When you need an object |
| `delete obj` | Destruct + free | When done with object |
| `__init__` | Initialize fields | Called by `new` |
| `__del__` | Cleanup before free | Called by `delete` |

Memory management is one of the hardest parts of systems programming. Our simple model teaches the fundamentals without the full complexity of real-world solutions.

<div class="checkpoint">

At this point, you should understand:

- Why objects live on the heap (can outlive functions, shared references)
- How `new` calls malloc then the constructor
- How `delete` calls the destructor then free
- Common memory bugs: leaks, use-after-free, double free

</div>

<div class="related-topics">
<strong>Related Topics</strong>

- [Constructors](constructors.md) - The `__init__` method
- [Why Classes](why_classes.md) - Stack vs heap motivation
- [LLVM Codegen](codegen_classes.md) - How new/delete compile to IR

</div>

Next, let us look at [LLVM code generation for classes](codegen_classes.md).
