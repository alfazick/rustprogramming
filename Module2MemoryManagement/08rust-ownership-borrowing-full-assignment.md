# Rust Ownership and Borrowing Assignments

## Assignment 1: Mutable Reference Sum with Step

### Problem Statement

Implement a function `sum_with_step` that:
- Takes a mutable reference for the sum result, and three integers: low, high, and step
- Calculates the sum from low to high (inclusive) with the given step
- Stores the result in the mutable reference

### Solution

```rust
fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    unimplemented();
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}
```

Expected output:
```
Sum 0 to 100, step 1: 5050
Sum 0 to 10, step 2: 30
Sum 5 to 15, step 3: 35
```

## Assignment 2: Word Frequency Counter

### Problem Statement

Create a program that:
1. Takes a string of text as input
2. Splits the text into words (space as separator) // text.split_whitespace().collect();
3. Counts the frequency of each word
4. Returns the word with the highest frequency and its count

Requirements:
- Use mutable references where appropriate
- Avoid using HashMaps or complex data structures

### Solution

```rust
fn most_frequent_word(text: &str) -> (String, usize) {
    
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
```

Expected output:
```
Most frequent word: "the" (3 times)
```

1. Use of mutable references to modify values
2. Borrowing of input data
3. Working with string slices
4. Basic loop structures and indexing
5. Ownership rules in function parameters and return values
