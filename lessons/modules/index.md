---
layout: default
title: Modules & Packages in Rust
---

# Modules & Packages in Rust

Rust's module system is designed to help you organize your code and control privacy. It provides powerful tools for creating reusable, maintainable, and well-structured programs.

## What You'll Learn

- How to organize code using modules
- Understanding packages, crates, and the module tree
- Managing visibility with `pub` and privacy rules
- Using `use` statements for bringing paths into scope
- Creating and publishing crates
- Workspace management for large projects

## Understanding the Module System

Rust's module system has several key concepts:

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules and use**: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

## Packages and Crates

### What is a Package?

A package is one or more crates that provide a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

```toml
# Cargo.toml
[package]
name = "restaurant"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Types of Crates

**Binary Crate**: A program you can run (has a `main` function)
**Library Crate**: Code intended to be used by other programs

```rust
// src/main.rs - Binary crate root
fn main() {
    println!("Hello from binary crate!");
}

// src/lib.rs - Library crate root
pub fn greet() {
    println!("Hello from library crate!");
}
```

## Defining Modules

Modules let you organize code within a crate for readability and easy reuse:

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

### Module Tree Structure

```
restaurant
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## Paths for Referring to Items

### Absolute vs Relative Paths

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

### Privacy Rules

- All items (functions, methods, structs, enums, modules, and constants) are private by default
- Items in a parent module can't use private items inside child modules
- Items in child modules can use items in their ancestor modules

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn private_function() {} // Private by default
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // Private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

## The `use` Keyword

Bring paths into scope to avoid repetitive path writing:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // Much cleaner!
}
```

### Creating Idiomatic `use` Paths

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new(); // Idiomatic for structs
    map.insert(1, 2);
}

// For functions, it's idiomatic to bring the parent module into scope
use std::fmt;

fn some_function() -> fmt::Result {
    // --snip--
    Ok(())
}
```

### Providing New Names with `as`

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

### Re-exporting Names with `pub use`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## Using External Packages

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }
```

Then use them in your code:

```rust
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("Serialized: {}", serialized);
}
```

## Separating Modules into Different Files

### Single File Module

```rust
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

```rust
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

### Module with Subdirectory

```rust
// src/front_of_house.rs
pub mod hosting;
```

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

## Advanced Module Patterns

### Module with Multiple Files

```
src/
├── lib.rs
├── user/
│   ├── mod.rs
│   ├── authentication.rs
│   └── profile.rs
└── order/
    ├── mod.rs
    └── management.rs
```

```rust
// src/user/mod.rs
pub mod authentication;
pub mod profile;

pub use authentication::*;
pub use profile::*;
```

```rust
// src/user/authentication.rs
pub struct User {
    pub id: u64,
    pub username: String,
}

impl User {
    pub fn new(id: u64, username: String) -> Self {
        User { id, username }
    }
    
    pub fn authenticate(&self, password: &str) -> bool {
        // Authentication logic here
        !password.is_empty()
    }
}
```

## Working with Workspaces

For large projects, use workspaces to manage multiple related packages:

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "adder",
    "add_one",
]
```

```toml
# adder/Cargo.toml
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]
add_one = { path = "../add_one" }
```

```toml
# add_one/Cargo.toml
[package]
name = "add_one"
version = "0.1.0"
edition = "2021"
```

```rust
// add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
```

```rust
// adder/src/main.rs
use add_one;

fn main() {
    let num = 10;
    println!("{} plus one is {}!", num, add_one::add_one(num));
}
```

## Practical Example: Building a Library

Let's create a practical library for handling configuration:

```rust
// src/lib.rs
//! # Configuration Library
//! 
//! A simple library for managing application configuration.

pub mod config;
pub mod env;
pub mod file;

pub use config::Config;

/// Initialize a new configuration
pub fn init() -> Config {
    Config::new()
}
```

```rust
// src/config.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    settings: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            settings: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.settings.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }
    
    pub fn load_from_env(&mut self) {
        if let Ok(value) = std::env::var("DATABASE_URL") {
            self.set("database_url".to_string(), value);
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
```

```rust
// src/env.rs
use crate::Config;

pub fn load_environment_variables(config: &mut Config) {
    for (key, value) in std::env::vars() {
        if key.starts_with("APP_") {
            let config_key = key.strip_prefix("APP_")
                .unwrap()
                .to_lowercase();
            config.set(config_key, value);
        }
    }
}
```

## Best Practices

1. **Keep modules focused**: Each module should have a single responsibility
2. **Use clear naming**: Module names should clearly indicate their purpose
3. **Minimize public API**: Only expose what's necessary
4. **Group related functionality**: Put related functions, structs, and traits together
5. **Use re-exports wisely**: Make your API easy to use with `pub use`
6. **Document public APIs**: Add documentation comments for public items

```rust
/// A user authentication module.
/// 
/// This module provides functionality for user authentication,
/// including login, logout, and session management.
pub mod auth {
    /// Represents a user in the system.
    pub struct User {
        id: u64,
        username: String,
    }
    
    impl User {
        /// Creates a new user with the given ID and username.
        /// 
        /// # Examples
        /// 
        /// ```
        /// use myapp::auth::User;
        /// 
        /// let user = User::new(1, "alice".to_string());
        /// ```
        pub fn new(id: u64, username: String) -> Self {
            User { id, username }
        }
    }
}
```

## Common Patterns

### The Facade Pattern

```rust
// src/lib.rs
mod database;
mod cache;
mod logger;

pub use database::Database;
pub use cache::Cache;

// Provide a simplified interface
pub struct App {
    db: Database,
    cache: Cache,
}

impl App {
    pub fn new() -> Self {
        App {
            db: Database::connect(),
            cache: Cache::new(),
        }
    }
    
    pub fn get_user(&self, id: u64) -> Option<String> {
        // Check cache first, then database
        if let Some(user) = self.cache.get(&id.to_string()) {
            Some(user)
        } else {
            self.db.find_user(id)
        }
    }
}
```

## Testing Modules

```rust
// src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }
}
```

## Publishing Crates

To publish your crate to crates.io:

1. **Add metadata to Cargo.toml**:

```toml
[package]
name = "my_awesome_crate"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2021"
description = "A short description of my crate"
license = "MIT"
repository = "https://github.com/yourusername/my_awesome_crate"
keywords = ["rust", "example", "awesome"]
categories = ["development-tools"]
readme = "README.md"
```

2. **Login to crates.io**:
```bash
cargo login
```

3. **Publish**:
```bash
cargo publish
```

## Next Steps

Now that you understand modules and packages, you're ready to learn about:
- [Error Handling]({{ '/lessons/error-handling/' | relative_url }}) - Managing and handling errors gracefully
- [Testing]({{ '/lessons/testing/' | relative_url }}) - Writing comprehensive tests for your modules
- [Documentation]({{ '/lessons/documentation/' | relative_url }}) - Documenting your code effectively

Modules are the foundation of well-organized Rust code. Master them, and you'll be able to build maintainable, scalable applications!