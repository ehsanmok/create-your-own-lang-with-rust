# LLVM Code Generation for Classes

> **Prerequisites**: This chapter builds directly on [Secondlang's code generation](../03_secondlang/codegen.md). Make sure you understand how we compile expressions and functions to LLVM IR before proceeding.

Now we see how classes translate to LLVM IR. The key insight: **classes become structs, methods become functions**. We extend the patterns from [From AST to IR](../03_secondlang/ir.md) with new concepts for object-oriented features.

## The CodeGen Structure

Our code generator has new fields for class support:

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:codegen_struct}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/codegen.rs">thirdlang/src/codegen.rs</a>

The new fields:

- **class_types** - Maps class names to LLVM struct types
- **classes** - Maps class names to `ClassInfo` (field/method metadata)
- **current_class** - The class we are currently compiling (for `self` resolution)

## Compilation Pipeline

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:compile}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/codegen.rs">thirdlang/src/codegen.rs</a>

The compilation happens in phases:

1. **Declare libc functions** - `malloc` and `free`
2. **Create class struct types** - Define LLVM struct for each class
3. **Declare methods** - Create function signatures
4. **Compile class bodies** - Generate method implementations
5. **Compile top-level code** - Generate `__main` wrapper
6. **Verify module** - Check IR is well-formed

## Classes as LLVM Structs

Each class becomes an LLVM struct type:

```
class Point {
    x: int
    y: int
}
```

Becomes:

```
%Point = type { i64, i64 }
;               ^^^  ^^^
;                x    y (in field_order)
```

### Creating the Struct Type

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:create_class_type}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/codegen.rs">thirdlang/src/codegen.rs</a>

The `field_order` is crucial - it determines the memory layout.

## Methods as Functions

Methods compile to regular functions with a naming convention:

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
entry:
    %x_ptr = getelementptr %Point, ptr %self, i32 0, i32 0
    %x = load i64, ptr %x_ptr
    ret i64 %x
}
```

### Method Naming

We use `ClassName__methodName`:

| Method | LLVM Function Name |
|--------|--------------------|
| `Point.__init__` | `@Point____init__` |
| `Point.get_x` | `@Point__get_x` |
| `Counter.increment` | `@Counter__increment` |

This avoids name collisions between classes.

### Self as First Parameter

Every method takes `self` (a pointer) as its first parameter:

```
define i64 @Point__get_x(ptr %self) { ... }
;                        ^^^^^^^^ self is a pointer to Point
```

When calling `p.get_x()`, we pass `p` as the first argument.

## Field Access

Reading a field uses LLVM's `getelementptr` (GEP):

```
return self.x
```

Becomes:

```
%x_ptr = getelementptr %Point, ptr %self, i32 0, i32 0
;                                         ^^^     ^^^
;                                         struct  field index
%x = load i64, ptr %x_ptr
ret i64 %x
```

The GEP instruction calculates the address of field 0 (which is `x`).

### Writing a Field

```
self.x = 42
```

Becomes:

```
%x_ptr = getelementptr %Point, ptr %self, i32 0, i32 0
store i64 42, ptr %x_ptr
```

Same GEP, but followed by `store` instead of `load`.

## Compiling Expressions

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:compile_expr}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/codegen.rs">thirdlang/src/codegen.rs</a>

The important new cases:

### Self Reference

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:compile_self_ref}}
```

`self` is stored as a local variable pointer, just like other parameters.

### New Expression

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:compile_new}}
```

### Field Access

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:compile_field_access}}
```

### Method Call

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:compile_method_call}}
```

## Complete Example

Let us trace through compiling this class:

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
}

c = new Counter()
c.increment()
```

### Generated LLVM IR

```
; Struct type
%Counter = type { i64 }

; Constructor
define void @Counter____init__(ptr %self) {
entry:
    %count_ptr = getelementptr %Counter, ptr %self, i32 0, i32 0
    store i64 0, ptr %count_ptr
    ret void
}

; Increment method
define i64 @Counter__increment(ptr %self) {
entry:
    ; self.count + 1
    %count_ptr = getelementptr %Counter, ptr %self, i32 0, i32 0
    %count = load i64, ptr %count_ptr
    %new_count = add i64 %count, 1

    ; self.count = new_count
    store i64 %new_count, ptr %count_ptr

    ; return self.count
    %result = load i64, ptr %count_ptr
    ret i64 %result
}

; Main function
define i64 @__main() {
entry:
    ; c = new Counter()
    %raw = call ptr @malloc(i64 8)
    call void @Counter____init__(ptr %raw)

    ; c.increment()
    %result = call i64 @Counter__increment(ptr %raw)

    ret i64 %result
}
```

## JIT Execution

Now that we can generate IR for classes, let's actually run it. Here's how we take our Thirdlang program from source code to executed result:

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:jit_run}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/codegen.rs">thirdlang/src/codegen.rs</a>

This follows the same pattern as Secondlang, but now with class support. Let's walk through what happens:

1. **Create the code generator** - This sets up our LLVM context, module, and builder. Think of it as preparing our workspace.

2. **Compile the program** - We walk through the AST and emit LLVM IR for each class, method, and statement. When we encounter `new Counter()`, we emit malloc + constructor calls. When we see `c.increment()`, we emit a method call with `c` passed as `self`.

3. **Create the JIT engine** - LLVM takes our IR and compiles it to native machine code for your CPU. This happens at runtime, hence "Just-In-Time".

4. **Get the `__main` function** - Remember how we wrapped top-level code in a `__main` function? We look it up in the compiled code.

5. **Call it and return** - We execute the native code. When it calls `malloc`, it's calling the real C malloc. When it calls our constructor, it's running the native x86/ARM code we generated. Fast!

The magic is that the code we're running isn't being interpreted - it's real compiled code, just like if you wrote it in C or Rust. Objects really live on the heap. Methods really jump to function addresses. It's all native.

## Memory Layout Summary

| Thirdlang | LLVM IR | Notes |
|-----------|---------|-------|
| `class Point { x: int }` | `%Point = type { i64 }` | Struct type |
| `new Point(10)` | `call @malloc` + `call @Point____init__` | Heap allocation |
| `p.x` | `getelementptr` + `load` | Field read |
| `p.x = 5` | `getelementptr` + `store` | Field write |
| `p.method()` | `call @Point__method(ptr %p)` | Method call |
| `delete p` | `call @Point____del__` + `call @free` | Destruction |

## Performance Considerations

Our implementation is straightforward but not optimal:

### What We Do

- Direct field access via GEP (fast)
- Method calls are static (no vtable lookup)
- Objects are contiguous in memory

### What Real Compilers Add

- Inline method calls when possible
- Escape analysis (stack allocate short-lived objects)
- Field alignment optimization
- Dead field elimination

Our simple approach is sufficient for learning the concepts.

## Summary

| Concept | Implementation |
|---------|----------------|
| Class | LLVM struct type |
| Object | Pointer to struct on heap |
| Field | Struct element (GEP access) |
| Method | Function with self as first param |
| Constructor | `ClassName____init__` function |
| Destructor | `ClassName____del__` function |
| new | malloc + constructor call |
| delete | destructor call + free |

<div class="checkpoint">

At this point, you should be able to:

- Run `cargo run --bin thirdlang -- --ir examples/point.tl` and see IR
- See `%Point` struct type in the output
- See `Point__init` and other method functions

</div>

<div class="related-topics">
<strong>Related Topics</strong>

- [Secondlang Codegen](../03_secondlang/codegen.md) - The foundation we extended
- [From AST to IR](../03_secondlang/ir.md) - Understanding LLVM IR
- [Memory Management](memory.md) - How malloc/free work
- [Optimizing IR](optimization.md) - Making the IR efficient

</div>

In the final chapter, we [run Thirdlang programs](running.md) and see everything working together.
