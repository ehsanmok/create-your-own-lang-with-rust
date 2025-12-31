# Create Your Own Programming Language with Rust

<!-- toc -->

- [Introduction](intro.md)
- [Crash Course on Computing](crash_course.md)

---

## Part I: Calculator

- [Calculator](01_calculator/calc_intro.md)
  - [Grammar, Lexer and Parser](01_calculator/grammar_lexer_parser.md)
  - [Abstract Syntax Tree (AST) and Interpreter](01_calculator/ast.md)
  - [Just-In-Time (JIT) Compiler with LLVM](01_calculator/jit_intro.md)
    - [Basic Example](01_calculator/basic_llvm.md)
    - [AST Traversal Patterns](01_calculator/ast_traversal.md)
  - [Virtual Machine (VM), Bytecode and Interpreter](01_calculator/vm.md)
  - [Read-Eval-Print Loop (REPL)](01_calculator/repl.md)
  - [Debugging Your Language](01_calculator/debugging.md)
  - [Exercises](01_calculator/exercise.md)

---

- [From Calculator to Real Language](transition_1_to_2.md)

## Part II: Firstlang (Interpreted)

- [Firstlang: Your First Real Language](02_firstlang/intro.md)
  - [Python-like Syntax](02_firstlang/syntax.md)
  - [Variables and Assignment](02_firstlang/variables.md)
  - [Functions](02_firstlang/functions.md)
  - [Control Flow: If/Else and While](02_firstlang/control_flow.md)
  - [Recursion](02_firstlang/recursion.md)
  - [Building the REPL](02_firstlang/repl.md)
  - [Computing Fibonacci](02_firstlang/fibonacci.md)
  - [Exercises](02_firstlang/exercises.md)

---

- [From Interpreted to Compiled](transition_2_to_3.md)

## Part III: Secondlang (Compiled)

- [Secondlang: Adding Types and Compilation](03_secondlang/intro.md)
  - [Why Types Matter](03_secondlang/why_types.md)
  - [Type Annotations](03_secondlang/annotations.md)
  - [Type Inference](03_secondlang/inference.md)
  - [AST Optimizations (Visitor Pattern)](03_secondlang/optimizations.md)
  - [From AST to IR](03_secondlang/ir.md)
  - [LLVM Code Generation](03_secondlang/codegen.md)
  - [JIT Compiling Fibonacci](03_secondlang/jit_fibonacci.md)
  - [Exercises](03_secondlang/exercises.md)

---

- [From Functions to Objects](transition_3_to_4.md)

## Part IV: Thirdlang (Object-Oriented)

- [Thirdlang: Adding Classes and Objects](04_thirdlang/intro.md)
  - [Why Classes?](04_thirdlang/why_classes.md)
  - [Class Syntax and Parsing](04_thirdlang/classes_syntax.md)
  - [Constructors and Object Creation](04_thirdlang/constructors.md)
  - [Methods and Self](04_thirdlang/methods.md)
  - [Memory Management](04_thirdlang/memory.md)
  - [LLVM Code Generation for Classes](04_thirdlang/codegen_classes.md)
  - [Optimizing LLVM IR](04_thirdlang/optimization.md)
  - [Running Thirdlang](04_thirdlang/running.md)
  - [Exercises](04_thirdlang/exercises.md)

---

- [What's Next: Your Journey Starts Here](whats_next.md)

---

- [Appendix: Testing Your Language](testing.md)
- [Glossary](glossary.md)
- [Resources](resources.md)
