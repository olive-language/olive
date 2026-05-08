# Getting Started with Olive

Follow these steps to set up the Olive compiler and run your first program.

## Installation

Currently, Olive is built from source. Ensure you have the [Rust toolchain](https://rustup.rs/) installed on your machine.

1. **Clone the repository**:
   ```bash
   git clone https://github.com/ecnivs/olive.git
   cd olive
   ```

2. **Build the compiler**:
   ```bash
   cargo build --release
   ```

3. **Verify the installation**:
   ```bash
   ./target/release/olive --help
   ```

## Your First Program

Create a new file named `hello.liv` and add the following code:

```python
fn __main__():
    print_str("Hello, Olive!")
    return 0
```

> **Note**: Olive currently uses `__main__` as the entry point for execution.

## Running Your Code

To run your program, simply pass the file path to the Olive compiler:

```bash
./target/release/olive hello.liv
```

### Compiler Options

Olive provides several flags to help you understand what's happening under the hood:

- `--check`: Perform semantic analysis and borrow checking without running the code.
- `--emit-ast`: Print the Abstract Syntax Tree for debugging.
- `--emit-mir`: Print the Middle Intermediate Representation blocks.

## Hello, World Explained

Let's break down our first program:

- `fn __main__():`: Defines a function named `__main__`. In Olive, indentation is used to define blocks.
- `print_str("Hello, Olive!")`: A built-in function to print a string to the console.
- `return 0`: Olive functions usually return an integer status code.
