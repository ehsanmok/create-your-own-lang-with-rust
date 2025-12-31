# Exercises

These exercises will deepen your understanding of interpreted languages. Try them in order.

## Exercise 1: Add Print Statement

Add a `print` builtin that outputs values:

```
print(42)
print(fib(10))
```

**Hints:**

- Add `Print` variant to `Stmt` in `ast.rs`
- Parse `print(expr)` in `parser.rs`
- In the interpreter, evaluate the expression and use `println!`

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>print(1 + 2)</code> should output <code>3</code></p>

## Exercise 2: Add For Loop

Add a `for` loop with range syntax:

```
for i in 0..10 {
    print(i)
}
```

**Hints:**

- Add `For { var, start, end, body }` to `Stmt`
- Grammar: `For = { "for" ~ Identifier ~ "in" ~ Expr ~ ".." ~ Expr ~ Block }`
- In interpreter: create a loop that assigns `var` from `start` to `end-1`

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>for i in 0..3 { print(i) }</code> outputs <code>0</code>, <code>1</code>, <code>2</code></p>

## Exercise 3: Add Comparison Operators

Add `<`, `>`, `<=`, `>=`, `==`, `!=`:

```
if (x < 10) {
    return 1
}
```

**Hints:**

- Add new variants to `BinaryOp` enum
- Return `1` for true, `0` for false (we have no bool type yet)
- Update grammar: `CompOp = { "<=" | ">=" | "<" | ">" | "==" | "!=" }`

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>5 &lt; 10</code> returns <code>1</code>, <code>5 &gt; 10</code> returns <code>0</code></p>

## Exercise 4: Add Logical Operators

Add `and` and `or` with short-circuit evaluation:

```
if (x > 0 and x < 10) {
    return 1
}
```

**Hints:**

- For `and`: if left is `0`, return `0` without evaluating right
- For `or`: if left is non-zero, return it without evaluating right
- This is called short-circuit evaluation

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>0 and crash()</code> should not call <code>crash()</code></p>

## Exercise 5: Add String Type

Add string literals and concatenation:

```
s = "hello"
t = s + " world"
print(t)
```

**Hints:**

- Add `Value::String(String)` to your value type
- Parse string literals: `StringLit = { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }`
- Implement `+` for string concatenation

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>"hello" + " world"</code> returns <code>"hello world"</code></p>

## Stretch Goals

If you complete the above:

1. **Arrays**: `arr = [1, 2, 3]; arr[0]`
2. **Modulo operator**: `10 % 3` returns `1`
3. **Unary minus**: `-5`, `-(3 + 2)`
4. **Comments**: `# this is a comment`
5. **Multi-line strings**: `"""multi\nline"""`

## What You Practiced

- Extending the grammar incrementally
- Adding new AST node types
- Modifying the interpreter for new constructs
- The pattern: Grammar → AST → Interpreter
