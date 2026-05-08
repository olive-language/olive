# Introduction to Olive

Olive is a systems-oriented programming language that combines Python's expressive syntax with Rust's memory safety and performance. 

Instead of relying on a garbage collector, Olive uses **Ownership-Based Resource Management (OBRM)** and a Middle Intermediate Representation (MIR) to manage memory deterministically at compile-time, making Olive suitable for latency-sensitive applications.

## Key Features

- **Pythonic Syntax**: Clean, readable code with indentation-based scoping.
- **Static Typing**: Optional type annotations that enable powerful compile-time checks.
- **Memory Safety**: A borrow checker inspired by Rust, featuring Non-Lexical Lifetimes (NLL).
- **JIT Compilation**: Powered by Cranelift, Olive compiles directly to machine code for lightning-fast execution.
- **Industrial Diagnostics**: Beautiful, informative error reporting using the `ariadne` library.

## Why Olive?

Olive was born from the desire to have a language that is as easy to write as Python but as fast and safe as Rust. It is ideal for:

- High-performance backend services.
- Systems programming where developer productivity is paramount.
- Scientific computing where speed and safety are non-negotiable.

## The Architecture

The Olive compiler pipeline is designed for clarity and extensibility:

1. **Lexical Analysis**: Source code is tokenized.
2. **Parsing**: Tokens are transformed into an Abstract Syntax Tree (AST).
3. **Semantic Analysis**: Symbol resolution and type checking ensure code correctness.
4. **MIR Lowering**: The AST is lowered to a Middle Intermediate Representation (MIR) for optimizations.
5. **Borrow Checking**: The MIR is analyzed to enforce memory safety rules.
6. **Codegen**: The MIR is compiled to machine code using the Cranelift JIT backend.
