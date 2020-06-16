Here is a bird's-eye view of a computer program execution

<p align="center">
</br>
    <a href><img alt="compiler" src="./img/code_compiler_executor.svg"> </a>
</p>


All these three components are intertwined together and learning their connections is crucial in understanding what makes *Computing* possible. Informally, a *language* is a structured text with syntax and semantics. A *Source Code* written in a programming language needs a translator/compiler of *some sort*, to translate it to *another* language/format. Then an executor of *some sort*, to execute/run the translated commands with the goal of matching the syntax (and semantics) to *some form* of output.

## Elements of Computing

### Instructions and the Machine Language

If you want to create a "computer" from scratch, you need to start by defining an *abstract model* for your computer. This abstract model is also referred to as **Instruction Set Architecture (ISA)** (instruction set or simply *instructions*). A CPU is an *implementation* of such ISA. A standard ISA defines its basic elements such as *data types*, *register* values, various hardware supports, I/O etc. and they all make up the  *lowest-level language* of computing which is the **Machine Language Instructions.**

Instructions are comprised of *instruction code* (aka *operation code*, in short **opcode** or p-code) which are directly executed by CPU. An opcode can either have operand(s) or no operand. For example, in an 8-bits machine where instructions are 8-bits an opcode *load* might be defined by the 4-bits **0011** following by the second 4-bits as operand with **0101** that makes up the instruction **00110101** in the Machine Language while the opcode for *incrementing by 1* of the previously loaded value could be defined by **1000** with no operand.

Since *opcodes are like atoms of computing*, they are presented in an opcode table. An example of that is [Intel x86 opcode table](http://sparksandflames.com/files/x86InstructionChart.html).

### Assembly Language

Since it's hard to remember the opcodes by their bit-patterns, we can assign *abstract* symbols to opcodes matching their operations by name. This way, we can create Assembly language from the Machine Language. In the previous Machine Language example above, **00110101** (means load the binary **0101**), we can define the symbol **LOAD** referring to **0011** as a higher level abstraction so that **00110101** can be written as **LOAD 0101**.

The utility program that translates the Assembly language to Machine Language is called **Assembler**.

### Compiler

<p align="center">
</br>
    <a href><img alt="compiler" src="./img/compiler.svg"> </a>
</p>

Compiler is any program that translates (maps, encodes) a language A to language B. Each compiler has two major components

* **Frontend:** deals with mapping the source code string to a structured format called **Abstract Syntax Tree (AST)**
* **Backend (code generator):** translates the AST into the [Bytecode](./crash_course.md#bytecode) / [IR](./crash_course.md#intermediate-representation-ir) or Assembly

 Most often, when we talk about compiler, we mean **Ahead-Of-Time (AOT)** compiler where the translation happens *before* execution. Another form of translation is **Just-In-Time (JIT)** compilation where translation happens right at the time of the execution.

From the diagram above, to distinguish between a program that translates for example, Python to Assembly vs. Python to Java, the former is called compiler and the latter **transpiler**.

#### *Relativity of low-level, high-level*

Assembly is a *high-level* language compared to the Machine Language but is considered *low-level* when viewing it from C/C++/Rust. High-level and low-level are relative terms conveying the amount of *abstractions* involved.


### Virtual Machine (VM)

[Instructions](./crash_course.md#instructions-and-the-machine-language) are hardware and vendor specific. That is, an Intel CPU instructions are different from AMD CPU. A **VM** abstracts away details of the underlying hardware or operating system so that programs translated/compiled into the VM language becomes platform agnostic. A famous example is the **Java Virtual Machine (JVM)**
which translates/compiles Java programs to JVM language aka Java **Bytecode**. Therefore, if you have a valid Java Bytecode and *Java Runtime Environment (JRE)* in your system, you can execute the Bytecode, regardless on what platform it was compiled on.

#### Bytecode

Another technique to translate a source code to Machine Code, is emulating the Instruction Set with a new (human friendly) encoding (perhaps easier than assembly). Bytecode is such an *intermediate language/representation* which is lower-level than the actual programming language that has been translated from and higher-level than Assembly language.

#### Stack Machine

Stack Machine is a simple model for a computing machine with two main components
* a memory (stack) array keeping the Bytecode instructions that supports `push`ing and `pop`ing instructions
* an instruction pointer (IP) and stack pointer (SP) guiding which instruction was executed and what is next.

### Intermediate Representation (IR)

Any representation that's between source code and (usually) Assembly language is considered an intermediate representation. Mainstream languages usually have more than one such representations and going from one IR to another IR is called *lowering*.

### Code Generation

Code generation for a compiler is when the compiler *converts an IR to some Machine Code*. But it has a wider semantic too for example, when using Rust declarative macro via `macro_rules!` to automate some repetitive implementations, you're essentially generating codes (as well as expanding the syntax).

## Conclusion

In conclusion, we want to settle one of the most frequently asked questions

## <span style="color:blue">Is Python (or a language X) Compiled or Interpreted?</span>

This is in fact the <span style="color:red">WRONG</span> question to ask!

Being AOT compiled, JIT compiled or interpreted is **implementation-dependent**. For example, the standard Python *implementation* is [**CPython**](https://www.python.org/) which compiles a Python source code (in CPython VM) to CPython Bytecode (contents of `.pyc`) and **interprets** the Bytecode. However, another implementation of Python is [**PyPy**](https://www.pypy.org/) which (more or less) compiles a Python source code (in PyPy VM) to PyPy Bytecode and **JIT** compiles the PyPy Bytecode to the Machine Code (and is usually faster than CPython interpreter).
