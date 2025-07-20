---
layout: default
title: References and Borrowing
---

# References and Borrowing

References allow you to use a value without taking ownership of it. Borrowing is the act of creating a reference. This enables flexible and efficient code while maintaining memory safety.

## What You'll Learn

- What references are and how they work
- Immutable and mutable references
- Borrowing rules that prevent data races
- Reference scope and lifetime basics
- Common borrowing patterns
- How to avoid common borrowing errors

## What Are References?

A reference is like a pointer to data owned by another variable. References allow you to refer to some value without taking ownership of it.

```rust
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);  // &s1 creates a reference
    
    println!("The length of '{}' is {}.", s1, len);  // s1 is still valid!
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}  // s goes out of scope, but it doesn't own the data, so nothing is dropped
```

### Reference Syntax

```rust
fn main() {
    let x = 5;
    let y = &x;  // y is a reference to x
    
    println!("x = {}", x);
    println!("y = {}", y);   // Prints the value y points to
    println!("y = {:p}", y); // Prints the memory address
    
    // Dereferencing
    println!("*y = {}", *y); // Use * to access the value y points to
    
    assert_eq!(5, x);
    assert_eq!(5, *y);  // Must dereference to compare values
}
```

## Immutable References

By default, references are immutable:

```rust
fn main() {
    let s = String::from("hello");
    
    // Multiple immutable references are allowed
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("{}, {}, and {}", r1, r2, r3);
    
    // This would not compile:
    // change(&s);  // Error: cannot borrow as mutable
}

fn cannot_change(some_string: &String) {
    // some_string.push_str(", world");  // Error: cannot modify
}
```

### Function Parameters

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    // Both strings remain valid after the function call
    let result = concatenate(&s1, &s2);
    println!("Result: {}", result);
    println!("Original strings: '{}' and '{}'", s1, s2);
}

fn concatenate(s1: &String, s2: &String) -> String {
    format!("{} {}", s1, s2)
}
```

## Mutable References

To modify data through a reference, use mutable references:

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);  // &mut creates a mutable reference
    
    println!("{}", s);  // Prints "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Mutable Reference Restrictions

Rust has strict rules for mutable references to prevent data races:

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;  // First mutable reference
    // let r2 = &mut s;  // Error: cannot borrow `s` as mutable more than once
    
    println!("{}", r1);
    
    // Now we can create another mutable reference
    let r2 = &mut s;
    println!("{}", r2);
}
```

### The Borrowing Rules

1. **At any given time, you can have either one mutable reference OR any number of immutable references**
2. **References must always be valid**

```rust
fn main() {
    let mut s = String::from("hello");
    
    // Multiple immutable references are OK
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    // Now we can create a mutable reference
    let r3 = &mut s;
    println!("{}", r3);
}
```

### Why These Rules Exist

The borrowing rules prevent data races at compile time:

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    
    // This would be problematic if allowed:
    // let immutable = &data;        // Immutable reference
    // let mutable = &mut data;      // Mutable reference (Error!)
    // 
    // data.push(4);                 // mutable modifies data
    // println!("{:?}", immutable);  // immutable might see inconsistent state
}
```

## Reference Scope

References follow scope rules:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // no problem
    let r2 = &s;     // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

### Non-Lexical Lifetimes

Modern Rust is smart about when references are last used:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are not used after this point

    let r3 = &mut s;  // This is now OK!
    println!("{}", r3);
}
```

## Dangling References

Rust prevents dangling references at compile time:

```rust
fn main() {
    // let reference_to_nothing = dangle();  // This won't compile
}

// fn dangle() -> &String {  // Error: missing lifetime specifier
//     let s = String::from("hello");
//     &s  // We return a reference to s, but s is dropped when function ends
// }

// Correct version - return the String itself, transferring ownership
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Return the String, ownership is transferred
}
```

## Practical Examples

### String Processing

```rust
fn main() {
    let text = String::from("Hello, Rust World!");
    
    println!("Length: {}", get_length(&text));
    println!("First word: {}", first_word(&text));
    println!("Contains 'Rust': {}", contains_word(&text, "Rust"));
    
    // text is still available for use
    println!("Original: {}", text);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn contains_word(s: &String, word: &str) -> bool {
    s.contains(word)
}
```

### Modifying Data

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    println!("Before: {:?}", numbers);
    
    double_values(&mut numbers);
    println!("After doubling: {:?}", numbers);
    
    add_value(&mut numbers, 10);
    println!("After adding 10: {:?}", numbers);
    
    let sum = calculate_sum(&numbers);
    println!("Sum: {}", sum);
}

fn double_values(vec: &mut Vec<i32>) {
    for value in vec.iter_mut() {
        *value *= 2;
    }
}

fn add_value(vec: &mut Vec<i32>, value: i32) {
    vec.push(value);
}

fn calculate_sum(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}
```

### Working with Structs

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  // &self is a reference to self
        self.width * self.height
    }
    
    fn set_width(&mut self, width: u32) {  // &mut self for modification
        self.width = width;
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    println!("Area of rect1: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    rect1.set_width(60);
    println!("After resize: {:?}", rect1);
}
```

## Advanced Borrowing Patterns

### Multiple Mutable References to Different Parts

```rust
fn main() {
    let mut data = [1, 2, 3, 4, 5];
    
    // Split mutable slice into two parts
    let (left, right) = data.split_at_mut(2);
    
    left[0] = 10;
    right[0] = 30;
    
    println!("Data: {:?}", data);  // [10, 2, 30, 4, 5]
}
```

### Interior Mutability Preview

```rust
use std::cell::RefCell;

fn main() {
    // RefCell allows "interior mutability" - mutation through shared reference
    let data = RefCell::new(vec![1, 2, 3]);
    
    {
        let mut borrowed = data.borrow_mut();
        borrowed.push(4);
    }  // Mutable borrow ends here
    
    println!("{:?}", data.borrow());  // [1, 2, 3, 4]
}
```

### Function That Returns References

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(&string1, string2);
    println!("The longest string is '{}'", result);
}
```

## Common Patterns

### Builder Pattern with References

```rust
struct QueryBuilder {
    query: String,
}

impl QueryBuilder {
    fn new() -> Self {
        QueryBuilder {
            query: String::new(),
        }
    }
    
    fn select(&mut self, fields: &str) -> &mut Self {
        self.query.push_str("SELECT ");
        self.query.push_str(fields);
        self
    }
    
    fn from(&mut self, table: &str) -> &mut Self {
        self.query.push_str(" FROM ");
        self.query.push_str(table);
        self
    }
    
    fn where_clause(&mut self, condition: &str) -> &mut Self {
        self.query.push_str(" WHERE ");
        self.query.push_str(condition);
        self
    }
    
    fn build(&self) -> &str {
        &self.query
    }
}

fn main() {
    let mut builder = QueryBuilder::new();
    
    let query = builder
        .select("name, age")
        .from("users")
        .where_clause("age > 18")
        .build();
    
    println!("Query: {}", query);
}
```

### Validation Functions

```rust
fn validate_email(email: &str) -> Result<(), &'static str> {
    if email.contains('@') && email.contains('.') {
        Ok(())
    } else {
        Err("Invalid email format")
    }
}

fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() >= 8 {
        Ok(())
    } else {
        Err("Password must be at least 8 characters")
    }
}

fn main() {
    let email = "user@example.com";
    let password = "mypassword123";
    
    match validate_email(email) {
        Ok(()) => println!("Email is valid"),
        Err(e) => println!("Email error: {}", e),
    }
    
    match validate_password(password) {
        Ok(()) => println!("Password is valid"),
        Err(e) => println!("Password error: {}", e),
    }
}
```

## str vs String

Understanding the difference between `String` and `&str`:

```rust
fn main() {
    // String - owned, heap-allocated, mutable
    let owned_string = String::from("hello");
    
    // &str - borrowed, can point to string literals or String slices
    let string_slice: &str = "world";
    let borrowed_slice: &str = &owned_string;
    
    // Function that accepts string slices is more flexible
    print_str("literal");
    print_str(&owned_string);
    print_str(string_slice);
    
    // Function that accepts String is more restrictive
    print_string(owned_string);  // Moves ownership
    // print_string(string_slice);  // Error: expected String, found &str
}

fn print_str(s: &str) {  // Can accept both &str and &String
    println!("{}", s);
}

fn print_string(s: String) {  // Only accepts String
    println!("{}", s);
}
```

## Best Practices

1. **Prefer &str over String for function parameters** when you don't need ownership
2. **Use references by default** unless you need ownership
3. **Keep mutable borrows short** to allow other references
4. **Understand when references end** (non-lexical lifetimes)
5. **Use slice syntax** for borrowing parts of collections

## Common Mistakes

1. **Mixing mutable and immutable references**:
```rust
let mut s = String::from("hello");
let r1 = &s;           // Immutable reference
// let r2 = &mut s;    // Error: cannot borrow as mutable while immutable reference exists
```

2. **References lasting too long**:
```rust
let mut s = String::from("hello");
let r = &s;
s.push_str(" world");  // Error: cannot modify while borrowed
println!("{}", r);
```

3. **Returning references to local variables**:
```rust
// fn bad_function() -> &String {  // Error: missing lifetime
//     let s = String::from("hello");
//     &s  // s is dropped when function ends
// }
```

4. **Unnecessary String conversions**:
```rust
// Inefficient
fn process_text(text: String) -> String {
    text.to_uppercase()
}

// Better - use references
fn process_text(text: &str) -> String {
    text.to_uppercase()
}
```

## Try It Yourself

Create a program that:
1. Defines a struct for a `Book` with title and author
2. Implements methods that use different types of references (&self, &mut self)
3. Creates functions that borrow books and return information about them
4. Demonstrates that you can have multiple immutable references but only one mutable reference

## Next Steps

Now that you understand references and borrowing, explore:
- [Slices]({{ '/lessons/slices/' | relative_url }}) - References to contiguous sequences
- [Lifetimes]({{ '/lessons/lifetimes/' | relative_url }}) - Ensuring references are valid
- [Smart Pointers]({{ '/lessons/smart-pointers/' | relative_url }}) - Advanced reference types

References and borrowing are fundamental to writing efficient, safe Rust code!