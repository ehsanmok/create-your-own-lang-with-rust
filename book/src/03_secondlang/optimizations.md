# AST Optimizations

Before we generate [LLVM code](./codegen.md), we can make the AST *better*. By "better", we mean simpler expressions that do the same thing. For example, `1 + 2 * 3` can become just `7`. This is called **[optimization](https://en.wikipedia.org/wiki/Optimizing_compiler)**.

In this chapter, we introduce the **[visitor pattern](https://en.wikipedia.org/wiki/Visitor_pattern)**, a classic technique for walking through and transforming ASTs.

## The Visitor Pattern

Imagine we want to add a new operation on our AST, like "pretty print" or "count variables" or "simplify expressions". Without a good design, we would need to modify every AST node type:

```rust,ignore
// Bad: scattered across many files
impl Expr {
    fn pretty_print(&self) { ... }
    fn count_variables(&self) { ... }
    fn simplify(&self) { ... }
}
```

Every time we add a new operation, we touch every node. That gets messy.

The visitor pattern flips this around. Instead of adding methods to nodes, we create separate *visitor* objects:

```rust,ignore
struct PrettyPrinter;
impl ExprVisitor for PrettyPrinter { ... }

struct ConstantFolder;
impl ExprVisitor for ConstantFolder { ... }
```

Each visitor is self-contained. Adding a new operation means adding a new visitor, not modifying existing code. This follows the [open/closed principle](https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle): open for extension, closed for modification.

## The ExprVisitor Trait

Our visitor trait provides a hook for each expression type:

```rust,ignore
{{#include ../../../secondlang/src/visitor.rs:expr_visitor}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/visitor.rs">secondlang/src/visitor.rs</a>

Let us understand how this works:

1. `visit_expr` is the entry point. It looks at the expression type and calls the appropriate visitor method.

2. Each `visit_*` method has a *default implementation* that just recurses into children. For example, `visit_binary` visits left and right, then rebuilds the binary expression.

3. To customize behavior, we override the methods we care about. For constant folding, we override `visit_binary` to check if both operands are constants.

This is sometimes called a **tree walk** or **[tree traversal](https://en.wikipedia.org/wiki/Tree_traversal)**.

## Optimization 1: Constant Folding

**[Constant folding](https://en.wikipedia.org/wiki/Constant_folding)** evaluates expressions where all values are known at compile time:

$$
\begin{aligned}
1 + 2 \times 3 &\Rightarrow 7 \\\\
5 < 10 &\Rightarrow \text{true} \\\\
-(-42) &\Rightarrow 42
\end{aligned}
$$

Why wait until runtime to compute `1 + 2` when we can do it now?

Here is the pseudocode:

```text
FUNCTION constant_fold(expr):
   case expr of:
   | Binary(op, left, right):
       folded_left = constant_fold(left)
       folded_right = constant_fold(right)

       if both folded_left and folded_right are constants:
           compute the result at compile time
           return the constant result
       else:
           return Binary(op, folded_left, folded_right)

   | Unary(op, inner):
       folded = constant_fold(inner)
       if folded is a constant:
           compute result
           return constant
       else:
           return Unary(op, folded)

   | other:
       return expr  # cannot fold
```

And the implementation:

```rust,ignore
{{#include ../../../secondlang/src/visitor.rs:constant_folder}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/visitor.rs">secondlang/src/visitor.rs</a>

Let us trace through `1 + 2 * 3`:

1. Visit outer `+` expression
2. Recursively visit left (`1`) → returns `Int(1)`
3. Recursively visit right (`2 * 3`) → visits `*`, finds `Int(2)` and `Int(3)`, returns `Int(6)`
4. Back at `+`: left is `Int(1)`, right is `Int(6)` → return `Int(7)`

The whole expression becomes just `7`.

## Optimization 2: Algebraic Simplification

**[Algebraic simplification](https://en.wikipedia.org/wiki/Algebraic_simplification)** (also called **strength reduction**) applies mathematical identities:

| Expression | Simplified | Identity Applied |
|------------|------------|------------------|
| `x + 0` | `x` | Additive identity |
| `x - 0` | `x` | Additive identity |
| `x * 0` | `0` | Zero property |
| `x * 1` | `x` | Multiplicative identity |
| `x / 1` | `x` | Multiplicative identity |
| `0 + x` | `x` | Commutativity + identity |
| `1 * x` | `x` | Commutativity + identity |

These transformations are always valid and can save runtime computation.

Pseudocode:

```text
FUNCTION simplify(expr):
   case expr of:
   | Binary(Add, x, Int(0)): return simplify(x)
   | Binary(Add, Int(0), x): return simplify(x)
   | Binary(Mul, x, Int(1)): return simplify(x)
   | Binary(Mul, Int(1), x): return simplify(x)
   | Binary(Mul, _, Int(0)): return Int(0)
   | Binary(Mul, Int(0), _): return Int(0)
   | ... # other cases
   | Binary(op, left, right):
       return Binary(op, simplify(left), simplify(right))
   | other:
       return expr
```

Implementation:

```rust,ignore
{{#include ../../../secondlang/src/visitor.rs:algebraic_simplifier}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/visitor.rs">secondlang/src/visitor.rs</a>

## Chaining Optimizations

Multiple optimization passes can be chained. This is called a **[pass pipeline](https://en.wikipedia.org/wiki/Multi-pass_compiler)**:

```rust,ignore
pub fn optimize_program(program: &Program) -> Program {
    // First: fold constants
    let program = ConstantFolder::fold_program(&program);
    // Then: simplify algebra
    AlgebraicSimplifier::simplify_program(&program)
}
```

Consider `x * (1 + 0)`:

1. After constant folding: `x * 1` (because `1 + 0 = 1`)
2. After algebraic simplification: `x` (because `x * 1 = x`)

Two passes, significant simplification. The order matters - constant folding first creates opportunities for algebraic simplification.

## Why Bother?

You might wonder: "LLVM will optimize this anyway. Why do it ourselves?"

Good question. LLVM *will* do these optimizations. But:

1. **Learning**: Implementing optimizations helps you understand how compilers work. These are the same techniques used in production compilers.

2. **Simplicity**: Simpler AST means simpler code generation. Less can go wrong.

3. **Debug output**: When you print the AST for debugging, optimized code is easier to read.

4. **Specialized optimizations**: You might know things about your language that LLVM does not. Custom optimizations can exploit that knowledge.

5. **Compile time**: Simpler AST means less work for LLVM, which means faster compilation.

## Other Common Optimizations

Production compilers do many more optimizations:

| Optimization | What it does |
|--------------|--------------|
| [Dead code elimination](https://en.wikipedia.org/wiki/Dead_code_elimination) | Remove unreachable code |
| [Common subexpression elimination](https://en.wikipedia.org/wiki/Common_subexpression_elimination) | Compute `x * y` once if used twice |
| [Loop unrolling](https://en.wikipedia.org/wiki/Loop_unrolling) | Replace loops with repeated code |
| [Inlining](https://en.wikipedia.org/wiki/Inline_expansion) | Replace function calls with function bodies |
| [Tail call optimization](https://en.wikipedia.org/wiki/Tail_call) | Turn tail recursion into loops |

We leave these as exercises. The visitor pattern makes adding new optimizations straightforward.

## Using the Optimizations

Enable optimizations with the `-O` flag:

```bash
# Without optimization
rustup run nightly cargo run -- --ir examples/fibonacci.sl

# With optimization
rustup run nightly cargo run -- --ir -O examples/fibonacci.sl
```

## Testing

```bash
rustup run nightly cargo test
```

In the next chapter, we look at what [LLVM IR](./ir.md) looks like before we start generating it.
