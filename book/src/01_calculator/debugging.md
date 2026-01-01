# Debugging Your Language

Before we move on, let's establish debugging techniques you'll use throughout this book. Language implementations have many moving parts - knowing how to inspect each layer saves hours of frustration.

## The Golden Rule

> Your language is a pipeline. Data flows through stages: Source → Tokens → AST → Output. When something breaks, find *which stage* produced the wrong output.

```
Source Code  →  Parser  →  AST  →  Executor  →  Result
     ↓            ↓         ↓         ↓           ↓
   "1+2"      tokens    tree      eval      3 (or error!)
```

Debug by checking each arrow: Is the input to this stage correct? Is the output?

## Print the AST

The AST is your program's structure. When behavior is wrong, print it:

```rust,ignore
// In your code
let ast = parse(source)?;
println!("{:#?}", ast);  // Pretty-print with {:#?}
```

Output:

```
Binary {
    op: Add,
    left: Int(1),
    right: Int(2),
}
```

If this looks wrong, your parser has a bug. If it looks right, the bug is in execution.

## Check Operator Precedence

A common parsing bug: `1 + 2 * 3` parses as `(1 + 2) * 3` instead of `1 + (2 * 3)`.

```rust,ignore
let ast = parse("1 + 2 * 3")?;
println!("{:#?}", ast);

// Should be: Add(1, Mul(2, 3))
// Bug if:    Mul(Add(1, 2), 3)
```

Fix: Check your grammar's precedence rules. In PEG, earlier alternatives have higher precedence.

## Test Small, Test Often

Don't write 100 lines then debug. Test each feature in isolation:

```rust,ignore
#[test]
fn test_addition() {
    assert_eq!(eval("1 + 2"), 3);
}

#[test]
fn test_subtraction() {
    assert_eq!(eval("5 - 3"), 2);
}

#[test]
fn test_combined() {
    assert_eq!(eval("1 + 2 - 3"), 0);
}
```

When a test fails, you know exactly what's broken.

## Use the REPL

The REPL is your best friend for quick experiments:

```
>>> 1 + 2
3
>>> 1 + 2 * 3
7
>>> (1 + 2) * 3
9
```

If a complex expression fails, simplify until you find the minimal failing case.

## When Using LLVM (Later)

When we add LLVM compilation, two more techniques become essential:

### Print the IR

```rust,ignore
// After code generation
codegen.module.print_to_stderr();
```

You'll see LLVM IR:

```llvm
define i64 @add(i64 %a, i64 %b) {
entry:
  %result = add i64 %a, %b
  ret i64 %result
}
```

If this looks wrong, your codegen has a bug.

### Use LLVM's Verifier

```rust,ignore
codegen.module.verify().map_err(|e| {
    eprintln!("LLVM verification failed: {}", e);
    e.to_string()
})?;
```

The verifier catches:

- Missing terminators (every basic block needs `ret` or `br`)
- Type mismatches
- Invalid instructions

Always verify before JIT execution!

## The Debugging Mindset

> Think like a detective. You have a crime (wrong output). You need to find where in the pipeline the crime occurred. Interrogate each stage until you find the culprit.

1. **Reproduce** - Find the smallest input that triggers the bug
2. **Isolate** - Which stage is producing wrong output?
3. **Inspect** - Print the data at that stage
4. **Fix** - Change the code
5. **Verify** - Run your test again

This systematic approach works for any compiler bug.

## Quick Reference

| Problem | Debug Technique |
|---------|-----------------|
| Wrong result | Print AST, check structure |
| Parse error | Simplify input, check grammar |
| Precedence wrong | Print AST, check grammar order |
| LLVM crash | Print IR, run verifier |
| Infinite loop | Add print statements in eval loop |

Now you have the tools. Let's continue building!
