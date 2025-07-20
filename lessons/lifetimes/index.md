---
layout: default
title: Lifetimes in Rust
---

# Lifetimes in Rust

Lifetimes ensure that references are valid as long as we need them to be. They're Rust's way of preventing dangling references and ensuring memory safety at compile time.

## What You'll Learn

- What lifetimes are and why they exist
- Lifetime syntax and annotations
- Lifetime elision rules
- Common lifetime patterns
- Generic lifetimes in structs and functions
- How to resolve lifetime errors

## What Are Lifetimes?

Every reference in Rust has a lifetime - the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, but sometimes we need to specify them explicitly.

```rust
fn main() {
    let r;                    // r has lifetime 'a
    
    {
        let x = 5;           // x has lifetime 'b
        r = &x;              // Error! x's lifetime 'b is shorter than r's lifetime 'a
    }                        // x goes out of scope here
    
    // println!("r: {}", r); // Would be using r after x is dropped
}
```

## Why Lifetimes Matter

Lifetimes prevent dangling references:

```rust
fn main() {
    let string1 = String::from("abcd");
    let result;
    
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);  // Which lifetime should result have?
    }  // string2 is dropped here
    
    // println!("The longest string is {}", result);  // Potential use after free!
}

// This function needs lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Lifetime Syntax

Lifetime parameters are declared using apostrophes:

```rust
// 'a is a lifetime parameter
fn function_with_lifetime<'a>(x: &'a str) -> &'a str {
    x
}

// Multiple lifetime parameters
fn function_with_two_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
```

### Reading Lifetime Syntax

```rust
fn example<'a>(x: &'a str, y: &'a str) -> &'a str {
    // This function:
    // - Takes two string slices with the same lifetime 'a
    // - Returns a string slice with the same lifetime 'a
    // - Guarantees the returned reference will be valid as long as both inputs are valid
    
    if x.len() > y.len() { x } else { y }
}
```

## Lifetime Elision Rules

Rust has rules that allow it to infer lifetimes in many cases:

### Rule 1: Each Input Parameter Gets Its Own Lifetime

```rust
// What you write:
fn first_word(s: &str) -> &str {
    // implementation
}

// What the compiler sees:
fn first_word<'a>(s: &'a str) -> &'a str {
    // implementation
}
```

### Rule 2: If There's One Input Lifetime, It's Assigned to All Outputs

```rust
// What you write:
fn get_first_char(s: &str) -> Option<&str> {
    // implementation
}

// What the compiler sees:
fn get_first_char<'a>(s: &'a str) -> Option<&'a str> {
    // implementation
}
```

### Rule 3: If There's a `&self` or `&mut self`, Its Lifetime Is Assigned to All Outputs

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // What you write:
    fn level(&self) -> i32 {
        3
    }
    
    // What you write:
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
    
    // What the compiler sees:
    fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## Functions with Lifetimes

### Basic Example

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }  // result goes out of scope here, which is fine
}
```

### Different Lifetimes

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: {}", word);
}
```

### Functions That Don't Need Explicit Lifetimes

```rust
// Returns an owned String, so no lifetime needed
fn combine_strings(s1: &str, s2: &str) -> String {
    format!("{} {}", s1, s2)
}

// Takes a reference but returns an owned value
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// Static lifetime
fn get_static_str() -> &'static str {
    "This string lives for the entire program"
}
```

## Structs with Lifetimes

Structs can hold references, but they need lifetime annotations:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Important excerpt: {}", i.part);
}
```

### Multiple References in Structs

```rust
struct Book<'a, 'b> {
    title: &'a str,
    author: &'b str,
}

impl<'a, 'b> Book<'a, 'b> {
    fn new(title: &'a str, author: &'b str) -> Book<'a, 'b> {
        Book { title, author }
    }
    
    fn info(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }
}

fn main() {
    let title = String::from("The Rust Programming Language");
    let author = String::from("Steve Klabnik and Carol Nichols");
    
    let book = Book::new(&title, &author);
    println!("{}", book.info());
}
```

## The Static Lifetime

The `'static` lifetime means the reference can live for the entire program duration:

```rust
fn main() {
    // String literals have 'static lifetime
    let s: &'static str = "I have a static lifetime.";
    
    // Static variables
    static HELLO_WORLD: &str = "Hello, world!";
    
    println!("{}", s);
    println!("{}", HELLO_WORLD);
}

// Function returning static reference
fn get_version() -> &'static str {
    "1.0.0"
}
```

### When to Use Static Lifetime

```rust
use std::collections::HashMap;

// Global configuration that lives for the entire program
static CONFIG: &str = "production";

// Error messages that don't change
fn get_error_message(code: u32) -> &'static str {
    match code {
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Error",
    }
}

fn main() {
    println!("Config: {}", CONFIG);
    println!("Error: {}", get_error_message(404));
}
```

## Generic Lifetimes with Types

Combining lifetimes with generic types:

```rust
struct Container<'a, T> {
    value: &'a T,
}

impl<'a, T> Container<'a, T> {
    fn new(value: &'a T) -> Container<'a, T> {
        Container { value }
    }
    
    fn get_value(&self) -> &T {
        self.value
    }
}

fn main() {
    let number = 42;
    let container = Container::new(&number);
    println!("Value: {}", container.get_value());
    
    let text = String::from("hello");
    let text_container = Container::new(&text);
    println!("Text: {}", text_container.get_value());
}
```

## Advanced Lifetime Patterns

### Lifetime Bounds

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest_with_an_announcement(
        &string1,
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}
```

### Higher-Ranked Trait Bounds (HRTB)

```rust
fn apply_to_all<F>(f: F) 
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let strings = vec!["hello", "world", "rust"];
    for s in strings {
        println!("{}", f(s));
    }
}

fn add_prefix(s: &str) -> &str {
    // In reality, this would need to return an owned String
    // This is just for demonstration
    s
}
```

## Practical Examples

### Text Parser

```rust
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser { input, position: 0 }
    }
    
    fn parse_word(&mut self) -> Option<&'a str> {
        let start = self.position;
        
        while self.position < self.input.len() {
            let byte = self.input.as_bytes()[self.position];
            if byte == b' ' || byte == b'\n' {
                break;
            }
            self.position += 1;
        }
        
        if start == self.position {
            None
        } else {
            let word = &self.input[start..self.position];
            self.skip_whitespace();
            Some(word)
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let byte = self.input.as_bytes()[self.position];
            if byte != b' ' && byte != b'\n' {
                break;
            }
            self.position += 1;
        }
    }
}

fn main() {
    let text = "hello world rust programming";
    let mut parser = Parser::new(text);
    
    while let Some(word) = parser.parse_word() {
        println!("Parsed word: {}", word);
    }
}
```

### Configuration Manager

```rust
struct Config<'a> {
    host: &'a str,
    port: u16,
    database_url: &'a str,
}

impl<'a> Config<'a> {
    fn new(host: &'a str, port: u16, database_url: &'a str) -> Config<'a> {
        Config {
            host,
            port,
            database_url,
        }
    }
    
    fn connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    
    fn is_local(&self) -> bool {
        self.host == "localhost" || self.host == "127.0.0.1"
    }
}

fn create_config<'a>(
    host: &'a str,
    database_url: &'a str,
) -> Config<'a> {
    let port = if host == "localhost" { 8080 } else { 80 };
    Config::new(host, port, database_url)
}

fn main() {
    let host = String::from("localhost");
    let db_url = String::from("postgresql://localhost/mydb");
    
    let config = create_config(&host, &db_url);
    
    println!("Connection: {}", config.connection_string());
    println!("Is local: {}", config.is_local());
}
```

## Resolving Lifetime Errors

### Common Error: Borrowed Value Does Not Live Long Enough

```rust
// Problem
fn problematic_function() -> &str {
    let s = String::from("hello");
    &s  // Error: s doesn't live long enough
}

// Solution 1: Return owned value
fn fixed_function_v1() -> String {
    let s = String::from("hello");
    s
}

// Solution 2: Take input parameter
fn fixed_function_v2(s: &str) -> &str {
    s
}

// Solution 3: Use static
fn fixed_function_v3() -> &'static str {
    "hello"
}
```

### Common Error: Lifetime Mismatch

```rust
// Problem
fn problematic_longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// Solution: Add lifetime annotation
fn fixed_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Common Error: Cannot Borrow as Mutable

```rust
// Problem
fn problematic_modify() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;  // Error: cannot borrow as mutable
    println!("{}, {}", r1, r2);
}

// Solution: Separate scopes
fn fixed_modify() {
    let mut s = String::from("hello");
    
    {
        let r1 = &s;
        println!("{}", r1);
    }  // r1 goes out of scope
    
    let r2 = &mut s;  // Now this is OK
    r2.push_str(" world");
    println!("{}", r2);
}
```

## Best Practices

1. **Let the compiler infer lifetimes** when possible
2. **Use lifetime elision rules** to avoid explicit annotations
3. **Prefer owned types** over references in struct fields when possible
4. **Use `'static` sparingly** - only for truly global data
5. **Read compiler error messages carefully** - they're very helpful

## Common Patterns

### Optional References

```rust
struct OptionalRef<'a> {
    value: Option<&'a str>,
}

impl<'a> OptionalRef<'a> {
    fn new() -> OptionalRef<'a> {
        OptionalRef { value: None }
    }
    
    fn set(&mut self, value: &'a str) {
        self.value = Some(value);
    }
    
    fn get(&self) -> Option<&str> {
        self.value
    }
}
```

### Iterators with Lifetimes

```rust
struct Words<'a> {
    text: &'a str,
}

impl<'a> Words<'a> {
    fn new(text: &'a str) -> Words<'a> {
        Words { text }
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }
        
        if let Some(pos) = self.text.find(' ') {
            let word = &self.text[..pos];
            self.text = &self.text[pos + 1..];
            Some(word)
        } else {
            let word = self.text;
            self.text = "";
            Some(word)
        }
    }
}

fn main() {
    let text = "hello world rust";
    let words = Words::new(text);
    
    for word in words {
        println!("Word: {}", word);
    }
}
```

## Try It Yourself

Create a program that:
1. Defines a `Library` struct that holds references to book titles
2. Implements methods to add, find, and list books
3. Ensures all references are valid throughout the program
4. Handles different lifetime scenarios correctly

## Next Steps

Now that you understand lifetimes, explore:
- [Smart Pointers]({{ '/lessons/smart-pointers/' | relative_url }}) - Advanced ownership patterns
- [Traits]({{ '/lessons/traits/' | relative_url }}) - Shared behavior across types
- [Generics]({{ '/lessons/generics/' | relative_url }}) - Writing flexible, reusable code

Lifetimes are a powerful feature that ensures memory safety while allowing flexible reference patterns!