# Rust Borrowing and Ownership Practice Problems

## Warm-up In-Class Practice

### Problem #1: String Concatenation with Borrowing

Write a function that concatenates two strings without taking ownership, i.e., by borrowing.

```rust
fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}
```

### Problem #2: Clone and Modify

Given a string, clone it and modify the cloned string by appending a new word. Print both the original string and the cloned, modified string to show that the original has not been changed.

```rust
fn clone_and_modify(s: &String) -> String {
    // Your code here
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}
```

### Problem #3: Mutable Reference Sum

Write a modified sum function that takes a mutable reference for the destination of the sum from low to high.

```rust
#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    0
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    // total should be 5050
}
```

## Instructions

1. Implement the functions for each problem.
2. Make sure your code compiles without errors.
3. Test your implementations with the provided `main` functions and verify the output.

Remember to use Rust's borrowing rules correctly and think about how ownership is managed in each scenario.
