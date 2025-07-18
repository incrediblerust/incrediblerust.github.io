---
title: Hello World
difficulty: beginner
version: 1.85.0
prev_lesson: /lessons/installation/
prev_lesson_title: Installation
next_lesson: /lessons/cargo/
next_lesson_title: Cargo Package Manager
---

# Hello World

Let's write your first Rust program! Following programming tradition, we'll make a simple program that prints "Hello, world!" to the screen.

## Creating a Rust File

First, create a new directory for your Rust projects:

```bash
mkdir rust_projects
cd rust_projects
```

Now create a file called `main.rs`:

```bash
touch main.rs
```

Open `main.rs` in your favorite text editor and add the following code:

```rust
fn main() {
    println!("Hello, world!");
}
```

Let's break down what this code does:

1. `fn main()` - This defines a function named `main`. The `main` function is special: it's the first code that runs in every executable Rust program.
2. `println!("Hello, world!");` - This calls the `println!` macro to print text to the screen. The `!` indicates this is a macro, not a function.

## Compiling and Running

To run this program, we need to compile it first. Rust is a compiled language, which means we need to translate our source code into machine code before we can execute it.

Compile the program:

```bash
rustc main.rs
```

This creates an executable file. On Linux and macOS, it's called `main`. On Windows, it's `main.exe`.

Run the program:

```bash
./main        # On Linux/macOS
main.exe      # On Windows
```

You should see:

```
Hello, world!
```

Congratulations! You've written and run your first Rust program!

## Understanding the Syntax

Let's examine the code more closely:

```rust
fn main() {
    println!("Hello, world!");
}
```

### Function Definition

```rust
fn main() {
    // function body
}
```

- `fn` is the keyword for defining functions
- `main` is the function name
- `()` indicates this function takes no parameters
- `{}` contain the function body

### The println! Macro

```rust
println!("Hello, world!");
```

- `println!` is a macro that prints text to the console
- The `!` tells us this is a macro, not a function
- The text inside quotes is called a string literal
- The semicolon `;` ends the statement

## More Examples

Let's try a few variations:

### Printing Multiple Lines

```rust
fn main() {
    println!("Hello, world!");
    println!("I'm learning Rust!");
    println!("This is exciting!");
}
```

### Using Variables

```rust
fn main() {
    let name = "Rustacean";
    println!("Hello, {}!", name);
}
```

This introduces:
- `let` keyword for creating variables
- `{}` as a placeholder in the string
- String interpolation

### Multiple Placeholders

```rust
fn main() {
    let language = "Rust";
    let year = 2025;
    println!("I'm learning {} in {}!", language, year);
}
```

## Common Mistakes

When starting with Rust, watch out for these common issues:

### Forgetting Semicolons

```rust
fn main() {
    println!("Hello, world!")  // Missing semicolon - this will cause an error!
}
```

### Wrong Function Name

```rust
fn Main() {  // Capital 'M' - Rust won't recognize this as the entry point
    println!("Hello, world!");
}
```

The `main` function must have a lowercase 'm'.

### Forgetting the Exclamation Mark

```rust
fn main() {
    println("Hello, world!");  // Missing '!' - println is a macro, not a function
}
```

## Key Takeaways

- Every Rust program starts with a `main` function
- `println!` is used to print to the console
- Rust code must be compiled before running
- Statements end with semicolons
- Macros are called with `!`

## Next Steps

Now that you can write and run basic Rust programs, let's learn about [Cargo]({{ '/lessons/cargo/' | relative_url }}), Rust's package manager and build system, which will make managing Rust projects much easier.

## Try It Yourself

Before moving on, try these exercises:

1. Print your name instead of "world"
2. Print multiple lines with different messages
3. Create variables for your favorite programming language and the current year, then print them

Happy coding! 🦀