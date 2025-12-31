# Glossary

Key terms used throughout this book.

---

**Abstract Syntax Tree (AST)**: A tree representation of source code structure. Each node represents a construct (expression, statement, etc.). Intermediate representation between parsing and execution.

**Alloca**: LLVM instruction that allocates space on the stack. Used for local variables. Optimized away by `mem2reg` when possible.

**AOT (Ahead-of-Time) Compilation**: Compiling to native code before execution, producing a standalone executable. Contrast with JIT.

**Backend**: The part of a compiler that generates output (machine code, bytecode, IR). Receives input from the frontend.

**Basic Block**: A sequence of instructions with no branches except at the end. Control enters at the beginning, exits at the end.

**Bytecode**: A compact, portable instruction set for a virtual machine. Higher-level than machine code, interpreted or JIT-compiled.

**Call Stack**: Runtime data structure tracking function calls. Each call pushes a frame; return pops it.

**Codegen (Code Generation)**: The compiler phase that produces output code (LLVM IR, machine code, bytecode) from the AST.

**Constructor**: Special method (`__init__`) called when creating an object. Initializes fields.

**Dead Code Elimination (DCE)**: Optimization that removes code whose results are never used.

**Destructor**: Special method (`__del__`) called when destroying an object. Cleans up resources.

**Dynamic Typing**: Type checking at runtime. Variables can hold values of any type. Used in Python, JavaScript.

**Frontend**: The part of a compiler that processes source code (lexing, parsing, type checking). Produces input for the backend.

**Fuzzing**: Automated testing technique that generates random inputs to find crashes, hangs, or security issues.

**GEP (GetElementPointer)**: LLVM instruction for calculating addresses within structs or arrays. Does not access memory, only computes pointers.

**Grammar**: Formal rules defining valid syntax. We use PEG (Parsing Expression Grammar) with pest.

**Heap**: Memory region for dynamic allocation (`malloc`/`new`). Objects persist until explicitly freed. Contrast with stack.

**Intermediate Representation (IR)**: Code representation between source and machine code. LLVM IR is a common example. Enables optimization independent of source/target.

**Interpreter**: Executes code directly without compilation to machine code. Tree-walking interpreters traverse the AST.

**JIT (Just-In-Time) Compilation**: Compiling to native code during execution. Combines interpretation flexibility with native speed.

**Lexer (Tokenizer)**: Converts source text into tokens (identifiers, numbers, operators). First stage of parsing.

**LLVM**: Compiler infrastructure providing reusable components for building compilers. We use it for code generation and optimization.

**Mangling**: Encoding names to be unique. `Counter.increment` becomes `Counter__increment` in generated code.

**mem2reg**: LLVM optimization pass that promotes stack allocations (alloca) to SSA registers. Essential for efficient code.

**Method**: Function associated with a class. Receives `self` as implicit first parameter.

**Monomorphization**: Generating specialized code for each concrete type when using generics. `Box<int>` and `Box<bool>` become separate implementations.

**Opaque Pointer**: LLVM pointer type (`ptr`) without element type information. Modern LLVM uses opaque pointers exclusively.

**Optimization Pass**: A transformation that improves code without changing behavior. Examples: DCE, constant folding, inlining.

**Parse Tree**: Tree structure directly reflecting grammar rules. Typically converted to AST.

**Parser**: Converts tokens into structured representation (AST). Checks syntactic correctness.

**Pass Manager**: Infrastructure for running and ordering optimization passes. We use LLVM's New Pass Manager.

**PEG (Parsing Expression Grammar)**: Grammar formalism that is unambiguous and efficient to parse. Used by pest.

**Phi Node (φ)**: SSA construct for merging values from different control flow paths. "If we came from block A, use X; if from B, use Y."

**REPL (Read-Eval-Print Loop)**: Interactive environment for entering and executing code one expression at a time.

**Scope**: Region of code where a name is visible. Variables are local to their scope.

**Semantic Analysis**: Checking meaning (not just syntax). Type checking is semantic analysis.

**SSA (Static Single Assignment)**: IR form where each variable is assigned exactly once. Simplifies optimization.

**Stack**: Memory region for function calls and local variables. Automatically managed (push on call, pop on return). Fast but limited size.

**Static Typing**: Type checking at compile time. Errors caught before execution. Used in Rust, Java, C++.

**Struct Type**: LLVM type representing a group of fields at specific offsets. Used for class memory layout.

**Symbol Table**: Data structure mapping names to their definitions (variables, functions, types).

**Target Triple**: String describing a compilation target: architecture-vendor-os (e.g., `x86_64-apple-darwin`).

**Terminator**: Instruction that ends a basic block. Must be `ret`, `br`, `switch`, etc.

**Tree-Walking Interpreter**: Interpreter that traverses the AST directly, executing nodes recursively.

**Type Inference**: Automatically determining types without explicit annotations. `x = 5` infers `x: int`.

**Type System**: Rules governing how types work. Defines what operations are valid on which types.

**Typed AST**: AST with type information attached to each node. Result of type checking.

**Virtual Machine (VM)**: Software that executes bytecode. Examples: JVM, Python VM, our calculator VM.

**Visitor Pattern**: Design pattern for traversing tree structures. Each node type has a visit method.

**Vtable (Virtual Method Table)**: Table of function pointers for dynamic dispatch in OOP. Enables polymorphism.
