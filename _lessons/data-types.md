---
title: Data Types
difficulty: beginner
version: 1.85.0
prev_lesson: /lessons/variables/
prev_lesson_title: Variables and Mutability
next_lesson: /lessons/functions/
next_lesson_title: Functions
---

# Data Types

Rust is a statically typed language, which means every variable must have a type known at compile time. The Rust compiler can usually infer the type based on the value and usage, but sometimes you need to specify it explicitly.

## Scalar Types

Scalar types represent single values. Rust has four primary scalar types:

### Integer Types

Integers are numbers without fractional components. Rust provides several integer types:

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

```rust
fn main() {
    let age: u8 = 25;                    // Unsigned 8-bit integer
    let temperature: i32 = -10;          // Signed 32-bit integer (default)
    let big_number: u64 = 1_000_000;     // Underscores for readability
    let hex = 0xff;                      // Hexadecimal
    let octal = 0o77;                    // Octal
    let binary = 0b1111_0000;            // Binary
    let byte = b'A';                     // Byte (u8 only)
    
    println!("Age: {}", age);
    println!("Temperature: {}", temperature);
    println!("Big number: {}", big_number);
    println!("Hex: {}", hex);
    println!("Binary: {}", binary);
}
```

**Note**: `i32` is the default integer type because it's generally the fastest, even on 64-bit systems.

### Floating-Point Types

Rust has two floating-point types for numbers with decimal points:

```rust
fn main() {
    let x = 2.0;        // f64 (default)
    let y: f32 = 3.0;   // f32
    
    println!("x: {}", x);
    println!("y: {}", y);
}
```

- `f32`: single-precision float
- `f64`: double-precision float (default)

### Boolean Type

Booleans can be either `true` or `false`:

```rust
fn main() {
    let is_rust_fun = true;
    let is_learning: bool = false;
    
    println!("Is Rust fun? {}", is_rust_fun);
    println!("Still learning? {}", is_learning);
}
```

### Character Type

The `char` type represents a Unicode scalar value:

```rust
fn main() {
    let letter = 'R';
    let emoji = '😻';
    let chinese = '中';
    
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);
    println!("Chinese: {}", chinese);
}
```

**Important**: `char` literals use single quotes, while string literals use double quotes.

## Compound Types

Compound types can group multiple values into one type.

### Tuple Type

Tuples group values of different types:

```rust
fn main() {
    let person = ("Alice", 30, true);
    let (name, age, is_employed) = person; // Destructuring
    
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Employed: {}", is_employed);
    
    // Access by index
    println!("First element: {}", person.0);
    println!("Second element: {}", person.1);
}
```

You can also specify types explicitly:

```rust
fn main() {
    let coordinates: (f64, f64) = (3.14, 2.71);
    println!("X: {}, Y: {}", coordinates.0, coordinates.1);
}
```

The unit tuple `()` is a special type with only one value, also written `()`. It represents an empty value or empty return type.

### Array Type

Arrays hold multiple values of the same type with a fixed length:

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    
    println!("First number: {}", numbers[0]);
    println!("Second fruit: {}", fruits[1]);
    println!("Array length: {}", numbers.len());
    
    // Iterate over array
    for number in numbers {
        println!("Number: {}", number);
    }
}
```

Array type annotation: `[type; length]`

## Type Annotations

Sometimes Rust cannot infer the type, and you must specify it:

```rust
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {}", guess);
}
```

Without the type annotation, this would cause a compilation error because `parse()` can return many different types.

## Numeric Operations

Rust supports standard mathematical operations:

```rust
fn main() {
    // Addition
    let sum = 5 + 10;
    
    // Subtraction
    let difference = 95.5 - 4.3;
    
    // Multiplication
    let product = 4 * 30;
    
    // Division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0 (integer division)
    
    // Remainder
    let remainder = 43 % 5;
    
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Floored: {}", floored);
    println!("Remainder: {}", remainder);
}
```

## Type Conversion

Rust is strict about type conversions. You must be explicit:

```rust
fn main() {
    let integer = 10;
    let float = 3.14;
    
    // This won't work:
    // let result = integer + float;
    
    // You must convert explicitly:
    let result = integer as f64 + float;
    println!("Result: {}", result);
    
    // Converting float to integer (truncates)
    let truncated = float as i32;
    println!("Truncated: {}", truncated);
}
```

## Practical Examples

### Temperature Converter

```rust
fn main() {
    let fahrenheit: f64 = 98.6;
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    
    println!("{}°F is {:.1}°C", fahrenheit, celsius);
}
```

### Working with User Input

```rust
use std::io;

fn main() {
    println!("Enter your age:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let age: u8 = input.trim().parse().expect("Please enter a valid number");
    
    if age >= 18 {
        println!("You are an adult!");
    } else {
        println!("You are a minor.");
    }
}
```

### Array Statistics

```rust
fn main() {
    let numbers = [23, 45, 12, 67, 34, 89, 56];
    
    let mut sum = 0;
    let mut max = numbers[0];
    let mut min = numbers[0];
    
    for &number in &numbers {
        sum += number;
        if number > max {
            max = number;
        }
        if number < min {
            min = number;
        }
    }
    
    let average = sum as f64 / numbers.len() as f64;
    
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);
    println!("Average: {:.2}", average);
    println!("Max: {}", max);
    println!("Min: {}", min);
}
```

## Common Patterns

### Multiple Variable Declaration

```rust
fn main() {
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

### Array Initialization

```rust
fn main() {
    // Initialize with same value
    let buffer: [u8; 1024] = [0; 1024];
    println!("Buffer size: {}", buffer.len());
    
    // Initialize with different values
    let rgb = [255, 128, 0]; // Orange color
    println!("Red: {}, Green: {}, Blue: {}", rgb[0], rgb[1], rgb[2]);
}
```

### Type Suffixes

```rust
fn main() {
    let x = 5u32;      // u32
    let y = 3.14f32;   // f32
    
    println!("x: {}, y: {}", x, y);
}
```

## Error Handling with Types

Rust prevents many common programming errors at compile time:

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    // This is safe - compiler knows the index is valid
    println!("First: {}", numbers[0]);
    
    // This would panic at runtime if index is out of bounds
    // let index = 10;
    // println!("Element: {}", numbers[index]);
    
    // Better approach - safe access
    if let Some(value) = numbers.get(10) {
        println!("Found: {}", value);
    } else {
        println!("Index out of bounds");
    }
}
```

## Key Takeaways

- Rust is statically typed - types are checked at compile time
- Scalar types: integers, floats, booleans, characters
- Compound types: tuples, arrays
- `i32` and `f64` are default numeric types
- Type conversions must be explicit using `as`
- Arrays have fixed length, tuples can mix types
- The compiler prevents many type-related errors

## Next Steps

Now that you understand Rust's data types, let's learn about [Functions]({{ '/lessons/functions/' | relative_url }}) to see how to organize code and work with different types.

## Try It Yourself

1. Create variables of each scalar type and print them
2. Write a program that calculates the area and perimeter of a rectangle
3. Create a tuple representing a book (title, author, pages, price)
4. Make an array of your favorite numbers and calculate their sum
5. Convert temperatures between Celsius and Fahrenheit

Happy coding! 🦀