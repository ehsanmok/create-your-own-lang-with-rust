Here is a bird's-eye view of a computer program execution

<p align="center">
</br>
    <a href><img alt="compiler" src="./img/code-compiler-executor.svg"> </a>
</p>

All these three components are intertwined together and learning their connections is crucial in understanding what makes *Computing* possible. Informally, a *language* is a structured text with syntax and semantics. A *Source Code* written in a programming language needs a translator/compiler of *some sort*, to translate it to *another* language/format. Then an executor of *some sort*, to execute/run the translated commands with the goal of matching the syntax (and semantics) to *some form* of output.

## Elements of Computing

### What is a Grammar?

A **[formal grammar](https://en.wikipedia.org/wiki/Formal_grammar)** is a set of rules that define what makes valid code in a programming language. Think of it like the grammar of English: "The cat sat" is valid, but "Cat the sat" is not.

For programming languages, a grammar specifies:

- **What tokens are valid** - keywords like `def`, `if`, `return`; operators like `+`, `-`, `*`; literals like `42`, `true`
- **How tokens can be combined** - `1 + 2` is valid, `+ + 1` is not
- **The structure of programs** - functions contain statements, statements contain expressions

We use **[PEG (Parsing Expression Grammar)](https://en.wikipedia.org/wiki/Parsing_expression_grammar)** to define our grammars. PEG is a modern approach that is easier to learn than traditional techniques like [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) or parser generators like [yacc](https://en.wikipedia.org/wiki/Yacc)/[bison](https://en.wikipedia.org/wiki/GNU_Bison).

### PEG and pest Syntax

We use **[pest](https://pest.rs)**, a Rust library that generates [parsers](https://en.wikipedia.org/wiki/Parsing) from PEG grammars. Here is a quick reference of pest syntax:

| Syntax | Meaning | Example |
|--------|---------|---------|
| `"text"` | Match exact text | `"def"` matches the keyword def |
| `~` | Sequence (then) | `"if" ~ "(" ~ Expr ~ ")"` matches `if` followed by `(` |
| `\|` | Choice (or) | `"true" \| "false"` matches either |
| `*` | Zero or more ([Kleene star](https://en.wikipedia.org/wiki/Kleene_star)) | `Stmt*` matches any number of statements |
| `+` | One or more | `ASCII_DIGIT+` matches one or more digits |
| `?` | Optional | `ReturnType?` matches zero or one return type |
| `{ }` | Rule definition | `Add = { "+" }` defines a rule |
| `_{ }` | Silent rule | `_{ Expr }` matches but does not appear in AST |
| `@{ }` | Atomic rule | `@{ ASCII_DIGIT+ }` matches as a single token |
| `SOI` | Start of input | Beginning of the source code |
| `EOI` | End of input | End of the source code |

**Silent rules** (`_{ }`) are useful for grouping without cluttering the [parse tree](https://en.wikipedia.org/wiki/Parse_tree). For example, whitespace rules are typically silent.

**Atomic rules** (`@{ }`) prevent whitespace from being inserted between parts. `@{ ASCII_DIGIT+ }` matches `123` as one token, not `1 2 3`.

A simple grammar example:

```text
Program = _{ SOI ~ Expr ~ EOI }       // A program is an expression
Expr = { Int | "(" ~ Expr ~ ")" }     // An expression is an int or parenthesized expr
Int = @{ ASCII_DIGIT+ }               // An int is one or more digits (atomic)
WHITESPACE = _{ " " | "\t" | "\n" }   // Whitespace is silent (ignored)
```

This grammar accepts: `42`, `(42)`, `((42))`, etc.

We use this pest syntax throughout the book. See [Calculator's grammar](./01_calculator/grammar_lexer_parser.md) for a real example, [Firstlang's syntax](./02_firstlang/syntax.md) for a more complex grammar, and [Secondlang's type annotations](./03_secondlang/annotations.md) for adding types.

### Instructions and the Machine Language

If you want to create a "computer" from scratch, you need to start by defining an *abstract model* for your computer. This abstract model is also referred to as **[Instruction Set Architecture (ISA)](https://en.wikipedia.org/wiki/Instruction_set_architecture)** (instruction set or simply *instructions*). A [CPU](https://en.wikipedia.org/wiki/Central_processing_unit) is an *implementation* of such ISA. A standard ISA defines its basic elements such as *data types*, *[register](https://en.wikipedia.org/wiki/Processor_register)* values, various hardware supports, I/O etc. and they all make up the  *lowest-level language* of computing which is the **[Machine Language](https://en.wikipedia.org/wiki/Machine_code) Instructions.**

Instructions are comprised of *instruction code* (aka *operation code*, in short **[opcode](https://en.wikipedia.org/wiki/Opcode)** or p-code) which are directly executed by the CPU. An opcode can either have operand(s) or no operand. For example, in an 8-bit machine where instructions are 8 bits, an opcode *load* might be defined by the 4 bits **0011** followed by the second 4 bits as operand with **0101**, making up the instruction **00110101** in Machine Language. The opcode for *incrementing by 1* of the previously loaded value could be defined by **1000** with no operand.

Since *opcodes are like atoms of computing*, they are presented in an opcode table. An example of that is the [x86 opcode reference](https://www.felixcloutier.com/x86/).

### Assembly Language

Since it's hard to remember the opcodes by their bit-patterns, we can assign *abstract* symbols to opcodes matching their operations by name. This way, we can create [Assembly language](https://en.wikipedia.org/wiki/Assembly_language) from the Machine Language. In the previous Machine Language example above, **00110101** (means load the binary **0101**), we can define the symbol **LOAD** referring to **0011** as a higher level abstraction so that **00110101** can be written as **LOAD 0101**.

The utility program that translates the Assembly language to Machine Language is called an **[Assembler](https://en.wikipedia.org/wiki/Assembly_language#Assembler)**.

### Compiler

<p align="center">
</br>
    <a href><img alt="compiler" src="./img/compiler.svg"> </a>
</p>

A [compiler](https://en.wikipedia.org/wiki/Compiler) is any program that translates (maps, encodes) a language A to language B. Each compiler has two major components:

- **Frontend:** deals with mapping the source code string to a structured format called **[Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree)**
- **Backend (code generator):** translates the AST into the [Bytecode](#bytecode) / [IR](#intermediate-representation-ir) or Assembly

Most often, when we talk about compiler, we mean **[Ahead-Of-Time (AOT)](https://en.wikipedia.org/wiki/Ahead-of-time_compilation)** compiler where the translation happens *before* execution. Another form of translation is **[Just-In-Time (JIT)](https://en.wikipedia.org/wiki/Just-in-time_compilation)** compilation where translation happens right at the time of the execution.

From the diagram above, to distinguish between a program that translates for example, Python to Assembly vs. Python to Java, the former is called compiler and the latter **[transpiler](https://en.wikipedia.org/wiki/Source-to-source_compiler)** (or source-to-source compiler).

#### *Relativity of low-level, high-level*

Assembly is a *high-level* language compared to the Machine Language but is considered *low-level* when viewing it from C/C++/Rust. High-level and low-level are relative terms conveying the amount of *abstractions* involved.

### Virtual Machine (VM)

[Instructions](#instructions-and-the-machine-language) are hardware and vendor specific. That is, an Intel CPU instructions are different from AMD CPU. A **[Virtual Machine (VM)](https://en.wikipedia.org/wiki/Virtual_machine#Process_virtual_machines)** abstracts away details of the underlying hardware or operating system so that programs translated/compiled into the VM language becomes platform agnostic. A famous example is the **[Java Virtual Machine (JVM)](https://en.wikipedia.org/wiki/Java_virtual_machine)** which translates/compiles Java programs to JVM language aka Java **[Bytecode](https://en.wikipedia.org/wiki/Java_bytecode)**. Therefore, if you have a valid Java Bytecode and *Java Runtime Environment (JRE)* in your system, you can execute the Bytecode, regardless on what platform it was compiled on.

#### Bytecode

Another technique to translate source code to Machine Code is emulating the Instruction Set with a new (human-friendly) encoding (perhaps easier than assembly). [Bytecode](https://en.wikipedia.org/wiki/Bytecode) is such an *intermediate language/representation* which is lower-level than the actual programming language that it was translated from, and higher-level than Assembly language.

#### Stack Machine

A [Stack Machine](https://en.wikipedia.org/wiki/Stack_machine) is a simple model for a computing machine with two main components:

- a memory (stack) array keeping the Bytecode instructions that supports `push`ing and `pop`ing instructions
- an instruction pointer (IP) and stack pointer (SP) guiding which instruction was executed and what is next.

We implement a stack-based bytecode VM in the [Calculator VM chapter](./01_calculator/vm.md).

### Intermediate Representation (IR)

Any representation that's between source code and (usually) Assembly language is considered an [intermediate representation](https://en.wikipedia.org/wiki/Intermediate_representation). Mainstream languages usually have more than one such representations and going from one IR to another IR is called *lowering*.

We explore [LLVM IR](https://en.wikipedia.org/wiki/LLVM#Intermediate_representation) in detail in the [Secondlang IR chapter](./03_secondlang/ir.md).

### Code Generation

[Code generation](https://en.wikipedia.org/wiki/Code_generation_(compiler)) for a compiler is when the compiler *converts an IR to some Machine Code*. But it has a wider semantic too for example, when using Rust declarative macro via `macro_rules!` to automate some repetitive implementations, you're essentially generating codes (as well as expanding the syntax).

## Conclusion

In conclusion, we want to settle one of the most frequently asked questions

## <span style="color:blue">Is Python (or a language X) Compiled or Interpreted?</span>

This is in fact the <span style="color:red">WRONG</span> question to ask!

Being AOT compiled, JIT compiled or interpreted is **implementation-dependent**. For example, the standard Python *implementation* is [**CPython**](https://www.python.org/) which compiles a Python source code (in CPython VM) to CPython Bytecode (contents of `.pyc`) and **interprets** the Bytecode. However, another implementation of Python is [**PyPy**](https://www.pypy.org/) which (more or less) compiles a Python source code (in PyPy VM) to PyPy Bytecode and **JIT** compiles the PyPy Bytecode to the Machine Code (and is usually faster than CPython interpreter).
