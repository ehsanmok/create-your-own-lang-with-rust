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

| Backend | Description | Rust Version |
|---------|-------------|--------------|
| **Interpreter** | Walks AST directly | Stable |
| **VM** | Compiles to bytecode | Stable |
| **JIT** | Compiles to native code via LLVM | Nightly |

This is powerful for learning! You can compare how the same expression is handled by each backend. Let's run through two examples with all three.

### Interpreter Output Example

The interpreter walks the AST and computes results directly:

```bash
cargo run --bin repl --features interpreter
```

You see the AST structure and direct evaluation:

```text
Calculator prompt. Expressions are line evaluated.
>> 1 + 2
Compiling the source: 1 + 2
[BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }]
3
```

The interpreter is the simplest backend. It parses the input into an AST (`BinaryExpr` with `Plus` operator, left-hand side `Int(1)`, right-hand side `Int(2)`), then walks the tree and computes the result directly.

A more complex expression shows a nested AST:

```text
>> (1 + 2) - (8 - 10)
Compiling the source: (1 + 2) - (8 - 10)
[BinaryExpr { op: Minus, lhs: BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }, rhs: BinaryExpr { op: Minus, lhs: Int(8), rhs: Int(10) } }]
5
```

The outer `BinaryExpr` has `Minus` as its operator, with two inner `BinaryExpr` nodes as children. The interpreter recursively evaluates each subtree: `(1 + 2) = 3`, `(8 - 10) = -2`, then `3 - (-2) = 5`.

### VM Output Example

The VM compiles AST to bytecode, then executes it on a stack machine:

```bash
cargo run --bin repl --no-default-features --features vm
```

You see bytecode generation step by step:

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

Instead of walking the tree directly, the VM compiles the AST to bytecode first. You can see each instruction being added:

1. `OpConstant(0)` - Push constant at index 0 (which is `1`)
2. `OpConstant(1)` - Push constant at index 1 (which is `2`)
3. `OpAdd` - Pop two values, push their sum
4. `OpPop` - Pop and return the result

A more complex expression shows more bytecode instructions being generated:

```text
>> (1 + 2) - (8 - 10)
Compiling the source: (1 + 2) - (8 - 10)
[BinaryExpr { op: Minus, lhs: BinaryExpr { ... }, rhs: BinaryExpr { ... } }]
compiling node BinaryExpr { ... }
added instructions [1, 0, 0] from opcode OpConstant(0)
added instructions [1, 0, 0, 1, 0, 1] from opcode OpConstant(1)
added instructions [1, 0, 0, 1, 0, 1, 3] from opcode OpAdd
added instructions [1, 0, 0, 1, 0, 1, 3, 1, 0, 2] from opcode OpConstant(2)
added instructions [1, 0, 0, 1, 0, 1, 3, 1, 0, 2, 1, 0, 3] from opcode OpConstant(3)
added instructions [1, 0, 0, 1, 0, 1, 3, 1, 0, 2, 1, 0, 3, 4] from opcode OpSub
added instructions [1, 0, 0, 1, 0, 1, 3, 1, 0, 2, 1, 0, 3, 4, 4] from opcode OpSub
added instructions [1, 0, 0, 1, 0, 1, 3, 1, 0, 2, 1, 0, 3, 4, 4, 2] from opcode OpPop
byte code: Bytecode { instructions: [1, 0, 0, 1, 0, 1, 3, 1, 0, 2, 1, 0, 3, 4, 4, 2], constants: [Int(1), Int(2), Int(8), Int(10)] }
5
```

Four constants, multiple operations, all encoded in a flat byte array. The VM then executes this bytecode using a simple stack machine.

### JIT Output Example

The JIT compiles to native machine code via LLVM (requires nightly Rust):

```bash
rustup run nightly cargo run --bin repl --no-default-features --features jit
```

You see the generated LLVM IR:

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

Let's try the same complex expression:

```text
>> (1 + 2) - (8 - 10)
Compiling the source: (1 + 2) - (8 - 10)
[BinaryExpr { op: Minus, lhs: BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }, rhs: BinaryExpr { op: Minus, lhs: Int(8), rhs: Int(10) } }]
Generated LLVM IR: define i32 @jit() {
entry:
  ret i32 5
}

5
```

Again, LLVM optimized the whole expression to just `ret i32 5`. The AST shows the full nested structure, but the compiled native code is minimal - just returning a constant!

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
