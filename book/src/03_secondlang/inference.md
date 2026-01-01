# Type Inference

> Type inference is like filling in a crossword puzzle. Some squares have letters (explicit annotations), others are blank (Unknown). You use the constraints - "this must be 5 letters", "it crosses with CAT" - to fill in the blanks. Type inference uses constraints like "this is added to an int, so it must be int" to fill in Unknown types.

In the [previous chapter](./annotations.md), we saw that the parser creates an AST with many `Type::Unknown` values. The type checker's job is to figure out what those unknown types should be. This process is called **[type inference](https://en.wikipedia.org/wiki/Type_inference)**.

## What Kind of Type Inference?

There are different approaches to type inference:

| Approach | Used By | Polymorphism | Complexity |
|----------|---------|--------------|------------|
| **[Hindley-Milner](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system)** | Haskell, ML, OCaml | Full parametric | High |
| **[Local Type Inference](https://en.wikipedia.org/wiki/Type_inference#Local_type_inference)** | TypeScript, Go, Rust, Swift | Limited | Low |
| **Bidirectional Type Checking** | Scala, Agda | Configurable | Medium |

We use **local type inference** (also called "flow-based" inference). This is simpler than Hindley-Milner but covers the common cases. The key difference:

- **Hindley-Milner**: Can infer polymorphic types like `fn identity<T>(x: T) -> T` without any annotations
- **Local inference**: Requires type annotations at function boundaries; infers types *within* function bodies

Our approach is similar to what TypeScript, Go, and Rust use. It is practical, easy to understand, and sufficient for our language.

## The Algorithm in Pseudocode

Before diving into Rust code, here is the algorithm in pseudocode:

```text
ALGORITHM: Local Type Inference
INPUT: AST with some types marked as Unknown
OUTPUT: AST with all types filled in, or an error

1. COLLECT SIGNATURES:
   for each function definition in program:
       record (function_name -> function_type) in environment

2. TYPECHECK EACH STATEMENT:
   for each statement in program:
       typecheck_statement(statement, environment)

FUNCTION typecheck_statement(stmt, env):
   case stmt of:
   | Assignment(name, value):
       inferred_type = typecheck_expr(value, env)
       if explicit_annotation exists:
           check annotation matches inferred_type
       add (name -> inferred_type) to env

   | Function(name, params, body):
       local_env = copy of env
       for each (param_name, param_type) in params:
           add (param_name -> param_type) to local_env
       for each stmt in body:
           typecheck_statement(stmt, local_env)

   | Return(expr):
       typecheck_expr(expr, env)
       check result matches declared return type

FUNCTION typecheck_expr(expr, env) -> Type:
   case expr of:
   | Int(n):       return Int
   | Bool(b):      return Bool
   | Var(name):    return lookup(name, env)
   | Binary(op, left, right):
       left_type = typecheck_expr(left, env)
       right_type = typecheck_expr(right, env)
       return apply_op_rules(op, left_type, right_type)
   | Call(name, args):
       func_type = lookup(name, env)
       for each (arg, expected_type) in zip(args, func_type.params):
           actual_type = typecheck_expr(arg, env)
           check actual_type matches expected_type
       return func_type.return_type
   | If(cond, then, else):
       check typecheck_expr(cond, env) == Bool
       then_type = typecheck_block(then, env)
       else_type = typecheck_block(else, env)
       return unify(then_type, else_type)
```

The key insight: types **flow forward** from known sources (literals, parameters) through operations into variables.

## The Core Insight

Here is the key idea: types *flow* through expressions. If we know the type of the inputs, we can figure out the type of the output.

Consider `x = 1 + 2`. How does the compiler know `x` is an `int`?

1. `1` is an integer literal → type is `Int`
2. `2` is an integer literal → type is `Int`
3. `+` with two `Int` operands → produces `Int`
4. We are assigning an `Int` to `x` → `x` must be `Int`

The type "flows" from the literals, through the operator, into the variable. No explicit annotation needed.

## Step-by-Step Example

Let us trace through this code in detail:

```rust,ignore
x = 42
y = x * 2 + 10
is_big = y > 50
```

### Step 1: Parse (types are Unknown)

After parsing, the AST looks like this (simplified):

```
Assignment { name: "x", value: Int(42), ty: Unknown }
Assignment { name: "y", value: Binary(Var("x") * Int(2) + Int(10)), ty: Unknown }
Assignment { name: "is_big", value: Binary(Var("y") > Int(50)), ty: Unknown }
```

Every expression has `ty: Unknown`. We do not know the types yet.

### Step 2: Type check first assignment

For `x = 42`:

1. Check the value `42`:
   - It is an `Int` literal
   - Set its type: `Int(42).ty = Int`

2. Infer the variable type:
   - No explicit annotation, so we use the value's type
   - `x` has type `Int`
   - Add to environment: `{ x: Int }`

### Step 3: Type check second assignment

For `y = x * 2 + 10`:

1. Check `x * 2`:
   - Look up `x` in environment → `Int`
   - `2` is an `Int` literal
   - `*` with `Int * Int` → produces `Int`
   - Set type: `(x * 2).ty = Int`

2. Check `(x * 2) + 10`:
   - Left side `(x * 2)` has type `Int` (from step above)
   - Right side `10` is an `Int` literal
   - `+` with `Int + Int` → produces `Int`
   - Set type: `((x * 2) + 10).ty = Int`

3. Infer the variable type:
   - Value has type `Int`
   - `y` has type `Int`
   - Add to environment: `{ x: Int, y: Int }`

### Step 4: Type check third assignment

For `is_big = y > 50`:

1. Check `y > 50`:
   - Look up `y` in environment → `Int`
   - `50` is an `Int` literal
   - `>` with `Int > Int` → produces `Bool` (comparisons return boolean)
   - Set type: `(y > 50).ty = Bool`

2. Infer the variable type:
   - Value has type `Bool`
   - `is_big` has type `Bool`
   - Add to environment: `{ x: Int, y: Int, is_big: Bool }`

### Final Result

All `Unknown` types are now resolved:

```
Assignment { name: "x", value: Int(42), ty: Int }
Assignment { name: "y", value: Binary(...), ty: Int }
Assignment { name: "is_big", value: Binary(...), ty: Bool }
```

The compiler inferred all the types without us writing a single type annotation.

## Typing Rules

The type checker applies these **[typing rules](https://en.wikipedia.org/wiki/Type_rule)**:

| Expression | Rule | Result Type |
|------------|------|-------------|
| `42` | Integer literals are `Int` | `Int` |
| `true`, `false` | Boolean literals are `Bool` | `Bool` |
| `x` (variable) | Look up in [type environment](https://en.wikipedia.org/wiki/Type_system#Type_environments) | `env[x]` |
| `a + b`, `a * b`, etc. | Both operands must be `Int` | `Int` |
| `a < b`, `a > b`, `a == b` | Both operands must be `Int` | `Bool` |
| `!a` | Operand must be `Bool` | `Bool` |
| `-a` | Operand must be `Int` | `Int` |
| `f(args...)` | Args must match function parameter types | Function's return type |
| `if (c) { a } else { b }` | `c` must be `Bool`; `a` and `b` must unify | Unified type of `a` and `b` |

These rules are applied recursively, bottom-up through the expression tree.

## Type Unification

**[Unification](https://en.wikipedia.org/wiki/Unification_(computer_science))** is the process of checking if two types are compatible and finding a common type. This is a key operation in type inference.

Here is the pseudocode:

```text
FUNCTION unify(type1, type2) -> Type or Error:
   if type1 == type2:
       return type1                    # Same types match
   if type1 == Unknown:
       return type2                    # Unknown takes the other type
   if type2 == Unknown:
       return type1                    # Unknown takes the other type
   if type1 is Function and type2 is Function:
       unify each parameter type
       unify return types
       return unified function type
   else:
       return Error("Cannot unify type1 with type2")
```

Our implementation:

```rust,ignore
{{#include ../../../secondlang/src/types.rs:unify}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/types.rs">secondlang/src/types.rs</a>

Let us understand each case:

| Unify | Result | Why |
|-------|--------|-----|
| `Int.unify(Int)` | `Ok(Int)` | Same types match |
| `Bool.unify(Bool)` | `Ok(Bool)` | Same types match |
| `Unknown.unify(Int)` | `Ok(Int)` | Unknown takes on the concrete type |
| `Int.unify(Unknown)` | `Ok(Int)` | Unknown takes on the concrete type |
| `Int.unify(Bool)` | `Err` | Incompatible types cannot unify |

The `Unknown` case is the heart of type inference. When we unify `Unknown` with a concrete type, we *learn* what the unknown type should be.

## The Type Environment

The **type environment** (also called symbol table or context) maps names to types:

```rust,ignore
type TypeEnv = HashMap<String, Type>;
```

The environment is:

- **Extended** when we declare a variable or enter a function (adding new bindings)
- **Queried** when we reference a variable (looking up its type)
- **Scoped** - inner scopes can shadow outer bindings

This is the same concept as the runtime environment in [Firstlang's interpreter](../02_firstlang/variables.md), but storing types instead of values.

## Function Type Inference

Functions are trickier because we need to handle:

1. Parameters (types come from annotations)
2. Local variables (types are inferred)
3. Return value (must match declared return type)

Consider:

```rust,ignore
def compute(a: int, b: int) -> int {
    temp = a + b        # What type is temp?
    doubled = temp * 2  # What type is doubled?
    return doubled + 1
}
```

The type checker:

1. **Adds parameters to environment**: `{ a: Int, b: Int }`

2. **Checks `temp = a + b`**:
   - `a` is `Int`, `b` is `Int`
   - `a + b` is `Int`
   - `temp` is `Int`
   - Environment: `{ a: Int, b: Int, temp: Int }`

3. **Checks `doubled = temp * 2`**:
   - `temp` is `Int`, `2` is `Int`
   - `temp * 2` is `Int`
   - `doubled` is `Int`
   - Environment: `{ a: Int, b: Int, temp: Int, doubled: Int }`

4. **Checks `return doubled + 1`**:
   - `doubled` is `Int`, `1` is `Int`
   - `doubled + 1` is `Int`
   - Declared return type is `Int` - matches

All types are inferred from the parameter types flowing through the expressions.

## The Two-Pass Algorithm

The type checker uses two passes:

```rust,ignore
{{#include ../../../secondlang/src/typeck.rs:typecheck}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/typeck.rs">secondlang/src/typeck.rs</a>

**Pass 1: Collect function signatures**

We scan all function definitions and record their types *before* checking any bodies. Why? Because functions can call each other (mutual recursion):

```rust,ignore
def isEven(n: int) -> bool {
    if (n == 0) { return true }
    else { return isOdd(n - 1) }
}
def isOdd(n: int) -> bool {
    if (n == 0) { return false }
    else { return isEven(n - 1) }
}
```

When checking `isEven`, we need to know the type of `isOdd`. By collecting all signatures first, [mutual recursion](https://en.wikipedia.org/wiki/Mutual_recursion) works.

**Pass 2: Check each statement**

Now we go through each statement, inferring types as we go.

## Type Checking Expressions

Here is the complete `typecheck_expr` function:

```rust,ignore
{{#include ../../../secondlang/src/typeck.rs:typecheck_expr}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/typeck.rs">secondlang/src/typeck.rs</a>

The pattern is always the same:

1. Recursively type check sub-expressions
2. Apply the typing rule for this expression kind
3. Set the type on this expression

## When Inference Fails

Type inference is not magic. It fails when there is not enough information:

```rust,ignore
# This would fail - what type is x?
x = some_function_that_could_return_anything()
```

Or when types conflict:

```rust,ignore
x = 42
x = true  # Error: x is Int, cannot assign Bool
```

We report errors with helpful messages:

```rust,ignore
let result = typecheck("1 + true");
// Error: "Arithmetic operation requires int operands, got int and bool"

let result = typecheck("add(1, true)");
// Error: "Type mismatch: expected int, got bool"
```

## Limitations of Local Inference

Our inference cannot handle some things that Hindley-Milner can:

```rust,ignore
# Hindley-Milner could infer: identity : forall a. a -> a
def identity(x) {
    return x
}

# We require annotations:
def identity(x: int) -> int {
    return x
}
```

For a simple language like Secondlang, this is fine. The annotation burden is low (just function boundaries), and the implementation is much simpler.

## Comparison with Other Systems

| Feature | Secondlang | TypeScript | Haskell |
|---------|------------|------------|---------|
| Variable inference | Yes | Yes | Yes |
| Function param inference | No | Partial | Yes |
| Polymorphism | No | Yes (generics) | Yes (parametric) |
| Bidirectional | No | Yes | Partial |

## Summary

Type inference works by:

1. **Starting with known types**: literals (`42` → Int, `true` → Bool) and annotated parameters
2. **Flowing types through expressions**: operators, function calls, assignments
3. **Recording types in the environment**: so variables can be looked up later
4. **Unifying types**: checking compatibility and resolving `Unknown`
5. **Reporting errors**: when types do not match

The beauty is that most of the time, you only need to annotate function parameters and return types. Everything else is inferred automatically.

## Further Reading

- [Type Inference on Wikipedia](https://en.wikipedia.org/wiki/Type_inference)
- [Hindley-Milner Type System](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system)
- [Unification in Computer Science](https://en.wikipedia.org/wiki/Unification_(computer_science))
- [Types and Programming Languages](https://www.cis.upenn.edu/~bcpierce/tapl/) by Benjamin Pierce - the definitive textbook

## Try It Yourself

Run the inference example:

```bash
rustup run nightly cargo run -- examples/inference.sl
```

This demonstrates all the inference concepts in action.

## Testing

```bash
cargo test typeck
```

<div class="checkpoint">

At this point, you should be able to:

- Compile `x = 5` and have `x` inferred as `int`
- Get an error for `x = 5 + true` (type mismatch)
- Compile functions without annotating local variable types

</div>

<div class="related-topics">
<strong>Related Topics</strong>

- [Type Annotations](./annotations.md) - Where types come from
- [Why Types Matter](./why_types.md) - Motivation for static types
- [Code Generation](./codegen.md) - How types inform IR generation
- [Thirdlang Types](../04_thirdlang/classes_syntax.md) - Extending types for classes

</div>

In the next chapter, we look at [optimizations](./optimizations.md) we can do on the typed AST before generating code.
