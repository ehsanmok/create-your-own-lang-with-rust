# Thirdlang

> **Prerequisites**: This chapter builds on [Secondlang](../03_secondlang/intro.md). Make sure you understand [type checking](../03_secondlang/inference.md) and [LLVM code generation](../03_secondlang/codegen.md) before proceeding.

In Part III, we built [Secondlang](../03_secondlang/intro.md), a statically typed language with LLVM JIT compilation. Now we take the final step: adding **classes** and **object-oriented programming**.

## What Changes from Secondlang?

The transition from Secondlang to Thirdlang demonstrates how to add user-defined types to a language. While Secondlang has primitive types (`int`, `bool`), Thirdlang allows programmers to define their own types with **classes**.

### Grammar: ~40 Lines Added

Secondlang's type system:

```text
Type = { IntType | BoolType }
IntType = { "int" }
BoolType = { "bool" }
```

Thirdlang's type system:

```text
Type = { IntType | BoolType | ClassType }
IntType = { "int" }
BoolType = { "bool" }
ClassType = { Identifier }  // Any class name is a type!
```

Plus new constructs for classes:

```text
ClassDef = { "class" ~ Identifier ~ "{" ~ ClassBody ~ "}" }
ClassBody = { (FieldDef | MethodDef)* }
FieldDef = { Identifier ~ ":" ~ Type }
MethodDef = { "def" ~ Identifier ~ "(" ~ SelfParam? ~ ... ~ ")" ~ ... ~ Block }
NewExpr = { "new" ~ Identifier ~ "(" ~ Args? ~ ")" }
Delete = { "delete" ~ Expr }
```

The syntactic additions unlock a powerful new paradigm: **object-oriented programming**.

### Compiler: New Concepts

| Concept | Secondlang | Thirdlang |
|---------|------------|-----------|
| Types | `int`, `bool` | `int`, `bool`, `ClassName` |
| User-defined types | None | Classes with fields |
| Methods | None | Functions attached to objects |
| Memory | Stack only | Stack + heap (malloc/free) |
| Object creation | None | `new ClassName(args)` |
| Object destruction | None | `delete obj` |

The biggest change is **heap allocation**. In Secondlang, all values live on the stack. In Thirdlang, objects are allocated on the heap with `malloc` and freed with `free`.

## Why Classes?

Classes solve a fundamental problem: **grouping related data together**.

Without classes, you might write:

```
# Unrelated variables floating around
point_x = 10
point_y = 20
point2_x = 30
point2_y = 40

def distance(x1, y1, x2, y2) -> int {
    dx = x2 - x1
    dy = y2 - y1
    return dx * dx + dy * dy
}

distance(point_x, point_y, point2_x, point2_y)
```

With classes:

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
}

p1 = new Point(10, 20)
p2 = new Point(30, 40)
p1.distance_squared(p2)
```

The second version is:

- **More organized** - data and behavior are grouped together
- **Safer** - you cannot accidentally mix up different points
- **More readable** - `p1.distance_squared(p2)` is clearer than `distance(x1, y1, x2, y2)`

## Feature Comparison

| Feature | [Firstlang](../02_firstlang/intro.md) | [Secondlang](../03_secondlang/intro.md) | Thirdlang |
|---------|-----------|------------|-----------|
| Type System | Dynamic | Static | Static |
| User-defined Types | None | None | Classes |
| Fields | None | None | `self.x` |
| Methods | None | None | `def method(self)` |
| Constructor | None | None | `__init__` |
| Destructor | None | None | `__del__` |
| Object Creation | None | None | `new Point(...)` |
| Object Deletion | None | None | `delete obj` |
| Memory Model | Stack | Stack | Stack + Heap |

## Syntax Examples

### Class Definition

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

    def get(self) -> int {
        return self.count
    }
}
```

### Object Creation and Method Calls

```
c = new Counter()
c.increment()   # returns 1
c.increment()   # returns 2
c.get()         # returns 2
delete c        # free memory
```

### Classes as Parameters

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
}

p1 = new Point(0, 0)
p2 = new Point(3, 4)
p1.distance_squared(p2)  # returns 25
```

## Design Decisions

We made several deliberate simplifications:

### No Inheritance

Many OOP languages support **inheritance** (class B extends class A). We skip this because:

1. It adds significant complexity (vtables, dynamic dispatch)
2. [Composition over inheritance](https://en.wikipedia.org/wiki/Composition_over_inheritance) is often preferred
3. The core concepts are clearer without it

### Explicit Memory Management

Instead of garbage collection, we use **explicit delete**:

```
p = new Point(1, 2)
# ... use p ...
delete p  # programmer's responsibility
```

This is similar to C++ and teaches important concepts:

- Objects live on the heap
- Memory must be freed when no longer needed
- Forgetting to `delete` causes memory leaks

### Python-like Constructor Syntax

We use `__init__` for constructors (like Python):

```
def __init__(self, x: int) {
    self.x = x
}
```

And `__del__` for destructors:

```
def __del__(self) {
    # cleanup code
}
```

## Project Structure

```
thirdlang/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Library exports
│   ├── main.rs          # CLI entry point
│   ├── grammar.pest     # PEG grammar with classes
│   ├── parser.rs        # Parser → Typed AST
│   ├── ast.rs           # Typed AST with class nodes
│   ├── types.rs         # Type system with ClassInfo
│   ├── typeck.rs        # Type checker for classes
│   ├── visitor.rs       # AST visitors
│   └── codegen.rs       # LLVM code generation with malloc/free
├── examples/
│   ├── point.tl         # Point class with methods
│   ├── counter.tl       # Counter with state
│   ├── destructor.tl    # Destructor example
│   └── linked_node.tl   # Linked data structure
└── tests/
    └── integration_tests.rs
```

Compare to [Secondlang's structure](../03_secondlang/intro.md#project-structure):

```
secondlang/
├── src/
│   ├── types.rs         # Simpler: no ClassInfo
│   ├── typeck.rs        # Simpler: no class type checking
│   └── codegen.rs       # Simpler: no malloc/free
```

The new additions handle class metadata, method resolution, and heap memory management.

## Prerequisites

Like Secondlang, Thirdlang requires LLVM and nightly Rust:

```bash
# Install nightly Rust
rustup toolchain install nightly

# Check LLVM version
llvm-config --version
```

Update `Cargo.toml` for your LLVM version:

- LLVM 20.x: `features = ["llvm20-1"]`
- LLVM 19.x: `features = ["llvm19-1"]`

## Quick Start

```bash
cd thirdlang

# Run Point example
rustup run nightly cargo run --bin thirdlang -- examples/point.tl

# Run Counter example
rustup run nightly cargo run --bin thirdlang -- examples/counter.tl

# Run all tests
rustup run nightly cargo test
```

## Outline

In the following chapters, we build Thirdlang step by step:

1. [Why Classes?](why_classes.md) - Object-oriented concepts
2. [Class Syntax and Parsing](classes_syntax.md) - Grammar and AST
3. [Constructors and Object Creation](constructors.md) - `__init__` and `new`
4. [Methods and Self](methods.md) - Method calls and the `self` parameter
5. [Memory Management](memory.md) - Heap allocation with malloc/free
6. [LLVM Code Generation for Classes](codegen_classes.md) - Structs and method compilation
7. [Running Thirdlang](running.md) - Examples and tests
