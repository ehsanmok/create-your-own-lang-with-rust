# Thirdlang

An object-oriented programming language with classes, methods, and explicit memory management.

## Features

- **Classes** with fields and methods
- **Constructors** (`__init__`) for object initialization
- **Destructors** (`__del__`) for cleanup on deletion
- **Object creation** with `new ClassName(args)`
- **Object deletion** with `delete obj`
- **Method calls** with `obj.method(args)`
- **Field access** with `obj.field`
- **Static type checking** with type inference
- **JIT compilation** via LLVM

## Requirements

- Rust nightly toolchain
- LLVM 20 (with development headers)

## Building

```bash
cd thirdlang
rustup run nightly cargo build
```

## Running Examples

```bash
# Point distance calculation
rustup run nightly cargo run -- examples/point.tl
# Output: 25

# Counter with increment
rustup run nightly cargo run -- examples/counter.tl
# Output: 18

# Destructor demonstration
rustup run nightly cargo run -- examples/destructor.tl
# Output: 1

# Multiple nodes
rustup run nightly cargo run -- examples/linked_node.tl
# Output: 60
```

## Running Tests

```bash
rustup run nightly cargo test
```

## CLI Options

```bash
thirdlang <file.tl>                  # JIT compile and run
thirdlang --ir <file.tl>             # Print LLVM IR (unoptimized)
thirdlang --ir -O <file.tl>          # Print LLVM IR (optimized with default<O2>)
thirdlang --ir --passes mem2reg,dce <file.tl>  # Print IR with specific passes
thirdlang -O <file.tl>               # Run with default<O2> optimizations
thirdlang --passes dce,instcombine <file.tl>   # Run with custom passes
thirdlang --ast <file.tl>            # Print AST
thirdlang --check <file.tl>          # Type check only
```

### Optimization Passes

| Pass | What it does |
|------|--------------|
| `mem2reg` | Promotes stack allocations to SSA registers |
| `dce` | Dead Code Elimination |
| `instcombine` | Combines redundant instructions |
| `simplifycfg` | Simplifies control flow graph |
| `default<O2>` | Standard optimization level (used by `-O`) |

## Language Syntax

### Class Definition

```python
class Point {
    x: int
    y: int

    def __init__(self, x: int, y: int) {
        self.x = x
        self.y = y
    }

    def distance_squared(self, other: Point) -> int {
        dx = self.x - other.x
        dy = self.y - other.y
        return dx * dx + dy * dy
    }

    def __del__(self) {
        # Cleanup code (optional)
    }
}
```

### Object Lifecycle

```python
# Create object (calls __init__)
p = new Point(3, 4)

# Use object
x = p.x              # Field access
d = p.distance_squared(other)  # Method call

# Delete object (calls __del__, then frees memory)
delete p
```

### Types

- `int` - 64-bit signed integer
- `bool` - Boolean (`true`/`false`)
- `ClassName` - Class type (always a pointer)

### Methods

Methods always have `self` as first parameter (implicitly typed as the class):

```python
def method_name(self, param1: type1, param2: type2) -> return_type {
    # Method body
    return value
}
```

### Special Methods

- `__init__` - Constructor, called by `new`
- `__del__` - Destructor, called by `delete`

## Memory Model

- Objects are **heap-allocated** via `malloc`
- Objects are **explicitly freed** via `delete`
- **No garbage collection** - you must delete objects manually
- **Dangling pointers** are possible (use after delete = undefined behavior)
- **Double free** is undefined behavior

This is educational - real languages use reference counting or GC.

## Comparison with Secondlang

| Feature | Secondlang | Thirdlang |
|---------|------------|-----------|
| Types | `int`, `bool` | `int`, `bool`, classes |
| Functions | Yes | Yes |
| Classes | No | Yes |
| Methods | No | Yes |
| Objects | No | Yes (`new`/`delete`) |
| Memory | Stack only | Stack + heap |

## Architecture

```
Source Code
     │
     ▼
  [Parser]  ──► pest grammar (grammar.pest)
     │
     ▼
Typed AST   ──► types.rs, ast.rs
     │
     ▼
[Type Checker] ──► typeck.rs
     │
     ▼
[Code Generator] ──► codegen.rs
     │
     ▼
  LLVM IR
     │
     ▼
[JIT Engine]
     │
     ▼
Native Code
```

## Key Files

| File | Description |
|------|-------------|
| `grammar.pest` | PEG grammar with class syntax |
| `types.rs` | Type system with `ClassInfo` |
| `ast.rs` | AST nodes including `ClassDef`, `MethodCall`, `New` |
| `parser.rs` | Parser for class definitions and expressions |
| `typeck.rs` | Type checker with class/method resolution |
| `codegen.rs` | LLVM codegen with malloc/free, method compilation |
