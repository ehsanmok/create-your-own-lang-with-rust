## Just-In-Time (JIT) Compiler with LLVM

JIT compilation is a combination of Ahead-Of-Time (AOT) compilation and interpretation. As we saw previously, our `Calc` interpreter evaluates AST to values (actual integer `i32` values) but a JIT compiler differs from an interpreter in what it outputs. Intuitively, JIT outputs are like AOT outputs but generated at runtime when traversing the AST.

### LLVM

[LLVM](https://en.wikipedia.org/wiki/LLVM) (which is *not* an acronym) is a mature compiler backend (code generator) infrastructure powering many languages such as [Clang](https://clang.llvm.org/), [Rust](https://www.rust-lang.org/), [Swift](https://swift.org/), etc. It has its own IR and Virtual Machine Bytecode abstracting away the underlying platform-specific differences.

We will use [inkwell](https://github.com/TheDan64/inkwell) which provides a safe Rust wrapper around LLVM.

### Alternatives

Other code generators that you can use (see the [exercises](./exercise.md)) in this book (not at mature as LLVM) are [cratelift-simpljit](https://docs.rs/cranelift-simplejit/0.64.0/cranelift_simplejit/index.html) and [gcc-jit](https://github.com/swgillespie/gccjit.rs).
