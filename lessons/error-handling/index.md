---
layout: default
title: Error Handling in Rust
---

# Error Handling in Rust

Rust groups errors into two major categories: recoverable and unrecoverable errors. For recoverable errors, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

## What You'll Learn

- Understanding Rust's approach to error handling
- Working with `Result<T, E>` for recoverable errors
- Using `panic!` for unrecoverable errors
- Error propagation with the `?` operator
- Creating custom error types
- Best practices for robust error handling

## Unrecoverable Errors with `panic!`

When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

```rust
fn main() {
    panic!("crash and burn");
}
```

### When to Use `panic!`

- When your program is in an unrecoverable state
- During development for debugging
- In tests to indicate failure
- When a contract is violated (assertions)

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}

fn main() {
    let result = divide(10, 0); // This will panic
    println!("Result: {}", result);
}
```

### Backtrace Information

Set the `RUST_BACKTRACE` environment variable to get a backtrace:

```bash
RUST_BACKTRACE=1 cargo run
```

## Recoverable Errors with `Result`

Most errors aren't serious enough to require the program to stop entirely. The `Result` enum is defined as:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Basic Result Handling

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

### Shortcuts for Panic on Error: `unwrap` and `expect`

```rust
use std::fs::File;

fn main() {
    // Using unwrap - panics on error with generic message
    let greeting_file = File::open("hello.txt").unwrap();
    
    // Using expect - panics with custom error message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

## Propagating Errors

Instead of handling errors immediately, you can return them to the calling code:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

### The `?` Operator - Shortcut for Error Propagation

The `?` operator can only be used in functions that return `Result`, `Option`, or another type that implements the `FromResidual` trait:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Even shorter
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Using fs::read_to_string
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
```

## Working with Multiple Error Types

### Using Box<dyn Error>

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_file_and_parse() -> Result<i32, Box<dyn Error>> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    match read_file_and_parse() {
        Ok(number) => println!("Number: {}", number),
        Err(e) => println!("Error: {}", e),
    }
}
```

## Creating Custom Error Types

### Simple Custom Error

```rust
use std::fmt;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.message)
    }
}

impl std::error::Error for CustomError {}

fn might_fail(should_fail: bool) -> Result<String, CustomError> {
    if should_fail {
        Err(CustomError::new("Something went wrong"))
    } else {
        Ok("Success!".to_string())
    }
}
```

### Enum-Based Error Types

```rust
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(err) => write!(f, "IO error: {}", err),
            MyError::Parse(err) => write!(f, "Parse error: {}", err),
            MyError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl std::error::Error for MyError {}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<ParseIntError> for MyError {
    fn from(error: ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

fn process_file(filename: &str) -> Result<i32, MyError> {
    let contents = std::fs::read_to_string(filename)?;
    let number: i32 = contents.trim().parse()?;
    
    if number < 0 {
        return Err(MyError::Custom("Number cannot be negative".to_string()));
    }
    
    Ok(number * 2)
}
```

## Error Handling Patterns

### The `?` Operator with Different Return Types

```rust
fn example() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("test.txt")?;
    let number: i32 = "42".parse()?;
    println!("Number: {}", number);
    Ok(())
}
```

### Using `map_err` for Error Conversion

```rust
use std::fs::File;

fn read_file() -> Result<String, String> {
    std::fs::read_to_string("test.txt")
        .map_err(|e| format!("Failed to read file: {}", e))
}
```

### Combining Results with `and_then`

```rust
fn parse_and_double(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
        .and_then(|n| Ok(n * 2))
}

// Or using map
fn parse_and_double_map(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
        .map(|n| n * 2)
}
```

## Option and Result Interaction

### Converting Option to Result

```rust
fn find_item(items: &[i32], target: i32) -> Result<usize, &'static str> {
    items.iter()
        .position(|&x| x == target)
        .ok_or("Item not found")
}
```

### Using `transpose`

```rust
fn process_optional_result(
    opt_result: Option<Result<i32, &str>>
) -> Result<Option<i32>, &str> {
    opt_result.transpose()
}
```

## Advanced Error Handling

### Using `anyhow` for Application Errors

Add to `Cargo.toml`:
```toml
[dependencies]
anyhow = "1.0"
```

```rust
use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .with_context(|| "Failed to read configuration file")
}

fn parse_number(s: &str) -> Result<i32> {
    s.parse::<i32>()
        .with_context(|| format!("Failed to parse '{}' as a number", s))
}

fn main() -> Result<()> {
    let config = read_config()?;
    let number = parse_number(&config)?;
    println!("Parsed number: {}", number);
    Ok(())
}
```

### Using `thiserror` for Library Errors

Add to `Cargo.toml`:
```toml
[dependencies]
thiserror = "1.0"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

## Practical Examples

### File Processing with Error Handling

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum ProcessingError {
    IoError(std::io::Error),
    ParseError(String),
    ValidationError(String),
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingError::IoError(e) => write!(f, "IO error: {}", e),
            ProcessingError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProcessingError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ProcessingError {}

impl From<std::io::Error> for ProcessingError {
    fn from(error: std::io::Error) -> Self {
        ProcessingError::IoError(error)
    }
}

fn process_numbers_file<P: AsRef<Path>>(path: P) -> Result<Vec<i32>, ProcessingError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        
        if line.trim().is_empty() {
            continue; // Skip empty lines
        }
        
        let number: i32 = line.trim().parse()
            .map_err(|_| ProcessingError::ParseError(
                format!("Invalid number on line {}: '{}'", line_num + 1, line)
            ))?;
        
        if number < 0 {
            return Err(ProcessingError::ValidationError(
                format!("Negative number not allowed on line {}: {}", line_num + 1, number)
            ));
        }
        
        numbers.push(number);
    }

    Ok(numbers)
}

fn main() {
    match process_numbers_file("numbers.txt") {
        Ok(numbers) => {
            println!("Successfully processed {} numbers", numbers.len());
            println!("Sum: {}", numbers.iter().sum::<i32>());
        }
        Err(e) => {
            eprintln!("Error processing file: {}", e);
            std::process::exit(1);
        }
    }
}
```

### Network Request with Timeout

```rust
use std::time::Duration;
use std::thread;

#[derive(Debug)]
enum NetworkError {
    Timeout,
    ConnectionFailed,
    InvalidResponse,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError::Timeout => write!(f, "Request timed out"),
            NetworkError::ConnectionFailed => write!(f, "Connection failed"),
            NetworkError::InvalidResponse => write!(f, "Invalid response"),
        }
    }
}

impl std::error::Error for NetworkError {}

fn simulate_network_request(should_succeed: bool) -> Result<String, NetworkError> {
    // Simulate network delay
    thread::sleep(Duration::from_millis(100));
    
    if should_succeed {
        Ok("Response data".to_string())
    } else {
        Err(NetworkError::ConnectionFailed)
    }
}

fn fetch_with_retry(max_retries: u32) -> Result<String, NetworkError> {
    for attempt in 1..=max_retries {
        println!("Attempt {} of {}", attempt, max_retries);
        
        match simulate_network_request(attempt == max_retries) {
            Ok(data) => return Ok(data),
            Err(e) => {
                if attempt == max_retries {
                    return Err(e);
                }
                println!("Request failed: {}. Retrying...", e);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
    
    unreachable!()
}
```

## Best Practices

### 1. Use Specific Error Types

```rust
// Good: Specific error types
fn parse_config(content: &str) -> Result<Config, ConfigError> {
    // ...
}

// Avoid: Generic error types for libraries
fn parse_config_bad(content: &str) -> Result<Config, Box<dyn Error>> {
    // ...
}
```

### 2. Provide Context

```rust
use std::fs;

fn load_user_data(user_id: u64) -> Result<UserData, UserError> {
    let path = format!("users/{}.json", user_id);
    let content = fs::read_to_string(&path)
        .map_err(|e| UserError::FileRead {
            path: path.clone(),
            source: e,
        })?;
    
    let user_data: UserData = serde_json::from_str(&content)
        .map_err(|e| UserError::JsonParse {
            path,
            source: e,
        })?;
    
    Ok(user_data)
}
```

### 3. Fail Fast When Appropriate

```rust
fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Err(ValidationError::Empty);
    }
    
    if !email.contains('@') {
        return Err(ValidationError::InvalidFormat);
    }
    
    Ok(())
}
```

### 4. Use `main` Function that Returns Result

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    let data = fetch_data(&config)?;
    process_data(data)?;
    
    println!("Processing completed successfully!");
    Ok(())
}
```

## Testing Error Conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_by_zero() {
        let result = safe_divide(10, 0);
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Division by zero");
        }
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_unsafe_division_panics() {
        unsafe_divide(10, 0);
    }
}
```

## Summary

Rust's error handling system encourages you to acknowledge the possibility of errors and take appropriate action before your code will compile. This makes Rust programs more reliable and helps prevent bugs in production.

Key takeaways:
- Use `Result<T, E>` for recoverable errors
- Use `panic!` sparingly, only for unrecoverable errors
- The `?` operator makes error propagation concise
- Create custom error types for better error handling
- Provide context and specific error messages
- Test your error conditions

## Next Steps

Now that you understand error handling, you're ready to explore:
- [Testing]({{ '/lessons/testing/' | relative_url }}) - Writing comprehensive tests including error cases
- [Concurrency]({{ '/lessons/concurrency/' | relative_url }}) - Handling errors in concurrent code
- [Async Programming]({{ '/lessons/async/' | relative_url }}) - Error handling in asynchronous contexts

Mastering error handling will make your Rust code more robust and your debugging sessions much more productive!