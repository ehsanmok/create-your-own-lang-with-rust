# Exercises

These exercises extend the type system and code generation. Each builds on your understanding of static typing and LLVM.

## Exercise 1: Add Float Type

Add `float` type with 64-bit floating point:

```
def area(radius: float) -> float {
    return 3.14159 * radius * radius
}
area(2.0)
```

**Hints:**

- Add `Type::Float` and map to LLVM `f64`
- Parse float literals: `Float = { ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }`
- Use `build_float_add`, `build_float_mul`, etc. in codegen
- Handle type coercion: `int + float` should promote to `float`

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>2.0 * 3.0</code> returns <code>6.0</code></p>

## Exercise 2: Add Modulo Operator

Add the `%` operator:

```
def is_even(n: int) -> bool {
    return n % 2 == 0
}
```

**Hints:**

- Add `BinaryOp::Mod`
- Use `build_int_signed_rem` for signed modulo
- Type rule: `int % int -> int`

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>10 % 3</code> returns <code>1</code></p>

## Exercise 3: Add Short-Circuit Operators

Add `&&` and `||` with short-circuit evaluation:

```
def safe_div(a: int, b: int) -> int {
    if (b != 0 && a / b > 0) {
        return a / b
    }
    return 0
}
```

**Hints:**

- Cannot use simple `build_and` - need control flow
- For `a && b`: evaluate `a`, if false jump to end with `false`, else evaluate `b`
- Generate basic blocks: `eval_left`, `eval_right`, `merge`
- Use phi node to merge results

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>false && (1/0 &gt; 0)</code> should not crash (division never executes)</p>

## Exercise 4: Add Type Aliases

Allow type aliases:

```
type Distance = int
def manhattan(x: Distance, y: Distance) -> Distance {
    return x + y
}
```

**Hints:**

- Store aliases in a `HashMap<String, Type>`
- Resolve aliases during parsing or type checking
- Aliases should be interchangeable with their underlying type

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>Distance</code> and <code>int</code> should be compatible</p>

## Exercise 5: Add Array Type

Add fixed-size arrays:

```
def sum(arr: [int; 3]) -> int {
    return arr[0] + arr[1] + arr[2]
}
```

**Hints:**

- Add `Type::Array { element: Box<Type>, size: usize }`
- LLVM: use `context.i64_type().array_type(size)`
- Access: `build_extract_value` or GEP with indices
- Type check: ensure index is in bounds (statically if possible)

<p class="checkpoint-inline"><strong>Checkpoint:</strong> Array access compiles and runs correctly</p>

## Stretch Goals

1. **String type**: Pointer to null-terminated bytes
2. **Type inference for locals**: `let x = 5` infers `int`
3. **Generic functions**: `def identity<T>(x: T) -> T { return x }`
4. **Struct types**: `struct Point { x: int, y: int }`
5. **Pattern matching on ints**: `match n { 0 => ..., 1 => ..., _ => ... }`

## What You Practiced

- Extending the type system with new types
- Generating LLVM IR for new constructs
- Type checking rules and inference
- The pattern: Type → TypeCheck → Codegen
