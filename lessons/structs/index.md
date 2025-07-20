---
layout: default
title: Structs in Rust
---

# Structs in Rust

Structs (structures) let you create custom data types that group related pieces of data together. They're fundamental to organizing code and modeling real-world concepts in Rust.

## What You'll Learn

- How to define and instantiate structs
- Different types of structs
- Method syntax and implementation blocks
- Associated functions
- Ownership and borrowing with structs
- Practical struct patterns

## Defining Structs

A struct is defined with the `struct` keyword:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("User: {} ({})", user1.username, user1.email);
}
```

### Accessing Struct Fields

Use dot notation to access struct fields:

```rust
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // Read fields
    println!("Username: {}", user1.username);
    
    // Modify fields (struct must be mutable)
    user1.email = String::from("anotheremail@example.com");
    user1.sign_in_count += 1;
}
```

## Creating Instances

### Constructor Functions

It's common to create functions that return struct instances:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,              // Field init shorthand
        username,           // Same as username: username
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("user@example.com"),
        String::from("user123"),
    );
}
```

### Struct Update Syntax

Create new instances from existing ones:

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // Copy remaining fields from user1
    };
    
    // Note: user1 is no longer valid because username was moved
    // println!("{}", user1.username); // This would error
}
```

## Tuple Structs

Structs that look like tuples but have names:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Access by index
    println!("Red component: {}", black.0);
    println!("X coordinate: {}", origin.0);
    
    // Color and Point are different types even with same data
    // let point_color: Point = black; // This would error
}
```

## Unit-Like Structs

Structs with no fields, useful for implementing traits:

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    
    // Useful when you need a type but no data
}
```

## Methods

Define methods on structs using `impl` blocks:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method that modifies the struct
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // Method that takes ownership
    fn into_square(mut self) -> Rectangle {
        let size = self.width.max(self.height);
        self.width = size;
        self.height = size;
        self
    }
    
    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Area: {}", rect1.area());
    
    rect1.double_size();
    println!("After doubling: {:?}", rect1);
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
```

## Associated Functions

Functions associated with structs but don't take `self`:

```rust
impl Rectangle {
    // Associated function (like a static method)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Another associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Call with :: syntax
    let rect1 = Rectangle::new(10, 20);
    let square = Rectangle::square(15);
    
    println!("Rectangle: {:?}", rect1);
    println!("Square: {:?}", square);
}
```

## Multiple impl Blocks

You can have multiple `impl` blocks for the same struct:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// Both methods are available on Rectangle instances
```

## Practical Examples

### Person Struct

```rust
#[derive(Debug, Clone)]
struct Person {
    first_name: String,
    last_name: String,
    age: u32,
    email: String,
}

impl Person {
    fn new(first_name: String, last_name: String, age: u32, email: String) -> Person {
        Person {
            first_name,
            last_name,
            age,
            email,
        }
    }
    
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("{} is now {} years old!", self.full_name(), self.age);
    }
    
    fn introduce(&self) -> String {
        format!(
            "Hi, I'm {} and I'm {} years old. You can reach me at {}",
            self.full_name(),
            self.age,
            self.email
        )
    }
}

fn main() {
    let mut person = Person::new(
        String::from("Alice"),
        String::from("Johnson"),
        25,
        String::from("alice@example.com"),
    );
    
    println!("{}", person.introduce());
    println!("Is adult: {}", person.is_adult());
    
    person.have_birthday();
}
```

### Bank Account

```rust
#[derive(Debug)]
struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: String, holder_name: String) -> BankAccount {
        BankAccount {
            account_number,
            holder_name,
            balance: 0.0,
        }
    }
    
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        
        self.balance += amount;
        Ok(())
    }
    
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        
        self.balance -= amount;
        Ok(())
    }
    
    fn get_balance(&self) -> f64 {
        self.balance
    }
    
    fn transfer(&mut self, to_account: &mut BankAccount, amount: f64) -> Result<(), String> {
        self.withdraw(amount)?;
        to_account.deposit(amount)?;
        Ok(())
    }
}

fn main() {
    let mut account1 = BankAccount::new(
        String::from("12345"),
        String::from("Alice"),
    );
    
    let mut account2 = BankAccount::new(
        String::from("67890"),
        String::from("Bob"),
    );
    
    // Deposit money
    account1.deposit(1000.0).unwrap();
    println!("Alice's balance: ${:.2}", account1.get_balance());
    
    // Transfer money
    match account1.transfer(&mut account2, 250.0) {
        Ok(()) => println!("Transfer successful"),
        Err(e) => println!("Transfer failed: {}", e),
    }
    
    println!("Alice's balance: ${:.2}", account1.get_balance());
    println!("Bob's balance: ${:.2}", account2.get_balance());
}
```

### Generic Point

```rust
#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// Implementation specific to certain types
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Point<i32> {
    fn manhattan_distance(&self, other: &Point<i32>) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("Integer point: {:?}", integer_point);
    println!("Float point: {:?}", float_point);
    
    println!("Distance from origin: {:.2}", float_point.distance_from_origin());
    
    let another_point = Point::new(1, 2);
    println!("Manhattan distance: {}", integer_point.manhattan_distance(&another_point));
}
```

## Ownership and Borrowing

### Structs and Ownership

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: String, author: String, pages: u32) -> Book {
        Book { title, author, pages }
    }
    
    // Borrowing (most common)
    fn summary(&self) -> String {
        format!("'{}' by {} ({} pages)", self.title, self.author, self.pages)
    }
    
    // Mutable borrowing
    fn add_pages(&mut self, additional_pages: u32) {
        self.pages += additional_pages;
    }
    
    // Taking ownership
    fn into_title(self) -> String {
        self.title  // Book is consumed, only title is returned
    }
}

fn main() {
    let mut book = Book::new(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik and Carol Nichols"),
        560,
    );
    
    println!("{}", book.summary());  // Borrowing
    book.add_pages(40);             // Mutable borrowing
    println!("{}", book.summary());
    
    let title = book.into_title();  // Book is consumed
    println!("Title: {}", title);
    // println!("{:?}", book);      // Error: book was moved
}
```

## Debugging and Display

### Debug Trait

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };
    
    println!("{:?}", user);    // Debug output
    println!("{:#?}", user);   // Pretty debug output
}
```

### Custom Display

```rust
use std::fmt;

struct User {
    username: String,
    email: String,
    active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User: {} ({})", self.username, self.email)
    }
}

fn main() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };
    
    println!("{}", user);  // Uses Display trait
}
```

## Best Practices

1. **Use descriptive field names**
2. **Keep structs focused** - single responsibility
3. **Implement common traits** when appropriate (`Debug`, `Clone`, etc.)
4. **Use associated functions** for constructors
5. **Prefer borrowing** over taking ownership in methods
6. **Group related functionality** in impl blocks

## Common Mistakes

1. **Forgetting to make struct mutable**:
```rust
let user = User { /* ... */ };
user.email = String::from("new@example.com"); // Error!

// Correct:
let mut user = User { /* ... */ };
user.email = String::from("new@example.com");
```

2. **Mixing up method vs associated function syntax**:
```rust
Rectangle::area(&rect);     // Wrong (works but awkward)
rect.area();               // Correct

Rectangle::new(10, 20);    // Correct
rect.new(10, 20);          // Wrong (won't compile)
```

## Try It Yourself

Create a `Car` struct with:
1. Fields for make, model, year, and mileage
2. Methods to start the engine, drive (increases mileage), and get car info
3. An associated function to create a new car
4. Implement the `Debug` trait

## Next Steps

Now that you understand structs, explore:
- [Enums]({{ '/lessons/enums/' | relative_url }}) - Creating types with variants
- [Error Handling]({{ '/lessons/error-handling/' | relative_url }}) - Using `Result` and `Option`
- [Traits]({{ '/lessons/traits/' | relative_url }}) - Shared behavior across types

Structs are the foundation for creating complex, well-organized Rust programs!