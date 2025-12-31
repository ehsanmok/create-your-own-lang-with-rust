## Read-Eval-Print Loop (REPL)

A REPL is an interactive programming environment. You type code, it immediately runs, you see the result. Then you type more code. This tight feedback loop makes REPLs perfect for learning, experimenting, and debugging.

You've probably used REPLs before:

- Python's `>>>` prompt
- Node.js's interactive mode
- Browser developer console

Now we'll build one for our calculator! Even better, we'll be able to switch between three execution backends - seeing how the same code runs through different compilation paths.

### How a REPL Works

The name tells you everything:

1. **Read** - Get a line of input from the user
2. **Eval** - Parse and execute it
3. **Print** - Show the result
4. **Loop** - Go back to step 1

Here's our implementation using [rustyline](https://github.com/kkawakam/rustyline), a library that provides readline-style line editing (arrow keys, history, etc.):

```rust,no_run,noplaypen
{{#include ../../../calculator/src/bin/repl.rs:repl}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/src/bin/repl.rs">calculator/src/bin/repl.rs</a>

The REPL is simple:

1. Create a rustyline editor (handles input, history, etc.)
2. Loop forever, reading lines
3. For each line, compile and execute using the chosen backend
4. Print the result
5. On Ctrl-C or Ctrl-D, exit

### Three Backends, One Interface

The magic is in the feature flags. The same REPL works with three different backends:

```bash
# Interpreter (stable Rust)
# Walks the AST and computes directly
cargo run --bin repl --features interpreter

# Bytecode VM (stable Rust)
# Compiles to bytecode, then interprets that
cargo run --bin repl --features vm

# JIT (requires nightly Rust + LLVM)
# Compiles to native machine code
rustup run nightly cargo run --bin repl --features jit
```

This is powerful for learning! You can compare how the same expression is handled:

- The interpreter shows the AST structure
- The VM shows the bytecode being generated
- The JIT shows the LLVM IR

### JIT Output Example

With `--features jit`, you see the generated LLVM IR:

```text
Calculator prompt. Expressions are line evaluated.
>> 1 + 2
Compiling the source: 1 + 2
[BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }]
Generated LLVM IR: define i32 @jit() {
entry:
  ret i32 3
}

3
```

Notice something interesting: the IR just says `ret i32 3`! LLVM computed `1 + 2 = 3` at compile time and baked the answer directly into the code. This is **constant folding**, one of LLVM's many optimizations.

Let's try a more complex expression:

```text
>> -2 + 5
Compiling the source: -2 + 5
[BinaryExpr { op: Plus, lhs: UnaryExpr { op: Minus, child: Int(2) }, rhs: Int(5) }]
Generated LLVM IR: define i32 @jit() {
entry:
  ret i32 3
}

3
```

Again, LLVM optimized `(-2) + 5` to just `3`. The AST shows the full structure, but the compiled code is minimal.

### VM Output Example

With `--features vm`, you see bytecode generation step by step:

```text
Calculator prompt. Expressions are line evaluated.
>> 1 + 2
Compiling the source: 1 + 2
[BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }]
compiling node BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }
added instructions [1, 0, 0] from opcode OpConstant(0)
added instructions [1, 0, 0, 1, 0, 1] from opcode OpConstant(1)
added instructions [1, 0, 0, 1, 0, 1, 3] from opcode OpAdd
added instructions [1, 0, 0, 1, 0, 1, 3, 2] from opcode OpPop
byte code: Bytecode { instructions: [1, 0, 0, 1, 0, 1, 3, 2], constants: [Int(1), Int(2)] }
3
```

You can see each instruction being added:

1. `OpConstant(0)` - Push constant at index 0 (which is `1`)
2. `OpConstant(1)` - Push constant at index 1 (which is `2`)
3. `OpAdd` - Pop two values, push their sum
4. `OpPop` - Pop and return the result

A more complex expression shows more bytecode:

```text
>> (1 + 2) - (8 - 10)
byte code: Bytecode {
  instructions: [1, 0, 0, 1, 0, 1, 3, 1, 0, 2, 1, 0, 3, 4, 4, 2],
  constants: [Int(1), Int(2), Int(8), Int(10)]
}
5
```

Four constants, multiple operations, all encoded in a flat byte array.

### Why Build a REPL?

Building a REPL teaches you:

1. **The edit-compile-run cycle** - Even simpler than files
2. **Error handling** - What happens when input is invalid?
3. **State management** - In later chapters, we'll maintain variables across lines
4. **Debugging** - Print AST, bytecode, or IR to see what's happening

Professional language implementations always include a REPL. It's one of the most useful tools for both language developers and language users.

## Conclusion

This concludes our [Calculator](./calc_intro.md) chapter. We took advantage of the simplicity of our `Calc` language to cover a lot of ground:

- **Grammar and parsing** - Converting text to structured data
- **AST** - Representing programs as trees
- **Interpretation** - Walking the tree and computing
- **JIT compilation** - Generating native code with LLVM
- **Bytecode VMs** - An intermediate approach
- **REPL** - Interactive programming

Note that our Calculator grammar is intentionally simple. It handles basic cases like negative first numbers (`-1 + 2`) and flexible whitespace, but it doesn't have proper operator precedence. The expression `1 + 2 * 3` might not evaluate as you'd expect! In the next chapter, we'll see how [Firstlang](../02_firstlang/intro.md) builds a more sophisticated grammar with proper operator precedence and multiple expression types.

Thanks for following along! In the next chapter, we'll build **Firstlang** - a dynamically typed language with variables, functions, control flow, and recursion.
