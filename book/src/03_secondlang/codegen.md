# LLVM Code Generation

Now we write the code that turns our typed AST into LLVM IR. We use the **inkwell** library, which provides safe Rust bindings to LLVM.

## The CodeGen Structure

Our code generator keeps track of several things:

```rust,ignore
{{#include ../../../secondlang/src/codegen.rs:codegen_struct}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/codegen.rs">secondlang/src/codegen.rs</a>

Let us understand each field:

- **context** - The LLVM context. All LLVM objects belong to a context. Think of it as the "workspace" for LLVM.
- **module** - A container for functions. Think of it as a single source file or compilation unit.
- **builder** - The tool we use to create IR instructions. We position it in a basic block and it adds instructions there.
- **variables** - Maps variable names to their stack locations (pointers from `alloca`). When we see `x`, we look it up here to find where it lives in memory.
- **functions** - Maps function names to their LLVM function objects. Needed so we can call functions by name.
- **current_fn** - The function we are currently compiling. Needed to create new basic blocks for conditionals and loops.

## The Compilation Process

Compilation happens in three passes:

```rust,ignore
{{#include ../../../secondlang/src/codegen.rs:compile}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/codegen.rs">secondlang/src/codegen.rs</a>

**Pass 1: Declare all functions**

Before we can compile function bodies, we need to know about all functions. Why? Because `foo` might call `bar`, and `bar` might call `foo`. We declare all functions first so calls can find their targets.

A function *declaration* tells LLVM "there is a function with this name and signature" but does not include the body yet.

**Pass 2: Compile function bodies**

Now we go through each function and generate its body - the actual instructions.

**Pass 3: Create the `__main` wrapper**

If there is a top-level expression (like `fib(10)`), we wrap it in a `__main` function. This gives the JIT an entry point to call.

**Verify the module**

Finally, we ask LLVM to verify that our IR is well-formed. This catches bugs in our code generator.

## Compiling Expressions

The heart of code generation is `compile_expr`. It takes a typed expression and produces an LLVM value:

```rust,ignore
{{#include ../../../secondlang/src/codegen.rs:compile_expr}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/codegen.rs">secondlang/src/codegen.rs</a>

Let us walk through the important cases:

### Integers and Booleans

```rust,ignore
Expr::Int(n) => Ok(self.context.i64_type().const_int(*n as u64, false)),
Expr::Bool(b) => Ok(self.context.bool_type().const_int(*b as u64, false)),
```

Constants are simple. We create a constant integer value of the right type. The `false` argument means the value is unsigned (we use signed arithmetic in operations).

### Variables: The Alloca/Load Pattern

```rust,ignore
Expr::Var(name) => {
    let ptr = self.variables.get(name)
        .ok_or_else(|| format!("Undefined variable: {}", name))?;
    let val = self.builder.build_load(self.context.i64_type(), *ptr, name)?;
    Ok(val.into_int_value())
}
```

Variables are stored on the stack using the **alloca/load/store pattern** we discussed in the [IR chapter](./ir.md#why-all-the-loading-and-storing):

1. When we declare a variable, we use `alloca` to reserve stack space and store the pointer
2. When we read a variable, we `load` from that pointer
3. When we write a variable, we `store` to that pointer

This pattern handles mutable variables naturally and LLVM optimizes it away when possible (promoting stack slots to registers).

### Binary Operations

```rust,ignore
Expr::Binary { op, left, right } => {
    let l = self.compile_expr(left)?;  // compile left operand
    let r = self.compile_expr(right)?; // compile right operand

    match op {
        BinaryOp::Add => Ok(self.builder.build_int_add(l, r, "add")?),
        BinaryOp::Sub => Ok(self.builder.build_int_sub(l, r, "sub")?),
        // ... etc
    }
}
```

We recursively compile left and right operands, then emit the appropriate instruction. The `"add"` string is a name for the result (helps when reading the IR).

### Function Calls

```rust,ignore
Expr::Call { name, args } => {
    let function = self.functions.get(name)
        .ok_or_else(|| format!("Undefined function: {}", name))?;

    let arg_values: Vec<_> = args.iter()
        .map(|a| self.compile_expr(a).map(|v| v.into()))
        .collect::<Result<_, _>>()?;

    let call = self.builder.build_call(*function, &arg_values, "call")?;
    Ok(call.try_as_basic_value().unwrap_basic().into_int_value())
}
```

We look up the function, compile each argument, then emit a `call` instruction.

The `try_as_basic_value().unwrap_basic()` deserves explanation. In LLVM, function calls can return either:

- A "basic value" (like an integer or pointer) that we can use
- Nothing (for void functions)

`try_as_basic_value()` returns an enum with both possibilities. Since our functions always return `int`, we know we have a basic value and can safely unwrap it. The `into_int_value()` converts it to the specific integer type we need.

### Conditionals

Conditionals need multiple basic blocks:

```rust,ignore
Expr::If { cond, then_branch, else_branch } => {
    // Compile condition and convert to i1 for branching
    let cond_val = self.compile_expr(cond)?;
    let cond_bool = self.builder.build_int_truncate(cond_val, self.context.bool_type(), "cond")?;

    // Create basic blocks for then, else, and merge
    let function = self.current_fn.unwrap();
    let then_bb = self.context.append_basic_block(function, "then");
    let else_bb = self.context.append_basic_block(function, "else");
    let merge_bb = self.context.append_basic_block(function, "merge");

    // Branch based on condition
    self.builder.build_conditional_branch(cond_bool, then_bb, else_bb)?;

    // Compile then branch
    self.builder.position_at_end(then_bb);
    // ... compile statements ...
    self.builder.build_unconditional_branch(merge_bb)?;

    // Compile else branch
    self.builder.position_at_end(else_bb);
    // ... compile statements ...
    self.builder.build_unconditional_branch(merge_bb)?;

    // Continue at merge point
    self.builder.position_at_end(merge_bb);
    // ... use phi node to select result ...
}
```

The key insight: we create separate basic blocks, compile each branch by positioning the builder at the right block, then use a **phi node** to merge the results.

Remember from the [IR chapter](./ir.md#phi-nodes-merging-values-from-different-paths): a phi node selects a value based on which block we came from. If the condition was true and we came from `then_bb`, use the then-result. Otherwise use the else-result.

## JIT Execution

Finally, we can run our compiled code:

```rust,ignore
{{#include ../../../secondlang/src/codegen.rs:jit_run}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/codegen.rs">secondlang/src/codegen.rs</a>

This function:

1. Creates a code generator
2. Compiles the program to IR
3. Creates a JIT execution engine
4. Gets a pointer to `__main` (our entry point)
5. Calls it and returns the result

The JIT engine compiles our IR to native machine code on the fly, then executes it. This is much faster than interpretation because we are running actual machine code, not walking a tree.

The `unsafe` block is required because we are calling raw machine code. We have to trust that our code generator produced valid code.

## Putting It All Together

Here is what happens when you run `cargo run -- examples/fibonacci.sl`:

1. **Parse** the source file → Typed AST (with `Unknown` types)
2. **Type check** → Typed AST (all types resolved)
3. **Optimize** (optional) → Simplified AST
4. **Compile** → LLVM IR
5. **JIT** → Native machine code
6. **Execute** → Result

All in a fraction of a second.

## Testing

```bash
rustup run nightly cargo test compile
```

<div class="checkpoint">

At this point, you should be able to:

- Run `rustup run nightly cargo run -- --ir examples/fibonacci.sl` and see LLVM IR output
- See functions like `@fib` in the IR
- Verify the IR compiles without errors

</div>

<div class="related-topics">
<strong>Related Topics</strong>

- [From AST to IR](./ir.md) - Understanding LLVM IR syntax
- [Type Inference](./inference.md) - How types flow into codegen
- [JIT Compiling Fibonacci](./jit_fibonacci.md) - Running the compiled code
- [Thirdlang Codegen](../04_thirdlang/codegen_classes.md) - Extending codegen for classes

</div>

In the next chapter, we put it all together and run Fibonacci.
