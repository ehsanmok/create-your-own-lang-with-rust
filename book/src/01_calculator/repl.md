## Read-Eval-Print Loop (REPL)

REPL as its name implies, loops through every line of input and compile it. We use [rustyline crate](https://github.com/kkawakam/rustyline) to create our REPL. We can optionally choose to interpret or JIT each line of input as follow

```rust,no_run,noplaypen
{{#include ../../../calculator/src/bin/repl.rs:repl}}
```
<span class="filename">Filename: calculator/src/bin/repl.rs</span>

We can either use interpreter, JIT compiler or VM interpreter in our [calculator](../../../calculator) with passing them as flags. Go ahead and run them one by one

```
cargo run --bin repl --features jit
// OR
cargo run --bin repl --features interpreter
// OR
cargo run --bin repl --features vm
```

In either of them, you should see the prompt like

```text
Calculator prompt. Expressions are line evaluated.
>>>
```

waiting for your inputs. Test it our with `1 + 2` examples and `CTRL-C` with break out of the REPL. You can see the different paths of compilation in debug mode. For example with `--features jit`, you will see

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

Checkout the other cases to better see their differences.

## Conclusion

This concludes our [Calculator](./calc_intro.md) chapter. We used the simplicity of our `Calc` language to cover a lot of topics.

Thanks for following along and reading up this far!

Stay tuned for the next chapter where we gradually work our way up to create a statically typed, JIT compiled language named *Jeslang*.
