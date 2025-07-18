---
title: Cargo Package Manager
difficulty: beginner
version: 1.85.0
prev_lesson: /lessons/hello-world/
prev_lesson_title: Hello World
next_lesson: /lessons/variables/
next_lesson_title: Variables and Mutability
---

# Cargo Package Manager

Cargo is Rust's built-in package manager and build system. It handles many tasks for you, including building your code, downloading dependencies, and running tests. Think of it as npm for Node.js or pip for Python.

## What is Cargo?

Cargo is much more than just a package manager. It's a complete build system that:

- Creates new projects with the right structure
- Builds your code
- Downloads and manages dependencies (called "crates" in Rust)
- Runs tests
- Generates documentation
- Publishes packages to [crates.io](https://crates.io)

## Creating a New Project

Instead of manually creating files like we did in the Hello World lesson, let's use Cargo to create a proper project:

```bash
cargo new hello_cargo
cd hello_cargo
```

This creates a new directory called `hello_cargo` with the following structure:

```
hello_cargo/
├── Cargo.toml
└── src/
    └── main.rs
```

Let's examine what Cargo created for us:

### Cargo.toml

The `Cargo.toml` file contains metadata about your project:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- `[package]` section contains configuration for your package
- `name` is the project name
- `version` follows semantic versioning
- `edition` specifies which Rust edition to use
- `[dependencies]` is where you list external crates

### src/main.rs

Cargo also created a basic "Hello, world!" program:

```rust
fn main() {
    println!("Hello, world!");
}
```

## Building and Running with Cargo

### Building Your Project

To compile your project:

```bash
cargo build
```

This creates an executable in `target/debug/hello_cargo` (or `target/debug/hello_cargo.exe` on Windows).

### Running Your Project

To compile and run in one step:

```bash
cargo run
```

This is much more convenient than manually compiling and running!

### Release Builds

For optimized builds (slower to compile, but faster to run):

```bash
cargo build --release
```

This creates the executable in `target/release/` instead of `target/debug/`.

### Checking Your Code

To check if your code compiles without creating an executable:

```bash
cargo check
```

This is much faster than `cargo build` and is useful for quickly checking for errors during development.

## Adding Dependencies

One of Cargo's most powerful features is dependency management. Let's add a popular crate for generating random numbers.

Edit your `Cargo.toml`:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

Now update `src/main.rs` to use the `rand` crate:

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
}
```

When you run `cargo build` or `cargo run`, Cargo will automatically download and compile the `rand` crate and its dependencies.

## Common Cargo Commands

Here are the most frequently used Cargo commands:

### Project Creation
```bash
cargo new project_name         # Create a new binary project
cargo new project_name --lib   # Create a new library project
```

### Building and Running
```bash
cargo build                    # Build the project
cargo build --release          # Build with optimizations
cargo run                      # Build and run the project
cargo check                    # Check for compilation errors
```

### Testing
```bash
cargo test                     # Run all tests
```

### Documentation
```bash
cargo doc                      # Generate documentation
cargo doc --open               # Generate and open documentation
```

### Cleaning
```bash
cargo clean                    # Remove target directory
```

## Project Structure

A typical Cargo project has this structure:

```
my_project/
├── Cargo.toml              # Project metadata and dependencies
├── Cargo.lock              # Exact dependency versions (auto-generated)
├── src/                    # Source code
│   ├── main.rs            # Main entry point for binary
│   └── lib.rs             # Main entry point for library
├── tests/                  # Integration tests
├── examples/               # Example code
└── target/                 # Compiled artifacts (auto-generated)
```

## Understanding Cargo.lock

When you build your project for the first time, Cargo creates a `Cargo.lock` file. This file contains the exact versions of all dependencies that were used. This ensures that anyone who builds your project will get exactly the same dependencies.

- **Commit `Cargo.lock`** if you're building a binary
- **Don't commit `Cargo.lock`** if you're building a library

## Finding Crates

The Rust community publishes crates (libraries) to [crates.io](https://crates.io). Some popular crates include:

- `serde` - Serialization and deserialization
- `tokio` - Async runtime
- `clap` - Command line argument parsing
- `reqwest` - HTTP client
- `regex` - Regular expressions

## Example: Building a Guessing Game

Let's create a simple guessing game to demonstrate Cargo in action:

```bash
cargo new guessing_game
cd guessing_game
```

Update `Cargo.toml`:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

Update `src/main.rs`:

```rust
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    println!("You guessed: {}", guess);
    println!("The secret number was: {}", secret_number);
}
```

Run the game:

```bash
cargo run
```

## Key Takeaways

- Cargo is Rust's package manager and build system
- Use `cargo new` to create new projects
- Use `cargo run` to build and run your project
- Dependencies are managed in `Cargo.toml`
- Cargo automatically downloads and builds dependencies
- `cargo check` is faster than `cargo build` for development

## Next Steps

Now that you understand how to use Cargo to manage Rust projects, let's dive into Rust's fundamentals. Continue to [Variables and Mutability]({{ '/lessons/variables/' | relative_url }}) to learn about how Rust handles data.

## Try It Yourself

1. Create a new Cargo project called `my_calculator`
2. Add the `rand` crate as a dependency
3. Modify the main function to print a random number
4. Use `cargo check` to verify your code compiles
5. Run your program with `cargo run`

Happy coding! 🦀