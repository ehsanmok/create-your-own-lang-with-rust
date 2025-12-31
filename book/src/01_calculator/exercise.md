## Exercises

Congratulations on completing the Calculator! You've learned the fundamentals of language implementation. Now it's time to solidify that knowledge through practice.

These exercises are ordered by difficulty. Try at least the first one to reinforce what you've learned.

### Exercise 1: Multiplication, Division, and Floats (Recommended)

Extend the calculator to support `*`, `/`, and floating-point numbers.

**New grammar rules needed:**

```pest
Operator = { "+" | "-" | "*" | "/" }
Float = @{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }
```

**Challenges:**

- **Operator precedence**: `2 + 3 * 4` should be `14`, not `20`. Multiplication binds tighter than addition.
- **Mixed types**: What happens with `2 + 3.5`? Do you support implicit conversion or require explicit types?
- **Division by zero**: Do you check at parse time, compile time, or runtime?

**Hints:**

- pest has built-in support for precedence climbing. Look at the `@{ ... }` operator in the pest docs.
- For LLVM, use `build_float_add`, `build_float_mul`, etc.
- For the VM, add new opcodes: `OpMul`, `OpDiv`, `OpFloatAdd`, etc.

### Exercise 2: New JIT Backend with Cranelift

Implement a third JIT backend using [Cranelift](https://docs.rs/cranelift-simplejit/).

**Why Cranelift?**

- Written in pure Rust (no LLVM dependency)
- Faster compilation than LLVM
- Used by Wasmtime (WebAssembly runtime)

**Steps:**

1. Add `cranelift-simplejit` to `Cargo.toml`
2. Create `src/compiler/cranelift_jit.rs`
3. Follow the Cranelift tutorial to:
   - Create a JIT module
   - Define a function signature
   - Translate your AST to Cranelift IR
   - Compile and execute

**Resources:**

- [Cranelift docs](https://docs.rs/cranelift-simplejit/)
- [Cranelift tutorial](https://cranelift.readthedocs.io/)

### Exercise 3: GCC JIT Backend

Implement JIT compilation using [gccjit.rs](https://github.com/swgillespie/gccjit.rs), a Rust wrapper around libgccjit.

**Why GCC JIT?**

- Mature optimization infrastructure
- Cross-platform support
- Simpler API than LLVM for simple cases

**Note:** This requires `libgccjit` to be installed on your system:

```bash
# Ubuntu/Debian
sudo apt install libgccjit-10-dev

# macOS
brew install gcc
```

**Challenge:** Compare compilation speed and runtime performance between LLVM, Cranelift, and GCC JIT for computing `(1 + 2) * 3 - 4 / 2` in a loop 1 million times.

---

<p class="checkpoint-inline"><strong>Checkpoint:</strong> After completing Exercise 1, you should be able to compile and run <code>3.14 * 2.0 + 1.5 / 3.0</code> and get <code>6.78</code>.</p>
