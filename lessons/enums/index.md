---
layout: default
title: Enums and Pattern Matching in Rust
---

# Enums and Pattern Matching in Rust

Enums (enumerations) allow you to define types with a fixed set of possible values. Combined with Rust's powerful pattern matching, they create expressive and safe code.

## What You'll Learn

- How to define and use enums
- Enums with data
- The `Option` and `Result` enums
- Pattern matching with `match`
- Conditional patterns with `if let`
- Advanced pattern matching techniques

## Basic Enums

Define an enum with the `enum` keyword:

```rust
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::North;
    
    match direction {
        Direction::North => println!("Heading north!"),
        Direction::South => println!("Heading south!"),
        Direction::East => println!("Heading east!"),
        Direction::West => println!("Heading west!"),
    }
}
```

### Enum Values

Each variant in an enum can have different types of data:

```rust
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Struct-like
    Write(String),              // Tuple-like
    ChangeColor(i32, i32, i32), // Tuple-like with multiple values
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, world!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for message in messages {
        process_message(message);
    }
}

fn process_message(message: Message) {
    match message {
        Message::Quit => println!("Quit message received"),
        Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}
```

## Methods on Enums

Like structs, enums can have methods:

```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting application"),
            Message::Move { x, y } => println!("Moving to position ({}, {})", x, y),
            Message::Write(text) => println!("Displaying: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Setting background to RGB({}, {}, {})", r, g, b);
            }
        }
    }
    
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

fn main() {
    let quit_msg = Message::Quit;
    let move_msg = Message::Move { x: 100, y: 200 };
    
    quit_msg.call();
    move_msg.call();
    
    println!("Is quit message? {}", quit_msg.is_quit());
    println!("Is move message quit? {}", move_msg.is_quit());
}
```

## The Option Enum

`Option<T>` is one of Rust's most important enums, used for values that might be absent:

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);
    
    match result1 {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero!"),
    }
    
    match result2 {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero!"),
    }
}
```

### Working with Option

```rust
fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn main() {
    let texts = vec!["hello", "", "world"];
    
    for text in texts {
        match get_first_char(text) {
            Some(ch) => println!("First character of '{}': '{}'", text, ch),
            None => println!("'{}' is empty", text),
        }
    }
    
    // Using Option methods
    let some_number = Some(5);
    let another_number = Some(10);
    let no_number: Option<i32> = None;
    
    // map transforms the value inside Some
    let doubled = some_number.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);
    
    // and_then chains operations that return Option
    let result = some_number.and_then(|x| another_number.map(|y| x + y));
    println!("Sum: {:?}", result);
    
    // unwrap_or provides a default value
    let value = no_number.unwrap_or(0);
    println!("Value or default: {}", value);
}
```

## The Result Enum

`Result<T, E>` handles operations that can fail:

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, MathError> {
    if denominator == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(numerator / denominator)
    }
}

fn safe_sqrt(number: f64) -> Result<f64, MathError> {
    if number < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(number.sqrt())
    }
}

fn main() {
    let operations = vec![
        (10.0, 2.0),
        (10.0, 0.0),
        (4.0, 2.0),
    ];
    
    for (a, b) in operations {
        match safe_divide(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(MathError::DivisionByZero) => println!("Error: Cannot divide {} by zero", a),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    
    let numbers = vec![4.0, -1.0, 16.0];
    
    for num in numbers {
        match safe_sqrt(num) {
            Ok(result) => println!("√{} = {}", num, result),
            Err(MathError::NegativeSquareRoot) => println!("Error: Cannot take square root of negative number {}", num),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
```

## Advanced Pattern Matching

### Guards

Add conditions to match arms:

```rust
fn categorize_number(x: i32) -> &'static str {
    match x {
        n if n < 0 => "negative",
        0 => "zero",
        n if n > 0 && n <= 10 => "small positive",
        n if n > 10 && n <= 100 => "medium positive",
        _ => "large positive",
    }
}

fn main() {
    let numbers = vec![-5, 0, 3, 15, 150];
    
    for num in numbers {
        println!("{} is {}", num, categorize_number(num));
    }
}
```

### Destructuring

Break apart complex data structures:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Shape {
    Circle { center: Point, radius: f64 },
    Rectangle { top_left: Point, bottom_right: Point },
    Triangle(Point, Point, Point),
}

fn describe_shape(shape: &Shape) {
    match shape {
        Shape::Circle { center: Point { x, y }, radius } => {
            println!("Circle at ({}, {}) with radius {}", x, y, radius);
        }
        Shape::Rectangle { 
            top_left: Point { x: x1, y: y1 }, 
            bottom_right: Point { x: x2, y: y2 } 
        } => {
            println!("Rectangle from ({}, {}) to ({}, {})", x1, y1, x2, y2);
        }
        Shape::Triangle(p1, p2, p3) => {
            println!("Triangle with vertices {:?}, {:?}, {:?}", p1, p2, p3);
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { 
            center: Point { x: 0, y: 0 }, 
            radius: 5.0 
        },
        Shape::Rectangle { 
            top_left: Point { x: 0, y: 10 }, 
            bottom_right: Point { x: 10, y: 0 } 
        },
        Shape::Triangle(
            Point { x: 0, y: 0 },
            Point { x: 5, y: 5 },
            Point { x: 10, y: 0 },
        ),
    ];
    
    for shape in &shapes {
        describe_shape(shape);
    }
}
```

### Multiple Patterns

Match multiple patterns in one arm:

```rust
#[derive(Debug)]
enum Animal {
    Dog,
    Cat,
    Bird,
    Fish,
    Hamster,
}

fn care_instructions(animal: &Animal) -> &'static str {
    match animal {
        Animal::Dog | Animal::Cat => "Feed twice daily, needs exercise",
        Animal::Bird => "Feed daily, change water, clean cage weekly",
        Animal::Fish => "Feed daily, clean tank monthly",
        Animal::Hamster => "Feed daily, provide wheel for exercise",
    }
}

fn main() {
    let pets = vec![Animal::Dog, Animal::Cat, Animal::Bird, Animal::Fish];
    
    for pet in pets {
        println!("{:?}: {}", pet, care_instructions(&pet));
    }
}
```

## If Let

For simple pattern matching, `if let` is more concise:

```rust
fn main() {
    let some_option = Some(42);
    
    // Using match
    match some_option {
        Some(value) => println!("Got value: {}", value),
        None => (),
    }
    
    // Using if let (more concise)
    if let Some(value) = some_option {
        println!("Got value: {}", value);
    }
    
    // With else
    if let Some(value) = some_option {
        println!("Value is: {}", value);
    } else {
        println!("No value");
    }
    
    // Chain multiple if let
    let message = Message::Write(String::from("Hello"));
    
    if let Message::Write(text) = message {
        println!("Text message: {}", text);
    } else if let Message::Move { x, y } = message {
        println!("Move to ({}, {})", x, y);
    } else {
        println!("Other message type");
    }
}
```

## While Let

Loop while a pattern matches:

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    
    // Pop elements while the vector is not empty
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    println!("Stack is now empty");
}
```

## Practical Examples

### State Machine

```rust
#[derive(Debug)]
enum State {
    Idle,
    Running { task: String },
    Paused { task: String, elapsed: u32 },
    Completed { task: String, duration: u32 },
}

impl State {
    fn start_task(&mut self, task: String) {
        *self = match self {
            State::Idle => State::Running { task },
            _ => {
                println!("Cannot start task: already busy");
                return;
            }
        };
    }
    
    fn pause(&mut self) {
        *self = match self {
            State::Running { task } => State::Paused { 
                task: task.clone(), 
                elapsed: 30  // Simulated elapsed time
            },
            _ => {
                println!("Cannot pause: not running");
                return;
            }
        };
    }
    
    fn resume(&mut self) {
        *self = match self {
            State::Paused { task, elapsed: _ } => State::Running { 
                task: task.clone() 
            },
            _ => {
                println!("Cannot resume: not paused");
                return;
            }
        };
    }
    
    fn complete(&mut self) {
        *self = match self {
            State::Running { task } => State::Completed { 
                task: task.clone(), 
                duration: 60  // Simulated duration
            },
            State::Paused { task, elapsed } => State::Completed { 
                task: task.clone(), 
                duration: elapsed + 30  // Simulated additional time
            },
            _ => {
                println!("Cannot complete: no active task");
                return;
            }
        };
    }
}

fn main() {
    let mut state = State::Idle;
    
    println!("Initial state: {:?}", state);
    
    state.start_task("Build project".to_string());
    println!("After starting: {:?}", state);
    
    state.pause();
    println!("After pausing: {:?}", state);
    
    state.resume();
    println!("After resuming: {:?}", state);
    
    state.complete();
    println!("After completing: {:?}", state);
}
```

### CLI Command Parser

```rust
#[derive(Debug)]
enum Command {
    Help,
    Version,
    Create { name: String, template: Option<String> },
    Delete { name: String, force: bool },
    List { filter: Option<String> },
}

fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    match parts.as_slice() {
        ["help"] => Some(Command::Help),
        ["version"] => Some(Command::Version),
        ["create", name] => Some(Command::Create { 
            name: name.to_string(), 
            template: None 
        }),
        ["create", name, "--template", template] => Some(Command::Create { 
            name: name.to_string(), 
            template: Some(template.to_string()) 
        }),
        ["delete", name] => Some(Command::Delete { 
            name: name.to_string(), 
            force: false 
        }),
        ["delete", name, "--force"] => Some(Command::Delete { 
            name: name.to_string(), 
            force: true 
        }),
        ["list"] => Some(Command::List { filter: None }),
        ["list", "--filter", filter] => Some(Command::List { 
            filter: Some(filter.to_string()) 
        }),
        _ => None,
    }
}

fn execute_command(command: Command) {
    match command {
        Command::Help => {
            println!("Available commands:");
            println!("  help                    - Show this help");
            println!("  version                 - Show version");
            println!("  create <name>           - Create new item");
            println!("  delete <name> [--force] - Delete item");
            println!("  list [--filter <term>]  - List items");
        }
        Command::Version => {
            println!("CLI Tool v1.0.0");
        }
        Command::Create { name, template } => {
            match template {
                Some(t) => println!("Creating '{}' with template '{}'", name, t),
                None => println!("Creating '{}' with default template", name),
            }
        }
        Command::Delete { name, force } => {
            if force {
                println!("Force deleting '{}'", name);
            } else {
                println!("Deleting '{}' (use --force to skip confirmation)", name);
            }
        }
        Command::List { filter } => {
            match filter {
                Some(f) => println!("Listing items filtered by '{}'", f),
                None => println!("Listing all items"),
            }
        }
    }
}

fn main() {
    let inputs = vec![
        "help",
        "version", 
        "create myproject",
        "create webapp --template react",
        "delete oldproject --force",
        "list --filter rust",
        "invalid command",
    ];
    
    for input in inputs {
        println!("\nInput: {}", input);
        match parse_command(input) {
            Some(command) => {
                println!("Parsed: {:?}", command);
                execute_command(command);
            }
            None => println!("Invalid command. Type 'help' for usage."),
        }
    }
}
```

## Best Practices

1. **Use meaningful variant names**
2. **Prefer enums over constants** for related values
3. **Use `match` for exhaustive pattern matching**
4. **Use `if let` for simple patterns**
5. **Consider using `#[derive(Debug)]`** for debugging
6. **Use `Option` instead of null values**
7. **Use `Result` for error handling**

## Common Mistakes

1. **Non-exhaustive match**:
```rust
// Wrong - compiler error
match direction {
    Direction::North => println!("North"),
    Direction::South => println!("South"),
    // Missing East and West
}

// Correct - add all variants or use _
match direction {
    Direction::North => println!("North"),
    Direction::South => println!("South"),
    _ => println!("East or West"),
}
```

2. **Forgetting to handle None/Err cases**:
```rust
// Risky - will panic if None
let value = some_option.unwrap();

// Better - handle the None case
let value = some_option.unwrap_or(0);

// Best - use pattern matching
match some_option {
    Some(value) => println!("Got: {}", value),
    None => println!("No value"),
}
```

## Try It Yourself

Create a traffic light enum with:
1. Variants for Red, Yellow, and Green
2. A method that returns the next state
3. A method that returns how long to wait (in seconds)
4. A program that cycles through all states

## Next Steps

Now that you understand enums and pattern matching, explore:
- [Error Handling]({{ '/lessons/error-handling/' | relative_url }}) - Advanced Result patterns
- [Collections]({{ '/lessons/collections/' | relative_url }}) - Vectors, hashmaps, and more
- [Traits]({{ '/lessons/traits/' | relative_url }}) - Shared behavior across types

Enums and pattern matching are powerful tools for creating expressive, safe Rust code!