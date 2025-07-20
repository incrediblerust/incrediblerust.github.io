---
layout: default
title: Functions in Rust
---

# Functions in Rust

Functions are fundamental building blocks in Rust programming. They let you organize code into reusable blocks and create cleaner, more maintainable programs.

## What You'll Learn

- How to define and call functions
- Function parameters and return values
- Rust's expression-based syntax
- Function scope and ownership
- Best practices for function design

## Function Basics

In Rust, functions are defined using the `fn` keyword:

```rust
fn main() {
    println!("Hello, world!");
    greet();
}

fn greet() {
    println!("Welcome to Rust functions!");
}
```

### Key Function Rules

1. **Snake case naming**: Use `snake_case` for function names
2. **Order doesn't matter**: Functions can be defined before or after they're called
3. **Main function**: Every executable Rust program must have a `main` function

## Function Parameters

Functions can accept parameters to make them more flexible:

```rust
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

fn add_numbers(x: i32, y: i32) {
    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}

fn main() {
    greet_person("Alice");
    add_numbers(5, 3);
}
```

### Parameter Requirements

- **Type annotations required**: You must specify the type of each parameter
- **Multiple parameters**: Separate with commas
- **Immutable by default**: Parameters are immutable unless specified otherwise

## Return Values

Functions can return values to their callers:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // No semicolon - this is an expression
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;  // Explicit return with semicolon
}

fn get_greeting() -> String {
    String::from("Hello from Rust!")
}

fn main() {
    let sum = add(5, 3);
    let product = multiply(4, 7);
    let message = get_greeting();
    
    println!("Sum: {}", sum);
    println!("Product: {}", product);
    println!("Message: {}", message);
}
```

### Return Value Rules

1. **Type annotation required**: Use `->` followed by the return type
2. **Expression vs Statement**: 
   - Last expression without semicolon is returned
   - `return` keyword for early returns
3. **Unit type**: Functions with no return value return `()` (unit type)

## Expressions vs Statements

Understanding the difference is crucial in Rust:

```rust
fn expression_example() -> i32 {
    let x = 5;
    let y = {
        let inner = 3;
        inner + 1  // Expression - no semicolon
    };
    
    x + y  // Expression returned - no semicolon
}

fn statement_example() {
    let x = 5;  // Statement
    let y = 6;  // Statement
    
    // This would be an error:
    // let z = (let a = 6);  // Statements don't return values
}
```

## Function Scope and Ownership

Functions in Rust follow ownership rules:

```rust
fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, but it's a copy

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // Return value transfers ownership
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // Return value transfers ownership
}

fn main() {
    let s = String::from("hello");
    take_ownership(s); // s's value moves into the function
    // s is no longer valid here
    
    let x = 5;
    makes_copy(x); // x would still be valid after this
    
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // s2 is moved into takes_and_gives_back, s3 gets return value
}
```

## Practical Examples

### Calculator Functions

```rust
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        panic!("Cannot divide by zero!");
    }
}

fn main() {
    let x = 10.0;
    let y = 3.0;
    
    println!("{} + {} = {}", x, y, add(x, y));
    println!("{} - {} = {}", x, y, subtract(x, y));
    println!("{} * {} = {}", x, y, multiply(x, y));
    println!("{} / {} = {}", x, y, divide(x, y));
}
```

### Text Processing Functions

```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn reverse_string(text: &str) -> String {
    text.chars().rev().collect()
}

fn is_palindrome(text: &str) -> bool {
    let cleaned: String = text.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    let sentence = "Hello Rust world";
    println!("'{}' has {} words", sentence, count_words(sentence));
    println!("Reversed: '{}'", reverse_string(sentence));
    
    let word = "racecar";
    println!("'{}' is palindrome: {}", word, is_palindrome(word));
}
```

## Best Practices

1. **Keep functions small**: Each function should do one thing well
2. **Use descriptive names**: Function names should clearly indicate their purpose
3. **Minimize parameters**: Too many parameters make functions hard to use
4. **Consider borrowing**: Use references (`&`) to avoid unnecessary ownership transfers
5. **Document public functions**: Use `///` comments for public API documentation

```rust
/// Calculates the area of a rectangle.
/// 
/// # Arguments
/// 
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
/// 
/// # Examples
/// 
/// ```
/// let area = calculate_area(5.0, 3.0);
/// assert_eq!(area, 15.0);
/// ```
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
```

## Common Mistakes

1. **Forgetting return type annotation**:
```rust
// Wrong
fn add(x: i32, y: i32) { x + y }

// Correct
fn add(x: i32, y: i32) -> i32 { x + y }
```

2. **Adding semicolon to return expression**:
```rust
// Wrong - returns (), not i32
fn add(x: i32, y: i32) -> i32 { x + y; }

// Correct
fn add(x: i32, y: i32) -> i32 { x + y }
```

3. **Ownership confusion**:
```rust
// Wrong - s is moved and can't be used after
fn main() {
    let s = String::from("hello");
    take_ownership(s);
    println!("{}", s); // Error!
}

// Correct - use borrowing
fn main() {
    let s = String::from("hello");
    print_string(&s);
    println!("{}", s); // Works!
}

fn print_string(s: &String) {
    println!("{}", s);
}
```

## Try It Yourself

Create a function that:
1. Takes a vector of integers as a parameter
2. Returns the largest number in the vector
3. Handles empty vectors gracefully

## Next Steps

Now that you understand functions, you're ready to learn about:
- [Control Flow]({{ '/lessons/control-flow/' | relative_url }}) - Making decisions in your code
- [Structs]({{ '/lessons/structs/' | relative_url }}) - Grouping related data and functions
- [Error Handling]({{ '/lessons/error-handling/' | relative_url }}) - Dealing with failures gracefully

Functions are the building blocks that will support all the advanced concepts you'll learn in Rust!