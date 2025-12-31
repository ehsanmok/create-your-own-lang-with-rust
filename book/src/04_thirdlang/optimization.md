# Optimizing LLVM IR

So far, our compiler generates correct but naive LLVM IR. Every variable gets a stack allocation (alloca), every parameter is stored and reloaded, and the code includes redundant operations. In this chapter, we integrate LLVM's pass manager to optimize our IR before execution.

> **Prerequisites**: This chapter assumes you understand LLVM IR syntax. If instructions like `alloca`, `load`, `store`, and `getelementptr` are unfamiliar, read [From AST to IR](../03_secondlang/ir.md) first.

> **Requirement**: This chapter requires LLVM 20+ because the `run_passes()` API we use is part of LLVM's New Pass Manager, which is fully supported in LLVM 20. Check your version with `llvm-config --version`.
>
> If you have a different LLVM version, update `thirdlang/Cargo.toml`:
>
> ```toml
> # For LLVM 20.x
> inkwell = { version = "0.7", features = ["llvm20-1"] }
> ```

## What Are Optimization Passes?

An optimization pass is a transformation that improves the IR without changing its behavior. LLVM provides dozens of passes that work together in a **pipeline**:

| Pass | What It Does |
|------|--------------|
| `mem2reg` | Promotes stack allocations to SSA registers |
| `dce` | Removes dead (unused) code |
| `instcombine` | Combines redundant instructions |
| `simplifycfg` | Simplifies control flow graph |

## The Optimization Pipeline in Action

Let us trace how each pass transforms the IR for a simple `increment` method. This is the most important section to understand.

### Step 0: Original Source

Create `examples/counter.tl` with a Counter class (or use the existing one):

```
class Counter {
    value: int

    def increment(self) -> int {
        self.value = self.value + 1
        return self.value
    }
}
```

### Step 1: Unoptimized IR (what our codegen produces)

Generate with:

```bash
cd thirdlang
rustup run nightly cargo run -- --ir examples/counter.tl
```

Output (showing just the `increment` method):

```
define i64 @Counter__increment(ptr %self) {
entry:
  %self1 = alloca ptr, align 8              ; allocate stack slot for self
  store ptr %self, ptr %self1, align 8      ; store parameter to stack
  %self2 = load ptr, ptr %self1, align 8    ; load self from stack
  %field_ptr = getelementptr %Counter, ptr %self2, i32 0, i32 0
  %field = load i64, ptr %field_ptr         ; load self.value
  %add = add i64 %field, 1                  ; add 1
  %self3 = load ptr, ptr %self1, align 8    ; load self again!
  %field_ptr4 = getelementptr %Counter, ptr %self3, i32 0, i32 0
  store i64 %add, ptr %field_ptr4           ; store to self.value
  %self5 = load ptr, ptr %self1, align 8    ; load self a third time!
  %field_ptr6 = getelementptr %Counter, ptr %self5, i32 0, i32 0
  %field7 = load i64, ptr %field_ptr6       ; load self.value for return
  ret i64 %field7
}
```

**Problems**: 14 instructions, redundant loads, stack allocation for a parameter.

### Step 2: After `mem2reg`

Generate with:

```bash
rustup run nightly cargo run -- --ir --passes "mem2reg" examples/counter.tl
```

The `mem2reg` pass promotes the alloca to an SSA register:

```
define i64 @Counter__increment(ptr %self) {
entry:
  ; No more alloca! %self is used directly
  %field_ptr = getelementptr %Counter, ptr %self, i32 0, i32 0
  %field = load i64, ptr %field_ptr
  %add = add i64 %field, 1
  %field_ptr4 = getelementptr %Counter, ptr %self, i32 0, i32 0
  store i64 %add, ptr %field_ptr4
  %field_ptr6 = getelementptr %Counter, ptr %self, i32 0, i32 0
  %field7 = load i64, ptr %field_ptr6
  ret i64 %field7
}
```

**Result**: Eliminated alloca, store, and 3 redundant loads of `%self`.

### Step 3: After `mem2reg,instcombine`

Generate with:

```bash
rustup run nightly cargo run -- --ir --passes "mem2reg,instcombine" examples/counter.tl
```

The `instcombine` pass merges redundant GEP instructions and simplifies patterns:

```
define i64 @Counter__increment(ptr %self) {
entry:
  %field = load i64, ptr %self              ; GEPs merged, %self IS the field ptr
  %add = add i64 %field, 1
  store i64 %add, ptr %self
  %field7 = load i64, ptr %self             ; still loading for return
  ret i64 %field7
}
```

**Result**: Since Counter has only one field at offset 0, GEP simplifies away.

### Step 4: After full pipeline (with `dce`)

Generate with:

```bash
rustup run nightly cargo run -- --ir -O examples/counter.tl
```

Or explicitly:

```bash
rustup run nightly cargo run -- --ir --passes "mem2reg,instcombine,dce,simplifycfg" examples/counter.tl
```

The `dce` pass removes the unnecessary final load - we already have the value in `%add`:

```
define i64 @Counter__increment(ptr %self) {
entry:
  %field = load i64, ptr %self, align 4
  %add = add i64 %field, 1
  store i64 %add, ptr %self, align 4
  ret i64 %add                              ; return %add directly!
}
```

**Final Result**: 4 instructions instead of 14!

### Summary of the Pipeline

<p align="center">
</br>
    <a href><img alt="llvm optimization pipeline" src="../img/llvm-opt-pipeline.svg"> </a>
</p>

## Implementation

### The run_passes Method

Inkwell provides access to the pass manager through `Module::run_passes()`:

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:run_passes}}
```

Key points:

1. **Initialize Native Target**: Required before creating a TargetMachine
2. **Get Target Triple**: The host machine description (e.g., `x86_64-apple-darwin`)
3. **Create TargetMachine**: Needed for target-specific optimizations
4. **run_passes**: Takes a comma-separated list of passes

### Pass Pipeline String

The `passes` argument is a string like:

- `"dce,mem2reg,instcombine"` - Custom pipeline
- `"default<O2>"` - LLVM's standard O2 optimization

### Integration with JIT

We add an optional optimization step to our JIT runner:

```rust,ignore
{{#include ../../../thirdlang/src/codegen.rs:jit_run_optimized}}
```

## Understanding Each Pass

### mem2reg: The Essential Pass

`mem2reg` converts stack-allocated variables to SSA registers. This is critical because our codegen creates allocas for every variable, but registers are much faster than memory.

Before:

```
%x = alloca i64
store i64 42, ptr %x
%val = load i64, ptr %x
```

After:

```
%val = 42
```

### dce: Dead Code Elimination

Removes instructions whose results are never used:

Before:

```
%unused = add i64 %a, %b    ; result never used
%result = mul i64 %c, %d
ret i64 %result
```

After:

```
%result = mul i64 %c, %d
ret i64 %result
```

### instcombine: Instruction Combining

Simplifies patterns:

- `sub i64 %x, 1` becomes `add i64 %x, -1`
- `mul i64 %x, 2` becomes `shl i64 %x, 1` (shift left)
- Constant folding: `add i64 3, 4` becomes `7`

### simplifycfg: Control Flow Simplification

Cleans up the control flow graph:

- Removes empty basic blocks
- Merges blocks with single predecessors
- Simplifies trivial branches

## Using the CLI

```bash
# Run without optimization
thirdlang examples/point.tl

# Run with optimization
thirdlang -O examples/point.tl

# Run with custom passes
thirdlang --passes "mem2reg,dce" examples/point.tl

# Print unoptimized IR
thirdlang --ir examples/point.tl

# Print optimized IR
thirdlang --ir -O examples/point.tl

# Use LLVM's O2 pipeline
thirdlang --passes "default<O2>" examples/point.tl
```

## Testing Optimization

We verify that optimization produces correct results:

```rust,ignore
{{#include ../../../thirdlang/tests/integration_tests.rs:test_optimization_pipeline}}
```

## Optimization Levels

LLVM provides preset pipelines:

| Level | Description |
|-------|-------------|
| `default<O0>` | No optimization (verification only) |
| `default<O1>` | Light optimization |
| `default<O2>` | Standard optimization (recommended) |
| `default<O3>` | Aggressive optimization |

For teaching, we use `dce,mem2reg,instcombine,simplifycfg` to see each pass individually.

## Summary

We added LLVM optimization:

1. Created `run_passes()` method that accepts a pipeline string
2. Integrated optimization into the JIT runner
3. Added `-O` and `--passes` CLI flags

The key insight: our naive codegen produces correct but inefficient IR. LLVM passes transform it into efficient code:

- `mem2reg` - Eliminates stack allocations
- `instcombine` - Merges redundant instructions
- `dce` - Removes unused code

Try `thirdlang --ir examples/counter.tl` vs `thirdlang --ir -O examples/counter.tl` to see the difference!

## Cross-References

- [LLVM Code Generation](codegen_classes.md) - How we generate the initial IR
- [From AST to IR](../03_secondlang/ir.md) - IR concepts from Secondlang
