---
layout: default
title: Understanding Ownership
---

# Understanding Ownership

Ownership is Rust's most unique feature and enables memory safety without garbage collection. Understanding ownership is crucial to mastering Rust programming.

## What You'll Learn

- What ownership is and why it matters
- The ownership rules
- Variable scope and memory management
- The `move` semantics
- Copy vs. Move types
- How ownership prevents common memory bugs

## What is Ownership?

Ownership is a set of rules that govern how Rust manages memory. Instead of using a garbage collector or requiring manual memory management, Rust uses an ownership system with rules that the compiler checks at compile time.

### The Memory Problem

Traditional approaches to memory management have issues:

```rust
// C-style manual memory management
// malloc()/free() - easy to forget free() or double-free
// let ptr = malloc(sizeof(int));
// *ptr = 42;
// free(ptr);  // Easy to forget!

// Garbage collection
// Automatic but with runtime overhead
// Non-deterministic cleanup timing
```

Rust's ownership system provides:
- **Memory safety** without garbage collection
- **Zero-cost abstractions** - no runtime overhead
- **Deterministic** cleanup
- **Thread safety** by default

## The Ownership Rules

Rust's ownership system has three fundamental rules:

1. **Each value in Rust has an owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value will be dropped**

Let's explore each rule:

### Rule 1: Each Value Has an Owner

```rust
fn main() {
    let s = String::from("hello");  // s owns the String
    let x = 5;                      // x owns the integer
    
    // s is the owner of the String "hello"
    // x is the owner of the integer 5
}
```

### Rule 2: Only One Owner at a Time

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership moves from s1 to s2
    
    // println!("{}", s1);  // Error! s1 no longer owns the value
    println!("{}", s2);     // OK: s2 owns the value
}
```

### Rule 3: Value Dropped When Owner Goes Out of Scope

```rust
fn main() {
    {
        let s = String::from("hello");  // s comes into scope
        // do stuff with s
    }  // s goes out of scope and is dropped (memory freed)
    
    // println!("{}", s);  // Error! s is no longer in scope
}
```

## Variable Scope

Scope is the range within a program where a variable is valid:

```rust
fn main() {
    // s is not valid here - not yet declared
    
    {
        let s = String::from("hello");   // s is valid from this point forward
        println!("{}", s);               // s is valid here
    }                                    // scope ends, s is dropped
    
    // s is not valid here - out of scope
}
```

## Memory and Allocation

Different data types are stored differently:

### Stack vs. Heap

```rust
fn main() {
    // Stack data - size known at compile time
    let x = 5;        // i32, stored on stack
    let y = true;     // bool, stored on stack
    let z = 'c';      // char, stored on stack
    
    // Heap data - size unknown or variable at compile time
    let s = String::from("hello");  // String data stored on heap
    let v = vec![1, 2, 3];          // Vec data stored on heap
}
```

### The String Type

Strings demonstrate ownership principles well:

```rust
fn main() {
    // String literals - stored in program binary
    let s1 = "hello";  // &str - immutable, fixed size
    
    // String type - stored on heap, mutable, growable
    let mut s2 = String::from("hello");
    s2.push_str(", world!");  // Can grow
    println!("{}", s2);
}
```

## Move Semantics

When ownership transfers, we call it a "move":

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // Move: s1's ownership moves to s2
    
    // s1 is no longer valid
    // This prevents double-free errors
    
    println!("{}", s2);  // OK
    // println!("{}", s1);  // Error: value used after move
}
```

### Why Moves Happen

```rust
fn main() {
    let s1 = String::from("hello");
    
    // If Rust allowed both s1 and s2 to own the same data:
    let s2 = s1;  // Hypothetical copy
    
    // When s1 and s2 go out of scope, both would try to free
    // the same memory - double free error!
    
    // Rust prevents this by moving ownership
}
```

### Functions and Ownership

```rust
fn main() {
    let s = String::from("hello");
    
    takes_ownership(s);  // s's value moves into the function
    // s is no longer valid here
    
    let x = 5;
    makes_copy(x);       // x is copied (integers implement Copy)
    // x is still valid here
    
    println!("x is still: {}", x);  // OK
    // println!("s is: {}", s);     // Error: s was moved
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}  // some_integer goes out of scope, but it's a copy so no problem
```

## Copy vs. Move Types

Some types implement the `Copy` trait, which means they're copied instead of moved:

### Copy Types

```rust
fn main() {
    // These types implement Copy - they're copied, not moved
    let x = 5;
    let y = x;      // x is copied to y
    println!("x: {}, y: {}", x, y);  // Both are valid
    
    let a = true;
    let b = a;      // a is copied to b
    println!("a: {}, b: {}", a, b);  // Both are valid
    
    let c = 'z';
    let d = c;      // c is copied to d
    println!("c: {}, d: {}", c, d);  // Both are valid
}
```

### Copy Types Include:
- All integer types (`i32`, `u32`, etc.)
- Boolean type (`bool`)
- Character type (`char`)
- Floating-point types (`f32`, `f64`)
- Tuples containing only Copy types

### Non-Copy Types

```rust
fn main() {
    // These types do NOT implement Copy - they're moved
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // Error: s1 was moved
    
    let v1 = vec![1, 2, 3];
    let v2 = v1;  // v1 is moved to v2
    // println!("{:?}", v1);  // Error: v1 was moved
}
```

## Return Values and Scope

Functions can transfer ownership through return values:

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, 
                                        // which also moves its return value into s3
    
    println!("s1: {}", s1);
    // println!("s2: {}", s2);  // Error: s2 was moved
    println!("s3: {}", s3);
    
} // s3 goes out of scope and is dropped. s2 was moved, so nothing happens. 
  // s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("yours");  // some_string comes into scope
    some_string                               // some_string is returned and moves out
}

fn takes_and_gives_back(a_string: String) -> String {  // a_string comes into scope
    a_string  // a_string is returned and moves out of calling function
}
```

## Clone for Deep Copies

Sometimes you want to actually copy heap data:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Expensive deep copy
    
    println!("s1: {}, s2: {}", s1, s2);  // Both are valid
}
```

### When to Use Clone

```rust
fn main() {
    let original = String::from("important data");
    
    // If you need the original after passing to a function
    let backup = original.clone();
    process_string(original);  // original is moved
    
    // You still have backup
    println!("Backup: {}", backup);
}

fn process_string(s: String) {
    println!("Processing: {}", s);
    // s is dropped here
}
```

## Practical Examples

### Safe String Processing

```rust
fn main() {
    let mut data = String::from("hello");
    
    // Method 1: Move and return
    data = add_world(data);
    println!("Result: {}", data);
    
    // Method 2: Clone when you need to keep original
    let original = String::from("rust");
    let modified = add_exclamation(original.clone());
    println!("Original: {}, Modified: {}", original, modified);
}

fn add_world(mut s: String) -> String {
    s.push_str(" world");
    s  // Return ownership
}

fn add_exclamation(mut s: String) -> String {
    s.push('!');
    s
}
```

### Vector Ownership

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    
    println!("Original: {:?}", numbers);
    
    // Move to function and get back
    numbers = double_values(numbers);
    println!("Doubled: {:?}", numbers);
    
    // Move to function that consumes
    let sum = sum_vector(numbers);
    println!("Sum: {}", sum);
    
    // numbers is no longer available
    // println!("{:?}", numbers);  // Error: numbers was moved
}

fn double_values(mut vec: Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() {
        vec[i] *= 2;
    }
    vec
}

fn sum_vector(vec: Vec<i32>) -> i32 {
    vec.iter().sum()
}
```

### Building a Simple Library

```rust
struct Library {
    books: Vec<String>,
}

impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }
    
    fn add_book(&mut self, book: String) {
        self.books.push(book);  // book is moved into the vector
    }
    
    fn remove_book(&mut self, title: &str) -> Option<String> {
        if let Some(pos) = self.books.iter().position(|x| x == title) {
            Some(self.books.remove(pos))  // Remove and return ownership
        } else {
            None
        }
    }
    
    fn list_books(&self) -> &Vec<String> {
        &self.books  // Return a reference, not ownership
    }
}

fn main() {
    let mut library = Library::new();
    
    // Add books (ownership transferred to library)
    library.add_book(String::from("The Rust Book"));
    library.add_book(String::from("Programming Rust"));
    
    println!("Books: {:?}", library.list_books());
    
    // Remove a book (ownership transferred back)
    if let Some(book) = library.remove_book("The Rust Book") {
        println!("Removed: {}", book);
        // book is dropped here
    }
    
    println!("Remaining books: {:?}", library.list_books());
}
```

## Common Ownership Patterns

### Builder Pattern

```rust
struct Config {
    host: String,
    port: u16,
    secure: bool,
}

impl Config {
    fn new() -> Config {
        Config {
            host: String::from("localhost"),
            port: 8080,
            secure: false,
        }
    }
    
    fn host(mut self, host: String) -> Config {
        self.host = host;
        self  // Return ownership for chaining
    }
    
    fn port(mut self, port: u16) -> Config {
        self.port = port;
        self
    }
    
    fn secure(mut self, secure: bool) -> Config {
        self.secure = secure;
        self
    }
}

fn main() {
    let config = Config::new()
        .host(String::from("example.com"))
        .port(443)
        .secure(true);
    
    println!("Host: {}, Port: {}, Secure: {}", 
             config.host, config.port, config.secure);
}
```

## Debugging Ownership Issues

### Common Error Messages

```rust
fn main() {
    let s = String::from("hello");
    let s2 = s;
    
    // Error: borrow of moved value: `s`
    // println!("{}", s);
    
    // Solution 1: Use s2
    println!("{}", s2);
    
    // Solution 2: Clone before move
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
}
```

### Using the Compiler's Help

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 moved here
    
    // The compiler will tell you exactly where the move happened
    // and suggest solutions
    
    // println!("{}", s1);  // Error with helpful message
}
```

## Best Practices

1. **Understand when moves happen** - assignment, function calls, returns
2. **Use references when you don't need ownership** (covered in next lesson)
3. **Clone sparingly** - it's expensive for heap data
4. **Design APIs to minimize unnecessary ownership transfers**
5. **Read compiler errors carefully** - they're very helpful

## Common Mistakes

1. **Using values after move**:
```rust
let s1 = String::from("hello");
let s2 = s1;
// println!("{}", s1);  // Error!
```

2. **Forgetting that function calls move values**:
```rust
let s = String::from("hello");
takes_ownership(s);
// println!("{}", s);  // Error!
```

3. **Unnecessary cloning**:
```rust
// Inefficient
fn process_string(s: String) -> String {
    // just returning it unchanged
    s
}

let s1 = String::from("hello");
let s2 = process_string(s1.clone());  // Unnecessary clone!

// Better: just move it
let s1 = String::from("hello");
let s2 = process_string(s1);
```

## Try It Yourself

Create a program that:
1. Creates a vector of strings
2. Moves it to a function that reverses the order
3. Returns the vector and prints it
4. Moves it to another function that counts the total characters
5. Handles ownership correctly throughout

## Next Steps

Now that you understand ownership, you're ready to learn about:
- [References and Borrowing]({{ '/lessons/references-borrowing/' | relative_url }}) - Using data without taking ownership
- [Lifetimes]({{ '/lessons/lifetimes/' | relative_url }}) - Ensuring references are valid
- [Smart Pointers]({{ '/lessons/smart-pointers/' | relative_url }}) - Advanced ownership patterns

Ownership is the foundation of Rust's memory safety guarantees!