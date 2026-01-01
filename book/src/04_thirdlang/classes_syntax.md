# Class Syntax and Parsing

Now that we understand [why we want classes](why_classes.md), let us see how to add them to our language. The grammar changes are significant but follow patterns we have seen before in [Secondlang's grammar](../03_secondlang/annotations.md).

If the PEG syntax looks unfamiliar, review the [PEG and pest Syntax](../crash_course.md#peg-and-pest-syntax) section in the Crash Course.

## New Grammar Rules

### Types: Adding Class Types

In Secondlang, types were just `int` or `bool`. Now any class name is also a type:

```text
{{#include ../../../thirdlang/src/grammar.pest:types}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/grammar.pest">thirdlang/src/grammar.pest</a>

The `ClassType` rule matches any identifier. When we see `Point` in a type position, we now parse it as a class type. The type checker (later) verifies that a class with that name actually exists.

### Class Definition

Here is the grammar for class definitions:

```text
{{#include ../../../thirdlang/src/grammar.pest:class_def}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/grammar.pest">thirdlang/src/grammar.pest</a>

Let us break this down:

- **ClassDef** - The whole class: `class Name { body }`
- **ClassBody** - Zero or more fields and methods
- **FieldDef** - A field declaration: `name: type`
- **MethodDef** - A method: `def name(self, params) -> type { body }`
- **SelfParam** - The literal `self` keyword
- **MethodParams** - Additional parameters after `self`

The key difference from regular functions: methods *must* have `self` as their first parameter.

### Object Creation with New

```text
{{#include ../../../thirdlang/src/grammar.pest:new_expr}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/grammar.pest">thirdlang/src/grammar.pest</a>

The `new` keyword followed by a class name and constructor arguments. This allocates memory and calls `__init__`.

### Object Deletion

```text
{{#include ../../../thirdlang/src/grammar.pest:delete}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/grammar.pest">thirdlang/src/grammar.pest</a>

The `delete` statement takes an expression (which should evaluate to an object) and frees its memory.

### Field Access and Method Calls

```text
{{#include ../../../thirdlang/src/grammar.pest:postfix}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/grammar.pest">thirdlang/src/grammar.pest</a>

Postfix operations handle:

- **Field access**: `obj.field` - read a field
- **Method calls**: `obj.method(args)` - call a method

The `PostfixOp*` means zero or more, allowing chaining: `a.b.c.method()`.

## The Typed AST

### Top-Level Items

Programs now contain both classes and statements:

```rust,ignore
{{#include ../../../thirdlang/src/ast.rs:top_level}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/ast.rs">thirdlang/src/ast.rs</a>

A program is now `Vec<TopLevel>` instead of `Vec<Stmt>`. Each top-level item is either a class definition or a statement.

### Class Definition AST

```rust,ignore
{{#include ../../../thirdlang/src/ast.rs:class_def}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/ast.rs">thirdlang/src/ast.rs</a>

The `ClassDef` struct contains:

- **name** - The class name (e.g., `"Point"`)
- **fields** - List of field definitions
- **methods** - List of method definitions

Each `FieldDef` has a name and type. Each `MethodDef` is like a function but with `self` implied.

### Statements with Delete

```rust,ignore
{{#include ../../../thirdlang/src/ast.rs:stmt}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/ast.rs">thirdlang/src/ast.rs</a>

The `Stmt` enum gains a `Delete` variant for the `delete` statement.

### Assignment Targets

Assignments can now target fields:

```rust,ignore
{{#include ../../../thirdlang/src/ast.rs:assign_target}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/ast.rs">thirdlang/src/ast.rs</a>

This allows both:

- `x = 10` - assign to variable
- `self.x = 10` - assign to field

### New Expressions

```rust,ignore
{{#include ../../../thirdlang/src/ast.rs:expr}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/ast.rs">thirdlang/src/ast.rs</a>

The `Expr` enum gains several new variants:

- **SelfRef** - The `self` keyword
- **New** - Object creation: `new Point(1, 2)`
- **FieldAccess** - Reading a field: `obj.x`
- **MethodCall** - Calling a method: `obj.method(args)`

## Parser Implementation

Here is how we parse classes in Rust:

```rust,ignore
{{#include ../../../thirdlang/src/parser.rs:parse_class}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/parser.rs">thirdlang/src/parser.rs</a>

The parser:

1. Extracts the class name from the first child
2. Iterates over the class body, sorting items into fields and methods
3. For each method, skips the `self` parameter (it is implicit)
4. Returns a `ClassDef` with all collected data

## Parsing Example

Let us trace through parsing this class:

```
class Point {
    x: int
    y: int

    def __init__(self, x: int, y: int) {
        self.x = x
        self.y = y
    }

    def get_x(self) -> int {
        return self.x
    }
}
```

### Step 1: Match ClassDef

The parser sees `class`, then:

1. **Identifier**: `Point`
2. **`{`**: Start of body
3. **ClassBody**: Fields and methods
4. **`}`**: End of body

### Step 2: Parse ClassBody

Inside the body:

1. **FieldDef**: `x: int` → `FieldDef { name: "x", ty: Type::Int }`
2. **FieldDef**: `y: int` → `FieldDef { name: "y", ty: Type::Int }`
3. **MethodDef**: `def __init__(...)`
4. **MethodDef**: `def get_x(...)`

### Step 3: Parse MethodDef

For `def __init__(self, x: int, y: int)`:

1. **`def`**: Method keyword
2. **Identifier**: `__init__`
3. **`(`**: Start parameters
4. **SelfParam**: `self`
5. **MethodParams**: `, x: int, y: int`
6. **`)`**: End parameters
7. **No ReturnType**: Defaults to `Unit`
8. **Block**: `{ self.x = x; self.y = y }`

### Step 4: Parse Method Body

Inside `{ self.x = x; self.y = y }`:

1. **Assignment**: `self.x = x`
   - Target: `AssignTarget::Field { object: SelfRef, field: "x" }`
   - Value: `Expr::Var("x")`
2. **Assignment**: `self.y = y`
   - Similar structure

### Final AST

```rust,ignore
TopLevel::Class(ClassDef {
    name: "Point".to_string(),
    fields: vec![
        FieldDef { name: "x".to_string(), ty: Type::Int },
        FieldDef { name: "y".to_string(), ty: Type::Int },
    ],
    methods: vec![
        MethodDef {
            name: "__init__".to_string(),
            params: vec![
                // Note: 'self' is NOT stored in params - it's implicit
                // Only the parameters AFTER self are stored
                ("x".to_string(), Type::Int),
                ("y".to_string(), Type::Int),
            ],
            return_type: Type::Unit,
            body: vec![/* assignments */],
        },
        MethodDef {
            name: "get_x".to_string(),
            params: vec![],  // No params after self
            return_type: Type::Int,
            body: vec![/* return self.x */],
        },
    ],
})
```

Note: The `self` parameter is implicit in methods - it is not stored in the `params` list. The type checker knows every method receives `self` of the class type.

## Type Information for Classes

Classes need metadata for type checking. We store this in `ClassInfo`:

```rust,ignore
{{#include ../../../thirdlang/src/types.rs:class_info}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/types.rs">thirdlang/src/types.rs</a>

The `ClassInfo` struct tracks:

- **name** - Class name
- **fields** - Map from field name to type
- **field_order** - Order of fields (for LLVM struct layout)
- **methods** - Map from method name to `MethodInfo`
- **has_destructor** - Whether `__del__` exists

The `MethodInfo` struct tracks each method's signature.

## The Type Enum

Our type system now includes class types:

```rust,ignore
{{#include ../../../thirdlang/src/types.rs:type_enum}}
```

<a class="filename" href="https://github.com/ehsanmok/create-your-own-lang-with-rust/blob/master/thirdlang/src/types.rs">thirdlang/src/types.rs</a>

The new `Class(String)` variant holds the class name. When we see `Point` as a type, we create `Type::Class("Point".to_string())`.

## Comparison with Secondlang

| Aspect | Secondlang | Thirdlang |
|--------|------------|-----------|
| Types | `int`, `bool` | `int`, `bool`, `ClassName` |
| Top-level | `Vec<Stmt>` | `Vec<TopLevel>` |
| Functions only | Yes | Functions + Methods |
| Field access | No | `obj.field` |
| Method calls | No | `obj.method()` |
| New expressions | No | `new Class(args)` |
| Delete statement | No | `delete obj` |

<div class="checkpoint">

At this point, you should be able to:

- Parse `class Point { x: int }` without errors
- Parse methods with `self` parameter
- Parse `new Point(1, 2)` expressions

</div>

<div class="related-topics">
<strong>Related Topics</strong>

- [Secondlang Grammar](../03_secondlang/annotations.md) - The grammar we extended
- [PEG Syntax](../crash_course.md#peg-and-pest-syntax) - Grammar rule reference
- [Constructors](constructors.md) - How `__init__` works
- [LLVM Codegen](codegen_classes.md) - How classes compile to IR

</div>

In the next chapter, we look at [constructors and object creation](constructors.md).
