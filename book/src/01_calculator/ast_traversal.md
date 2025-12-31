## AST Traversal Patterns

In the [previous section](./basic_llvm.md), we handcrafted every LLVM instruction for a simple `add` function. That worked, but it was tedious - imagine doing that for every possible expression! We need a systematic way to turn *any* AST into LLVM IR.

The answer is **recursive tree traversal**. We already do this in the interpreter - walk the tree, evaluate each node. For code generation, we walk the tree and *emit instructions* for each node instead of computing values.

Two common patterns help structure this:

* **Builder pattern** - Used here for LLVM IR generation
* **Visitor pattern** - Introduced in [Secondlang Optimizations](../03_secondlang/optimizations.md) for AST transformations

### Builder Pattern

Think of the LLVM builder like a cursor in a text editor. You position it somewhere in your code, then "type" instructions at that position. The builder keeps track of where you are and ensures instructions are added in the right place.

Let's compare our interpreter's recursive evaluation to the new JIT approach.

**Interpreter**: Walk tree, compute values

```rust,no_run,noplaypen
{{#include ../../../calculator/src/compiler/interpreter.rs:interpreter_recursive}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/src/compiler/interpreter.rs">calculator/src/compiler/interpreter.rs</a>

The interpreter looks at each node and returns a computed integer. For `Int(5)`, it returns `5`. For `Binary { op: Add, left: Int(1), right: Int(2) }`, it recursively evaluates both sides and adds them.

**JIT**: Walk tree, emit LLVM instructions

```rust,no_run,noplaypen
{{#include ../../../calculator/src/compiler/jit.rs:jit_recursive_builder}}
```

The structure is identical! We still match on node types and recurse. But instead of computing values directly, we build LLVM instructions:

* **`Int(n)`** - Create an LLVM integer constant. This doesn't "do" anything at compile time - it creates a value that will exist when the code runs.

* **`UnaryExpr`** - For negation, we recursively compile the inner expression (getting an LLVM value), then emit a subtraction from zero. In LLVM, `-x` is typically represented as `0 - x`.

* **`BinaryExpr`** - Recursively compile both sides, then emit the appropriate arithmetic instruction (`build_int_add`, `build_int_sub`). The `"add"` and `"sub"` strings are just names for debugging - they show up when you print the IR.

The key insight: `recursive_builder` returns LLVM *values*, not Rust integers. These values represent computation that will happen when the JIT-compiled code runs.

### Putting It Together

Now we wire up the complete JIT pipeline:

```rust,no_run,noplaypen
{{#include ../../../calculator/src/compiler/jit.rs:jit_ast}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/calculator/src/compiler/jit.rs">calculator/src/compiler/jit.rs</a>

Let's trace through what happens when we JIT `1 + 2`:

1. **Parse** - Turn `"1 + 2"` into AST: `Binary { op: Add, left: Int(1), right: Int(2) }`

2. **Setup LLVM** - Create context, module, builder

3. **Create wrapper function** - We need a function to call, so we create `__jit` with signature `() -> i64`

4. **Compile AST** - `recursive_builder` walks the tree:
   * Compile `Int(1)` → creates i64 constant `1`
   * Compile `Int(2)` → creates i64 constant `2`
   * Compile `Add` → creates `add i64 1, 2` instruction, returns the result

5. **Return result** - `build_return` emits a `ret` instruction with our computed value

6. **JIT compile** - LLVM turns our IR into native machine code

7. **Execute** - Call the function, get `3`

### Why This Matters

The recursive builder pattern scales to any expression, no matter how complex:

```
-((1 + 2) * (3 - 4))
```

This parses to a nested tree, and `recursive_builder` handles each level automatically. Each call compiles its children first, then emits its own instruction using those children as operands.

This is the foundation of every LLVM-based compiler. Rust, Swift, Julia - they all use this pattern (with many more node types and optimizations).

### Testing

```rust,ignore
assert_eq!(Jit::from_source("1 + 2").unwrap(), 3)
```

Run tests locally:

```bash
cargo test jit --tests
```

In the [next section](./vm.md), we'll see an alternative approach: compiling to bytecode for a virtual machine, which trades some speed for portability.
