# What's Next: Your Journey Starts Here

Congratulations! You have built four programming languages from scratch:

| Language | What You Learned |
|----------|------------------|
| **[Calculator](./01_calculator/calc_intro.md)** | Grammars, parsing, ASTs, multiple backends |
| **[Firstlang](./02_firstlang/intro.md)** | Variables, functions, control flow, recursion, interpretation |
| **[Secondlang](./03_secondlang/intro.md)** | Static types, type inference, LLVM, JIT compilation |
| **[Thirdlang](./04_thirdlang/intro.md)** | Classes, methods, heap allocation, memory management |

These ~4000 lines of Rust cover the fundamentals of language implementation. But this is just the beginning. Here are paths you can explore next.

## Path 1: Extend Thirdlang

The simplest next step is to add features to Thirdlang:

### Inheritance

Add `class Dog extends Animal`:

```
class Animal {
    name: int  # Would need strings!
    def speak(self) -> int { return 0 }
}

class Dog extends Animal {
    def speak(self) -> int { return 1 }  # Override
}
```

This requires:

- **Vtables** - Virtual method tables for dynamic dispatch
- **Super calls** - `super.speak()` to call parent method
- **Type hierarchy** - `Dog` is a subtype of `Animal`

Read about [virtual method tables](https://en.wikipedia.org/wiki/Virtual_method_table) to understand the implementation.

### Interfaces / Traits

Add `impl Trait for Type`:

```rust,ignore
trait Printable {
    def print(self) -> int
}

impl Printable for Point {
    def print(self) -> int { return self.x }
}
```

This enables polymorphism without inheritance. See how [Rust traits](https://doc.rust-lang.org/book/ch10-02-traits.html) work.

### Generics

Add type parameters:

```rust,ignore
class Box<T> {
    value: T
    def get(self) -> T { return self.value }
}

b = new Box<int>(42)
```

This requires **monomorphization** (generating specialized code for each type) or **type erasure** (using runtime type info).

## Path 2: Add New Types

### Strings

```
s = "hello"
t = s + " world"
print(t)
```

Requires:

- String literals in the lexer
- String type with length tracking
- Concatenation, comparison, indexing
- Memory management (strings are heap-allocated)

### Arrays / Lists

```
arr = [1, 2, 3]
arr[0] = 10
```

Requires:

- Array syntax and type (`[int]` or `Array<int>`)
- Bounds checking
- Dynamic resizing (for lists)

### Structs (without methods)

```rust,ignore
struct Point { x: int, y: int }
p = Point { x: 10, y: 20 }
```

Simpler than classes - no methods, no heap allocation, just grouped data.

## Path 3: Advanced Features

### Closures

Functions that capture their environment:

```
def make_adder(n: int) -> (int) -> int {
    return def(x: int) -> int { return x + n }
}

add5 = make_adder(5)
add5(10)  # returns 15
```

Requires:

- Function types as first-class values
- Capturing variables (by value or reference)
- Closure representation (function pointer + environment)

### Pattern Matching

```rust,ignore
match value {
    0 => "zero",
    1 => "one",
    n => "other"
}
```

Or with destructuring:

```rust,ignore
match point {
    Point { x: 0, y } => "on y-axis",
    Point { x, y: 0 } => "on x-axis",
    _ => "elsewhere"
}
```

### Algebraic Data Types (ADTs)

```rust,ignore
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

Combined with pattern matching, this is incredibly powerful.

## Path 4: Better Error Handling

### Source Locations

Track line and column numbers:

```
Error at line 5, column 10:
    x = 1 + true
            ^^^^ expected int, got bool
```

### Error Recovery

Continue parsing after errors to report multiple issues:

```
Error 1: Undefined variable 'foo' at line 3
Error 2: Type mismatch at line 7
Error 3: Missing semicolon at line 12
```

### Helpful Messages

```
Error: Cannot add int and bool
  --> example.tl:5:9
   |
 5 |     x = 1 + true
   |         ^^^^^^^^
   |
   = hint: did you mean to compare? Try `1 == true` or `1 != true`
```

See [Rust's error messages](https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html) for inspiration.

## Path 5: Different Execution Models

### Ahead-of-Time (AOT) Compilation

Instead of JIT, compile to a standalone executable:

```bash
./mycompiler program.tl -o program
./program  # Run without compiler
```

LLVM makes this straightforward - use `TargetMachine` to emit object files.

### Bytecode Interpreter

Like Python or Java:

1. Compile to bytecode (`.pyc`, `.class`)
2. Interpret bytecode in a VM
3. Optionally JIT hot paths

We touched on this in the [Calculator VM](./01_calculator/vm.md).

### Transpilation

Compile to another high-level language:

```bash
./mycompiler program.tl --target=javascript > program.js
node program.js
```

JavaScript, C, and WebAssembly are popular targets.

## Path 6: Memory Management

### Reference Counting

Automatically free objects when no references remain:

```
p = new Point(1, 2)  # refcount = 1
q = p                 # refcount = 2
delete q              # refcount = 1
# p goes out of scope # refcount = 0, freed
```

Watch out for **cycles** (A points to B, B points to A).

### Garbage Collection

Automatically find and free unreachable objects:

- **Mark and sweep** - Mark reachable objects, free the rest
- **Copying collector** - Copy live objects to new space
- **Generational** - Young objects collected more often

Read [The Garbage Collection Handbook](https://gchandbook.org/) for deep coverage.

### Ownership (Rust-style)

Compiler enforces memory safety without runtime cost:

```rust,ignore
p = new Point(1, 2)
q = p      # p is moved, can't use p anymore
delete q   # Only q can free
```

This is complex but eliminates memory bugs at compile time.

## Path 7: Optimizations

### Constant Folding

We did basic folding in [Secondlang](./03_secondlang/optimizations.md). Go further:

```
x = 1 + 2 * 3    # Fold to x = 7
y = x + 0        # Fold to y = x
z = x * 1        # Fold to z = x
```

### Inlining

Replace function calls with function bodies:

```
def square(x: int) -> int { return x * x }
y = square(5)  # Inline to: y = 5 * 5
```

### Dead Code Elimination

Remove code that cannot be reached:

```
if (false) {
    # This entire block can be removed
}
```

### Escape Analysis

Stack-allocate objects that don't escape:

```
def foo() {
    p = new Point(1, 2)  # Could be stack-allocated
    return p.x + p.y     # p doesn't escape
}
```

## Path 8: Tooling

### REPL

Interactive programming:

```
> x = 10
> x + 5
15
> def double(n) { return n * 2 }
> double(x)
20
```

We built simple REPLs in [Calculator](./01_calculator/repl.md) and [Firstlang](./02_firstlang/repl.md).

### Debugger

Step through code, inspect variables:

```
(debug) break main.tl:10
(debug) run
Breakpoint hit at main.tl:10
(debug) print x
x = 42
(debug) step
```

### Language Server (LSP)

IDE features for your language:

- Syntax highlighting
- Go to definition
- Autocomplete
- Inline errors

See the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/).

### Formatter

Automatically format code consistently:

```bash
./myfmt program.tl  # Like rustfmt or prettier
```

## Recommended Reading

To go deeper, study these resources:

### Books

- **[Crafting Interpreters](https://craftinginterpreters.com)** by Robert Nystrom
  - Free online, covers two complete interpreters
- **[Engineering a Compiler](https://www.elsevier.com/books/engineering-a-compiler/cooper/978-0-12-815412-0)** by Cooper & Torczon
  - Comprehensive academic textbook
- **[Types and Programming Languages](https://www.cis.upenn.edu/~bcpierce/tapl/)** by Benjamin Pierce
  - Deep dive into type systems

### Online Resources

- **[LLVM Tutorial](https://llvm.org/docs/tutorial/)** - Build a language with LLVM (C++)
- **[Make a Lisp](https://github.com/kanaka/mal)** - Implement Lisp in your favorite language
- **[Write You a Haskell](http://dev.stephendiehl.com/fun/)** - Build a functional language

### Study Real Languages

Read the source code of real language implementations:

- **[Rust](https://github.com/rust-lang/rust)** - Complex but well-documented
- **[Go](https://github.com/golang/go)** - Clean, readable compiler
- **[Lua](https://github.com/lua/lua)** - Small, elegant VM
- **[CPython](https://github.com/python/cpython)** - Bytecode interpreter

### Languages Implemented in Rust

These projects are excellent case studies since they use the same language (Rust) and often similar techniques:

| Project | Description | What to Learn |
|---------|-------------|---------------|
| **[Gleam](https://github.com/gleam-lang/gleam)** | Type-safe language for Erlang VM | Type inference, BEAM codegen |
| **[Roc](https://github.com/roc-lang/roc)** | Fast, friendly functional language | Advanced type system, optimizations |
| **[Boa](https://github.com/boa-dev/boa)** | JavaScript engine in Rust | Spec compliance, JIT compilation |
| **[RustPython](https://github.com/RustPython/RustPython)** | Python interpreter in Rust | Bytecode VM, Python compatibility |
| **[Artichoke](https://github.com/artichoke/artichoke)** | Ruby implementation in Rust | Dynamic language implementation |
| **[Rhai](https://github.com/rhaiscript/rhai)** | Embedded scripting for Rust | Embedding, safe sandboxing |
| **[Gluon](https://github.com/gluon-lang/gluon)** | Functional, statically typed | Type inference, embeddable |
| **[Mun](https://github.com/mun-lang/mun)** | Hot-reloading language | Incremental compilation, runtime |
| **[Ante](https://github.com/jfecher/ante)** | Low-level functional language | Refinement types, lifetime inference |
| **[Scryer Prolog](https://github.com/mthom/scryer-prolog)** | ISO Prolog in Rust | Logic programming, WAM |
| **[Jakt](https://github.com/SerenityOS/jakt)** | Memory-safe systems language | Safety without GC |
| **[Koto](https://github.com/koto-lang/koto)** | Simple scripting language | Clean codebase, good for learning |
| **[Dyon](https://github.com/PistonDevelopers/dyon)** | Scripting for game engines | Dynamic typing, Rust integration |

Start with smaller projects like **Koto** or **Rhai** - their codebases are approachable and well-organized. Graduate to **Gleam** or **Boa** for more advanced techniques.

## Final Thoughts

Building programming languages teaches you:

1. **How computers work** - From source code to machine instructions
2. **Trade-offs everywhere** - Safety vs speed, simplicity vs power
3. **Abstraction design** - What to expose, what to hide
4. **Systems thinking** - How parts compose into wholes

The concepts you learned - grammars, ASTs, type systems, code generation - appear everywhere in software:

- **Compilers and interpreters** (obviously)
- **Query languages** (SQL, GraphQL)
- **Configuration languages** (YAML, TOML, HCL)
- **Template engines** (Jinja, Handlebars)
- **Domain-specific languages** (regex, CSS, makefiles)

You now have the foundation to understand, modify, or create any of these.

**Your journey starts here. Happy hacking!**
