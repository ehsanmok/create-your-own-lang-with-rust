## Read-Eval-Print Loop (REPL)

REPL (as its name implies) loops through every line of the input and compiles it. We use [rustyline](https://github.com/kkawakam/rustyline) crate to create our REPL. For each line of input, we can optionally choose to

* directly interpret the AST
* JIT the AST
* compile to our bytecode VM and interpret it

```rust,no_run,noplaypen
{{#include ../../../calculator/src/bin/repl.rs:repl}}
```
<span class="filename">Filename: calculator/src/bin/repl.rs</span>

Now, we can use run the REPL and choose different compilation path

```
cargo run --bin repl --features jit
// OR
cargo run --bin repl --features interpreter
// OR
cargo run --bin repl --features vm
```

In any of them, you should see the prompt like

```text
Calculator prompt. Expressions are line evaluated.
>>>
```

waiting for your inputs. Here are some sample outputs of different compilation paths in debug mode.

*  with `--features jit`

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
>> (1 + 2) - (8 - 10)
Compiling the source: (1 + 2) - (8 - 10)
[BinaryExpr { op: Minus, lhs: BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }, rhs: BinaryExpr { op: Minus, lhs: Int(8), rhs: Int(10) } }]
Generated LLVM IR: define i32 @jit() {
entry:
  ret i32 5
}

5
>>
CTRL-C
```

* with `--features vm`

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
>> (1 + 2) - (8 - 10)
Compiling the source: (1 + 2) - (8 - 10)
[BinaryExpr { op: Minus, lhs: BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }, rhs: BinaryExpr { op: Minus, lhs: Int(8), rhs: Int(10) } }]
compiling node BinaryExpr { op: Minus, lhs: BinaryExpr { op: Plus, lhs: Int(1), rhs: Int(2) }, rhs: BinaryExpr { op: Minus, lhs: Int(8), rhs: Int(10) } }
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
>>>
CTRL-C
```

## Conclusion

This concludes our [Calculator](./calc_intro.md) chapter. We took advantage of the simplicity of our `Calc` language to touch on a lot of topics.

Thanks for following along and reading up this far!

Stay tuned for the next chapter where we gradually work our way up to create a statically typed language named creatively as **Firstlang** :D
