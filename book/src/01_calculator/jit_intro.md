## Just-In-Time (JIT) Compiler with LLVM

JIT compilation is a combination of Ahead-Of-Time (AOT) compilation and interpretation. As we saw previously, our `Calc` interpreter evaluates AST to values (actual integer `i32` values). JIT compiler differs from interpreter in what it outputs. Intuitively, JIT outputs are like AOT outputs but generated at runtime when walking the AST similar to interpreter.

### LLVM

[LLVM](https://en.wikipedia.org/wiki/LLVM) which stands for **Low-Level-Virtual-Machine** is a mature compiler backend (code generator) infrastructure powering a lot of languages, such as Clang, Rust, Swift etc. It has its own IR and Virtual Machine Bytecode abstracting away the underlying hardware specific differences.

We will use [inkwell](https://github.com/TheDan64/inkwell) which provides a safe Rust wrapper around LLVM. Remember to use the branch according to your installed `llvm-config --version`.

### Alternatives

Other code generators that you can use (see the [exercises](./exercise.md)) in this book (not at mature as LLVM) are [cratelift-simpljit](https://docs.rs/cranelift-simplejit/0.64.0/cranelift_simplejit/index.html) and [gcc-jit](https://github.com/philberty/gccrs).
