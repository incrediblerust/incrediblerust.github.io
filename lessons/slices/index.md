---
layout: default
title: Slices in Rust
---

# Slices in Rust

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. Slices are a kind of reference, so they don't have ownership.

## What You'll Learn

- What slices are and how they work
- String slices (`&str`)
- Array and vector slices
- Slice syntax and indexing
- Practical slice patterns
- How slices prevent common programming errors

## What Are Slices?

A slice is a reference to a part (or all) of a collection. Slices are useful because they let you work with portions of data without copying it.

```rust
fn main() {
    let s = String::from("hello world");
    
    let hello = &s[0..5];    // Slice of "hello"
    let world = &s[6..11];   // Slice of "world"
    let whole = &s[..];      // Slice of entire string
    
    println!("hello: '{}', world: '{}', whole: '{}'", hello, world, whole);
}
```

## String Slices

String slices (`&str`) are references to parts of a `String`:

```rust
fn main() {
    let s = String::from("hello world");
    
    let hello = &s[0..5];   // From index 0 to 5 (exclusive)
    let world = &s[6..11];  // From index 6 to 11 (exclusive)
    
    // Shorthand syntax
    let hello2 = &s[..5];   // From start to 5
    let world2 = &s[6..];   // From 6 to end
    let entire = &s[..];    // Entire string
    
    println!("{}, {}", hello, world);
}
```

### Range Syntax

```rust
fn main() {
    let s = String::from("programming");
    
    // Different ways to slice
    let prog = &s[0..4];     // "prog" - indices 0, 1, 2, 3
    let gram = &s[3..7];     // "gram" - indices 3, 4, 5, 6
    let ming = &s[7..];      // "ming" - from 7 to end
    let full = &s[..];       // entire string
    
    println!("prog: {}, gram: {}, ming: {}, full: {}", prog, gram, ming, full);
}
```

### String Slices Are References

```rust
fn main() {
    let mut s = String::from("hello world");
    
    let word = first_word(&s);  // Get slice reference
    
    // s.clear();  // Error! Can't modify s while word borrows it
    
    println!("First word: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]  // Return entire string if no space found
}
```

## String Literals Are Slices

String literals are actually slices pointing to a specific point in the binary:

```rust
fn main() {
    let s: &str = "Hello, world!";  // s is a slice pointing to binary data
    
    // This is why string literals are immutable
    // They're slices of immutable data stored in the program binary
    
    println!("String literal: {}", s);
}
```

## Array Slices

You can create slices of arrays and vectors:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..4];  // [2, 3, 4]
    
    println!("Array: {:?}", a);
    println!("Slice: {:?}", slice);
    
    // Slice type is &[i32]
    print_slice(slice);
}

fn print_slice(slice: &[i32]) {
    println!("Slice contents: {:?}", slice);
    println!("Slice length: {}", slice.len());
}
```

### Vector Slices

```rust
fn main() {
    let v = vec![10, 20, 30, 40, 50];
    
    let first_three = &v[0..3];   // [10, 20, 30]
    let last_two = &v[3..];       // [40, 50]
    let middle = &v[1..4];        // [20, 30, 40]
    
    println!("Vector: {:?}", v);
    println!("First three: {:?}", first_three);
    println!("Last two: {:?}", last_two);
    println!("Middle: {:?}", middle);
    
    // Process slices
    println!("Sum of first three: {}", sum_slice(first_three));
    println!("Sum of last two: {}", sum_slice(last_two));
}

fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
```

## Slice Methods

Slices have many useful methods:

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &numbers[2..8];  // [3, 4, 5, 6, 7, 8]
    
    println!("Slice: {:?}", slice);
    println!("Length: {}", slice.len());
    println!("Is empty: {}", slice.is_empty());
    println!("First element: {:?}", slice.first());
    println!("Last element: {:?}", slice.last());
    
    // Searching
    println!("Contains 5: {}", slice.contains(&5));
    println!("Position of 6: {:?}", slice.iter().position(|&x| x == 6));
    
    // Iteration
    for (i, &value) in slice.iter().enumerate() {
        println!("Index {}: {}", i, value);
    }
}
```

### Splitting Slices

```rust
fn main() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let slice = &data[..];
    
    // Split at index
    let (left, right) = slice.split_at(4);
    println!("Left: {:?}, Right: {:?}", left, right);
    
    // Split by predicate
    let parts: Vec<&[i32]> = slice.split(|&x| x % 3 == 0).collect();
    println!("Split by multiples of 3: {:?}", parts);
    
    // Chunks
    let chunks: Vec<&[i32]> = slice.chunks(3).collect();
    println!("Chunks of 3: {:?}", chunks);
}
```

## Mutable Slices

You can create mutable slices to modify data:

```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    
    {
        let slice = &mut numbers[1..4];  // Mutable slice of [2, 3, 4]
        slice[0] = 10;  // Change 2 to 10
        slice[2] = 40;  // Change 4 to 40
    }
    
    println!("Modified array: {:?}", numbers);  // [1, 10, 3, 40, 5]
    
    // Function that modifies slice
    double_values(&mut numbers[..]);
    println!("After doubling: {:?}", numbers);
}

fn double_values(slice: &mut [i32]) {
    for value in slice.iter_mut() {
        *value *= 2;
    }
}
```

## Practical Examples

### Text Processing

```rust
fn main() {
    let text = "The quick brown fox jumps over the lazy dog";
    
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("Words: {:?}", words);
    
    let first_sentence = get_first_sentence(text);
    println!("First sentence: '{}'", first_sentence);
    
    let without_articles = remove_articles(text);
    println!("Without articles: '{}'", without_articles);
}

fn get_first_sentence(text: &str) -> &str {
    if let Some(pos) = text.find('.') {
        &text[..=pos]  // Include the period
    } else {
        text
    }
}

fn remove_articles(text: &str) -> String {
    text.split_whitespace()
        .filter(|&word| !matches!(word.to_lowercase().as_str(), "the" | "a" | "an"))
        .collect::<Vec<&str>>()
        .join(" ")
}
```

### Data Processing

```rust
fn main() {
    let temperatures = [20.5, 22.0, 19.8, 25.2, 21.1, 23.7, 18.9, 24.5];
    
    let week1 = &temperatures[0..4];
    let week2 = &temperatures[4..];
    
    println!("Week 1 temperatures: {:?}", week1);
    println!("Week 2 temperatures: {:?}", week2);
    
    println!("Week 1 average: {:.1}°C", average(week1));
    println!("Week 2 average: {:.1}°C", average(week2));
    
    let above_20 = filter_above_threshold(&temperatures, 20.0);
    println!("Days above 20°C: {:?}", above_20);
}

fn average(temps: &[f64]) -> f64 {
    temps.iter().sum::<f64>() / temps.len() as f64
}

fn filter_above_threshold(temps: &[f64], threshold: f64) -> Vec<f64> {
    temps.iter()
        .filter(|&&temp| temp > threshold)
        .copied()
        .collect()
}
```

### Buffer Processing

```rust
fn main() {
    let mut buffer = [0u8; 10];
    
    // Fill buffer with data
    for (i, byte) in buffer.iter_mut().enumerate() {
        *byte = (i * 10) as u8;
    }
    
    println!("Buffer: {:?}", buffer);
    
    // Process chunks
    process_chunks(&buffer, 3);
    
    // Find pattern
    let pattern = [20, 30];
    if let Some(pos) = find_pattern(&buffer, &pattern) {
        println!("Pattern found at position: {}", pos);
    }
}

fn process_chunks(data: &[u8], chunk_size: usize) {
    for (i, chunk) in data.chunks(chunk_size).enumerate() {
        println!("Chunk {}: {:?}", i, chunk);
    }
}

fn find_pattern(data: &[u8], pattern: &[u8]) -> Option<usize> {
    data.windows(pattern.len())
        .position(|window| window == pattern)
}
```

## Advanced Slice Patterns

### Working with Nested Data

```rust
fn main() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    
    // Get a row
    let row = &matrix[1];  // [4, 5, 6]
    println!("Row 1: {:?}", row);
    
    // Get a column (more complex)
    let column: Vec<i32> = matrix.iter().map(|row| row[1]).collect();
    println!("Column 1: {:?}", column);  // [2, 5, 8]
    
    // Process submatrix
    let submatrix: Vec<&[i32]> = matrix[0..2].iter().map(|row| &row[1..3]).collect();
    println!("Submatrix: {:?}", submatrix);  // [[2, 3], [5, 6]]
}
```

### Range Patterns

```rust
fn main() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Different range patterns
    let patterns = [
        &data[..3],      // First 3: [1, 2, 3]
        &data[3..6],     // Middle 3: [4, 5, 6]
        &data[6..],      // Last 4: [7, 8, 9, 10]
        &data[..],       // All: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        &data[1..=3],    // Inclusive range: [2, 3, 4]
    ];
    
    for (i, slice) in patterns.iter().enumerate() {
        println!("Pattern {}: {:?}", i, slice);
    }
}
```

### Error Handling with Slices

```rust
fn main() {
    let data = [1, 2, 3, 4, 5];
    
    // Safe slicing with get
    match data.get(1..4) {
        Some(slice) => println!("Safe slice: {:?}", slice),
        None => println!("Invalid range"),
    }
    
    // This would panic:
    // let bad_slice = &data[10..20];  // Panic: index out of bounds
    
    // Safe alternative
    if let Some(slice) = data.get(10..20) {
        println!("Slice: {:?}", slice);
    } else {
        println!("Range out of bounds");
    }
}

fn safe_slice<T>(data: &[T], start: usize, end: usize) -> Option<&[T]> {
    if start <= end && end <= data.len() {
        Some(&data[start..end])
    } else {
        None
    }
}
```

## Converting Between String Types

```rust
fn main() {
    // String to &str
    let owned = String::from("hello world");
    let borrowed: &str = &owned;
    let slice: &str = &owned[0..5];
    
    // &str to String
    let from_slice = slice.to_string();
    let from_slice2 = slice.to_owned();
    let from_slice3 = String::from(slice);
    
    println!("Original: {}", owned);
    println!("Borrowed: {}", borrowed);
    println!("Slice: {}", slice);
    println!("From slice: {}", from_slice);
}

// Function that accepts both String and &str
fn print_text(text: &str) {
    println!("Text: {}", text);
}

// Usage
fn example_usage() {
    let owned = String::from("hello");
    let slice = "world";
    
    print_text(&owned);  // Convert String to &str
    print_text(slice);   // Already &str
}
```

## Performance Considerations

Slices are zero-cost abstractions:

```rust
fn main() {
    let large_data = vec![1; 1_000_000];
    
    // This doesn't copy data - just creates a reference
    let slice = &large_data[100_000..200_000];
    
    println!("Slice length: {}", slice.len());  // Very fast
    
    // Processing slice is efficient
    let sum: i32 = slice.iter().sum();
    println!("Sum: {}", sum);
}
```

## Best Practices

1. **Use &str instead of String for parameters** when you don't need ownership
2. **Use slices for function parameters** to accept both arrays and vectors
3. **Prefer get() over direct indexing** for potentially invalid ranges
4. **Use inclusive ranges (..=)** when the end should be included
5. **Remember slices prevent mutation** of the original data while borrowed

## Common Mistakes

1. **Out of bounds slicing**:
```rust
let data = [1, 2, 3];
// let slice = &data[0..10];  // Panic!

// Better:
if let Some(slice) = data.get(0..10) {
    println!("Slice: {:?}", slice);
} else {
    println!("Range out of bounds");
}
```

2. **Char boundary issues with strings**:
```rust
let s = "🦀 Rust";
// let slice = &s[0..2];  // Panic! 🦀 is 4 bytes

// Better:
let slice = &s[0..4];  // First emoji
println!("Slice: {}", slice);
```

3. **Forgetting slice lifetimes**:
```rust
// fn bad_function() -> &str {
//     let s = String::from("hello");
//     &s[0..2]  // Error: s is dropped when function ends
// }

// Better:
fn good_function(s: &str) -> &str {
    &s[0..2]  // Slice has same lifetime as input
}
```

## Try It Yourself

Create a program that:
1. Takes a sentence as input
2. Uses slices to extract individual words
3. Finds the longest and shortest words using slices
4. Creates a new sentence with words in reverse order
5. Handles edge cases safely

## Next Steps

Now that you understand slices, explore:
- [Lifetimes]({{ '/lessons/lifetimes/' | relative_url }}) - Ensuring slice references are valid
- [Collections]({{ '/lessons/collections/' | relative_url }}) - Vectors, strings, and hashmaps
- [Iterators]({{ '/lessons/iterators/' | relative_url }}) - Processing data efficiently

Slices are essential for writing efficient, safe Rust code that works with data without unnecessary copying!