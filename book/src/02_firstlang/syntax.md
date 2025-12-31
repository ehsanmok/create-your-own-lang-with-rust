# Python-like Syntax

Firstlang uses a Python-inspired syntax that's familiar and easy to read. Let's look at the grammar.

## The Grammar

Our grammar is defined using [PEG (Parsing Expression Grammar)](https://en.wikipedia.org/wiki/Parsing_expression_grammar) via the `pest` library. If you need a refresher on pest syntax (`~`, `|`, `*`, `@{}`, etc.), see the [PEG and pest Syntax](../crash_course.md#peg-and-pest-syntax) section. Here are the key constructs:

### Expressions

```pest
// Basic values
Int = @{ ASCII_DIGIT+ }           // 42, 123
Bool = @{ "true" | "false" }       // true, false
Identifier = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }  // x, myVar

// Operators (by precedence)
Comparison = { Additive ~ (CompOp ~ Additive)* }    // x < 10
Additive = { Multiplicative ~ (AddOp ~ Multiplicative)* }  // x + y
Multiplicative = { Unary ~ (MulOp ~ Unary)* }       // x * y
Unary = { UnaryOp ~ Unary | Call }                  // -x, !flag
```

### Statements

```pest
// Variable assignment
Assignment = { Identifier ~ "=" ~ Expr }
// Example: x = 42

// Return statement
Return = { "return" ~ Expr }
// Example: return x + 1

// Function definition
Function = { "def" ~ Identifier ~ "(" ~ Params? ~ ")" ~ Block }
// Example: def add(a, b) { return a + b }
```

### Control Flow

```pest
// Conditional
Conditional = { "if" ~ "(" ~ Expr ~ ")" ~ Block ~ "else" ~ Block }
// Example: if (x < 10) { 1 } else { 2 }

// While loop
WhileLoop = { "while" ~ "(" ~ Expr ~ ")" ~ Block }
// Example: while (x < 10) { x = x + 1 }

// Block of statements
Block = { "{" ~ Stmt* ~ "}" }
```

## Examples

### Simple Arithmetic

```
1 + 2 * 3       # = 7 (multiplication first)
(1 + 2) * 3     # = 9 (parentheses override)
-5 + 3          # = -2 (unary minus)
```

### Variables

```
x = 10
y = x + 5       # = 15
```

### Functions

```
def greet() {
    return 42
}

def add(a, b) {
    return a + b
}

add(3, 4)       # = 7
```

### Conditionals

```
def max(a, b) {
    if (a > b) {
        return a
    } else {
        return b
    }
}

max(10, 20)     # = 20
```

### Loops

```
x = 0
while (x < 5) {
    x = x + 1
}
x               # = 5
```

### Recursion

```
def factorial(n) {
    if (n <= 1) {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

factorial(5)    # = 120
```

## Full Grammar

The complete grammar is in `src/grammar.pest`. The key design choices are:

1. **Whitespace insensitive** - Unlike Python, braces `{}` delimit blocks
2. **No semicolons** - Newlines separate statements
3. **Parentheses required** - For `if` and `while` conditions
4. **Expression-based** - Everything returns a value

In the [next section](./variables.md), we'll see how variables work and how the interpreter manages scope.
