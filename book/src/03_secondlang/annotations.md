# Type Annotations

Now that we understand [why we want types](./why_types.md), let us see *how* to add them to our language. The good news: the grammar changes are minimal. Most of the work happens in the [type checker](./inference.md).

## What Changes in the Grammar?

Remember [Firstlang's](../02_firstlang/syntax.md) function definition:

```text
Function = { "def" ~ Identifier ~ "(" ~ Params? ~ ")" ~ Block }
Params = _{ Identifier ~ ("," ~ Identifier)* }
```

A function like `def add(a, b) { ... }` has parameters `a` and `b`, but we do not know their types.

In Secondlang, we add [type annotations](https://en.wikipedia.org/wiki/Type_signature):

```text
Function = { "def" ~ Identifier ~ "(" ~ TypedParams? ~ ")" ~ ReturnType? ~ Block }
TypedParams = _{ TypedParam ~ ("," ~ TypedParam)* }
TypedParam = { Identifier ~ ":" ~ Type }
ReturnType = { "->" ~ Type }
```

Now we write `def add(a: int, b: int) -> int { ... }`. The changes are:

1. Parameters become `name: type` instead of just `name`
2. Functions can have a return type `-> type`

This syntax is similar to [type hints in Python](https://peps.python.org/pep-0484/), [TypeScript](https://www.typescriptlang.org/), and Rust.

We also need to define what a `Type` is:

```text
Type = { IntType | BoolType }
IntType = { "int" }
BoolType = { "bool" }
```

That is it. Just these few lines enable static typing in our language.

## The Complete Grammar

Here is the full Secondlang grammar. Notice that *most of it is identical to Firstlang*. Only the rules involving declarations changed:

```text
{{#include ../../../secondlang/src/grammar.pest}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/grammar.pest">secondlang/src/grammar.pest</a>

Take a moment to compare this with [Firstlang's grammar](../02_firstlang/syntax.md). The expression rules (`Expr`, `Comparison`, `Additive`, etc.) are exactly the same. The only differences are in `Function`, `TypedParam`, `ReturnType`, `Type`, and `Assignment` (which now optionally accepts a type annotation). If the pest syntax looks unfamiliar, review the [PEG and pest Syntax](../crash_course.md#peg-and-pest-syntax) section.

## The Typed AST

In Firstlang, expressions were simple:

```rust,ignore
enum Expr {
    Int(i64),
    Bool(bool),
    Var(String),
    Binary { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
    // ...
}
```

In Secondlang, every expression carries its type. This is sometimes called a **decorated AST** or **[annotated AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree)**:

```rust,ignore
{{#include ../../../secondlang/src/ast.rs:typed_expr}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/ast.rs">secondlang/src/ast.rs</a>

The `TypedExpr` struct wraps an expression with its type. Let us understand the two constructors:

- `TypedExpr::new(expr, ty)` - creates an expression with a *known* type
- `TypedExpr::unknown(expr)` - creates an expression with `Type::Unknown`

When we parse `1 + 2`, we do not yet know the type of the result. So we create:

```rust,ignore
TypedExpr::unknown(Expr::Binary {
    op: BinaryOp::Add,
    left: Box::new(TypedExpr::unknown(Expr::Int(1))),
    right: Box::new(TypedExpr::unknown(Expr::Int(2))),
})
```

All the types are `Unknown`. The type checker (next chapter) will fill them in.

## Statements with Types

Statements also include type information:

```rust,ignore
{{#include ../../../secondlang/src/ast.rs:stmt}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/ast.rs">secondlang/src/ast.rs</a>

Notice the differences from Firstlang:

- `Function` now has `params: Vec<(String, Type)>` instead of `params: Vec<String>`
- `Function` now has a `return_type: Type` field
- `Assignment` now has an optional `type_ann: Option<Type>` for explicit type annotations

## The Expression Enum

```rust,ignore
{{#include ../../../secondlang/src/ast.rs:expr}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/secondlang/src/ast.rs">secondlang/src/ast.rs</a>

This looks almost identical to Firstlang. The key difference is that child expressions are `Box<TypedExpr>` instead of `Box<Expr>`. Every sub-expression carries its type.

## Parsing Example

Let us trace through parsing `def add(a: int, b: int) -> int { return a + b }`:

1. **`Function` rule matches** - we have `def`, an identifier, parameters, return type, and a block

2. **Parse function name** - `Identifier` matches `add`

3. **Parse parameters** - `TypedParams` matches `a: int, b: int`
   - First `TypedParam`: `a: int` → `("a", Type::Int)`
   - Second `TypedParam`: `b: int` → `("b", Type::Int)`

4. **Parse return type** - `ReturnType` matches `-> int` → `Type::Int`

5. **Parse body** - `Block` matches `{ return a + b }`
   - `Return` statement with expression `a + b`
   - The expression is parsed as `TypedExpr::unknown(Expr::Binary { ... })`

The final AST looks like:

```rust,ignore
Stmt::Function {
    name: "add".to_string(),
    params: vec![
        ("a".to_string(), Type::Int),
        ("b".to_string(), Type::Int),
    ],
    return_type: Type::Int,
    body: vec![
        Stmt::Return(TypedExpr {
            expr: Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(TypedExpr { expr: Expr::Var("a"), ty: Type::Unknown }),
                right: Box::new(TypedExpr { expr: Expr::Var("b"), ty: Type::Unknown }),
            },
            ty: Type::Unknown,  // <-- filled in by type checker
        })
    ],
}
```

Notice that all expression types are `Unknown`. The type checker will walk through this AST and fill in `Type::Int` everywhere.

In the next chapter, we will implement the [type checker](./inference.md).
