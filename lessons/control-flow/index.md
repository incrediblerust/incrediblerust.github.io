---
layout: default
title: Control Flow in Rust
---

# Control Flow in Rust

Control flow determines the order in which your code executes. Rust provides several constructs to control the flow of execution based on conditions and repetition.

## What You'll Learn

- Conditional statements with `if` expressions
- Loop constructs: `loop`, `while`, and `for`
- Pattern matching with `match`
- Break and continue statements
- Rust's expression-based approach to control flow

## If Expressions

In Rust, `if` is an expression, not a statement, which means it returns a value:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### If as an Expression

Since `if` is an expression, you can use it to assign values:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {}", number);
    
    // More complex example
    let x = 5;
    let description = if x > 0 {
        "positive"
    } else if x < 0 {
        "negative"
    } else {
        "zero"
    };
    
    println!("The number is {}", description);
}
```

**Important**: Both branches must return the same type!

```rust
// This won't compile
let number = if condition { 5 } else { "six" }; // Error: type mismatch
```

## Loop Constructs

Rust has three kinds of loops: `loop`, `while`, and `for`.

### The `loop` Keyword

The `loop` keyword creates an infinite loop:

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {}", counter);
        
        if counter == 5 {
            break;
        }
    }
    
    println!("Loop finished!");
}
```

### Returning Values from Loops

You can return values from loops using `break`:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Return value from loop
        }
    };

    println!("The result is: {}", result); // Prints: The result is: 20
}
```

### Loop Labels

For nested loops, you can use labels to break from outer loops:

```rust
fn main() {
    let mut count = 0;
    
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;  // Break from outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    
    println!("End count = {}", count);
}
```

### While Loops

`while` loops run while a condition is true:

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
    
    // Fibonacci with while
    let mut a = 0;
    let mut b = 1;
    
    while b < 100 {
        println!("{}", b);
        let temp = a + b;
        a = b;
        b = temp;
    }
}
```

### For Loops

`for` loops iterate over collections:

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];

    // Iterate over array elements
    for element in arr {
        println!("the value is: {}", element);
    }
    
    // Iterate over array with indices
    for (index, value) in arr.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    
    // Iterate over a range
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
    // Reverse range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
```

## Match Expressions

`match` is Rust's powerful pattern matching construct:

```rust
fn main() {
    let number = 13;

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}
```

### Match with Values

```rust
fn describe_number(x: i32) -> &'static str {
    match x {
        0 => "zero",
        1 => "one",
        2 => "two",
        3..=10 => "between three and ten",
        _ => "something else",
    }
}

fn main() {
    let numbers = vec![0, 1, 5, 15];
    
    for num in numbers {
        println!("{} is {}", num, describe_number(num));
    }
}
```

### Match Guards

You can add conditions to match arms:

```rust
fn categorize_number(x: i32) -> &'static str {
    match x {
        n if n < 0 => "negative",
        0 => "zero",
        n if n > 0 && n < 10 => "single digit positive",
        n if n >= 10 && n < 100 => "double digit positive",
        _ => "large positive",
    }
}

fn main() {
    let numbers = vec![-5, 0, 3, 42, 150];
    
    for num in numbers {
        println!("{} is {}", num, categorize_number(num));
    }
}
```

## If Let

For simple pattern matching, `if let` provides a more concise alternative:

```rust
fn main() {
    let some_u8_value = Some(3);
    
    // Using match
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
    // Using if let (more concise)
    if let Some(3) = some_u8_value {
        println!("three");
    }
    
    // With else clause
    if let Some(value) = some_u8_value {
        println!("Value is: {}", value);
    } else {
        println!("No value");
    }
}
```

## Practical Examples

### Grade Calculator

```rust
fn letter_grade(score: i32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

fn main() {
    let scores = vec![95, 87, 76, 62, 45];
    
    for score in scores {
        let grade = letter_grade(score);
        let status = if grade == 'F' { "Failing" } else { "Passing" };
        println!("Score: {}, Grade: {}, Status: {}", score, grade, status);
    }
}
```

### Number Guessing Game Logic

```rust
use std::cmp::Ordering;

fn check_guess(guess: i32, secret: i32) -> String {
    match guess.cmp(&secret) {
        Ordering::Less => {
            let diff = secret - guess;
            if diff > 10 {
                "Too small by a lot!".to_string()
            } else {
                "Too small!".to_string()
            }
        }
        Ordering::Greater => {
            let diff = guess - secret;
            if diff > 10 {
                "Too big by a lot!".to_string()
            } else {
                "Too big!".to_string()
            }
        }
        Ordering::Equal => "You win!".to_string(),
    }
}

fn main() {
    let secret_number = 42;
    let guesses = vec![20, 45, 42, 60];
    
    for guess in guesses {
        println!("Guess: {} -> {}", guess, check_guess(guess, secret_number));
    }
}
```

### Menu System

```rust
fn process_menu_choice(choice: char) {
    match choice {
        'a' | 'A' => println!("You chose: Add item"),
        'r' | 'R' => println!("You chose: Remove item"),
        'l' | 'L' => println!("You chose: List items"),
        'q' | 'Q' => {
            println!("Goodbye!");
            return;
        }
        _ => println!("Invalid choice. Please try again."),
    }
}

fn main() {
    let menu_choices = vec!['a', 'R', 'l', 'x', 'Q'];
    
    for choice in menu_choices {
        process_menu_choice(choice);
    }
}
```

## Advanced Control Flow

### Early Returns

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;  // Early return
    }
    
    Some(a / b)
}

fn process_division(a: f64, b: f64) {
    match divide(a, b) {
        Some(result) => println!("{} / {} = {}", a, b, result),
        None => println!("Cannot divide {} by zero", a),
    }
}

fn main() {
    process_division(10.0, 2.0);
    process_division(10.0, 0.0);
}
```

### Continue and Break

```rust
fn main() {
    // Skip even numbers
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;  // Skip rest of loop iteration
        }
        println!("Odd number: {}", i);
    }
    
    // Find first number divisible by 7
    for i in 1..=100 {
        if i % 7 == 0 {
            println!("First number divisible by 7: {}", i);
            break;  // Exit loop
        }
    }
}
```

## Best Practices

1. **Prefer `for` over `while`** for iterating over collections
2. **Use `match` for complex conditions** instead of multiple `if-else`
3. **Leverage `if let`** for simple pattern matching
4. **Use meaningful loop labels** for nested loops
5. **Keep conditions simple** and readable

## Common Mistakes

1. **Type mismatch in if expressions**:
```rust
// Wrong
let number = if condition { 5 } else { "six" };

// Correct
let number = if condition { 5 } else { 6 };
```

2. **Missing `mut` for loop variables**:
```rust
// Wrong
let counter = 0;
while counter < 10 {
    counter += 1;  // Error: cannot assign to immutable variable
}

// Correct
let mut counter = 0;
while counter < 10 {
    counter += 1;
}
```

3. **Infinite loops without break**:
```rust
// Be careful with this
loop {
    println!("This will run forever!");
    // Don't forget to add a break condition!
}
```

## Try It Yourself

Create a program that:
1. Generates numbers from 1 to 100
2. Prints "Fizz" for multiples of 3
3. Prints "Buzz" for multiples of 5
4. Prints "FizzBuzz" for multiples of both 3 and 5
5. Prints the number otherwise

## Next Steps

Now that you understand control flow, you're ready to explore:
- [Structs]({{ '/lessons/structs/' | relative_url }}) - Custom data types
- [Enums]({{ '/lessons/enums/' | relative_url }}) - Pattern matching with custom types
- [Error Handling]({{ '/lessons/error-handling/' | relative_url }}) - Using `Result` and `Option`

Control flow is essential for building complex logic in your Rust programs!