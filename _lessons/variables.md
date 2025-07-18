---
title: Variables and Mutability
difficulty: beginner
version: 1.85.0
prev_lesson: /lessons/cargo/
prev_lesson_title: Cargo Package Manager
next_lesson: /lessons/data-types/
next_lesson_title: Data Types
---

# Variables and Mutability

In Rust, variables work differently than in many other programming languages. By default, variables are **immutable**, meaning their values cannot be changed after they're set. This is one of Rust's key features that helps write safe and concurrent code.

## Creating Variables

You create variables using the `let` keyword:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
}
```

## Immutability by Default

Once a value is bound to a variable, you can't change it:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This will cause a compile error!
}
```

If you try to compile this code, you'll get an error:

```
error[E0384]: cannot assign twice to immutable variable `x`
```

This might seem restrictive, but immutability has many benefits:
- Prevents accidental changes to data
- Makes code easier to reason about
- Enables safe concurrency
- Helps the compiler optimize your code

## Making Variables Mutable

When you need to change a variable's value, use the `mut` keyword:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Output:
```
The value of x is: 5
The value of x is: 6
```

## Variable Shadowing

Rust allows you to declare a new variable with the same name as a previous variable. This is called **shadowing**:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x); // Prints: 12
}
```

Shadowing is different from mutability:

### Shadowing vs Mutability

```rust
fn main() {
    // Shadowing - creating new variables
    let spaces = "   ";           // string
    let spaces = spaces.len();    // number
    
    // This is allowed because we're creating new variables
    println!("Number of spaces: {}", spaces);
}
```

But this won't work with `mut`:

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len(); // Error! Can't change type
}
```

Shadowing allows you to:
- Change the type of a variable
- Reuse variable names
- Keep variables immutable

## Constants

Constants are values that are bound to a name and cannot change. They differ from immutable variables:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}
```

Key differences between constants and variables:
- Constants use `const` instead of `let`
- You must always annotate the type
- Constants can be declared in any scope, including global
- Constants can only be set to constant expressions, not runtime values
- Constants are written in SCREAMING_SNAKE_CASE by convention

## Practical Examples

### Counter Example

```rust
fn main() {
    let mut count = 0;
    
    for i in 1..=5 {
        count += i;
        println!("Count is now: {}", count);
    }
    
    println!("Final count: {}", count);
}
```

### Name Processing Example

```rust
fn main() {
    let name = "john doe";
    println!("Original name: {}", name);
    
    // Shadow to transform the name
    let name = name.to_uppercase();
    println!("Uppercase name: {}", name);
    
    // Shadow again to get the length
    let name = name.len();
    println!("Name length: {}", name);
}
```

### Configuration Example

```rust
const MAX_USERS: u32 = 100;
const TIMEOUT_SECONDS: u32 = 30;

fn main() {
    let mut current_users = 0;
    
    // Simulate adding users
    for _ in 0..5 {
        current_users += 1;
        println!("Current users: {}/{}", current_users, MAX_USERS);
    }
    
    println!("Timeout set to {} seconds", TIMEOUT_SECONDS);
}
```

## Best Practices

### Use Immutable Variables When Possible

```rust
fn main() {
    // Good - value doesn't change
    let message = "Hello, Rust!";
    println!("{}", message);
    
    // Good - only make mutable when necessary
    let mut counter = 0;
    counter += 1;
}
```

### Use Descriptive Variable Names

```rust
fn main() {
    // Good
    let user_age = 25;
    let is_logged_in = true;
    let user_email = "user@example.com";
    
    // Avoid
    let x = 25;
    let flag = true;
    let s = "user@example.com";
}
```

### Use Constants for Fixed Values

```rust
const PI: f64 = 3.14159;
const COMPANY_NAME: &str = "Rust Corp";

fn main() {
    let radius = 5.0;
    let area = PI * radius * radius;
    println!("Area: {}", area);
    println!("Welcome to {}!", COMPANY_NAME);
}
```

## Common Patterns

### Initialization Then Assignment

```rust
fn main() {
    let x; // Declare without initializing
    
    if true {
        x = 5; // Initialize in a branch
    } else {
        x = 10;
    }
    
    println!("x is: {}", x); // x is guaranteed to be initialized here
}
```

### Temporary Calculations

```rust
fn main() {
    let input = "42";
    
    // Parse and handle in separate steps
    let number = input.parse::<i32>().unwrap_or(0);
    let result = number * 2;
    
    println!("Double of {} is {}", number, result);
}
```

## Memory and Performance

Immutable variables don't add runtime overhead. The immutability is enforced at compile time, so there's no performance cost compared to mutable variables.

```rust
fn main() {
    let x = 5;          // No runtime cost for immutability
    let y = x + 1;      // Simple computation
    println!("{}", y);
}
```

## Key Takeaways

- Variables are immutable by default in Rust
- Use `mut` to make variables mutable when needed
- Shadowing allows reusing variable names and changing types
- Constants are always immutable and must have explicit types
- Prefer immutable variables when possible for safer code
- Immutability is enforced at compile time with no runtime cost

## Next Steps

Now that you understand variables and mutability, let's explore [Data Types]({{ '/lessons/data-types/' | relative_url }}) to learn about the different kinds of data you can store in variables.

## Try It Yourself

1. Create a program that calculates the area of a rectangle using immutable variables
2. Write a counter that increments from 1 to 10 using a mutable variable
3. Use shadowing to convert a string to uppercase, then get its length
4. Define constants for your favorite number and color, then print them

Happy coding! 🦀