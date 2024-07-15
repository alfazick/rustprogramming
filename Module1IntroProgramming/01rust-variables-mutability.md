# Rust: Variables and Mutability

## Introduction

Rust's approach to variables and mutability is one of its distinctive features. This document explores the concepts of immutability by default, mutable variables, and constants in Rust, providing both theoretical explanations and practical code examples.

## 1. Immutability by Default

In Rust, variables are immutable by default. This design choice enhances safety and facilitates concurrent programming.

### Theory

- Immutability as the default state reduces the potential for unexpected value changes.
- This approach is similar to other modern languages like Kotlin and Go.
- It aligns with best practices in C and C++, where declaring variables as constant (e.g., `const int* ptr`) is considered good practice.

### Code Example

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    
    // Uncommenting the next line will result in a compilation error
    // x = 6; // error: cannot assign twice to immutable variable `x`
}
```

## 2. Mutable Variables

When you need to change a variable's value, Rust provides the `mut` keyword to make a variable mutable.

### Theory

- The `mut` keyword explicitly indicates that a variable's value can change.
- This approach makes code more readable and helps prevent unintended mutations.
- The Rust compiler will warn you if you declare a mutable variable but don't modify it.

### Code Example

```rust
fn main() {
    let mut y = 10;
    println!("The initial value of y is: {}", y);
    
    y = 50;
    println!("The updated value of y is: {}", y);
}
```

## 3. Constants

Rust provides the `const` keyword for values that are truly constant throughout the program's execution.

### Theory

- Constants in Rust are always immutable.
- They must be annotated with a type.
- Their value must be known at compile-time and cannot be the result of a function call or any other computation performed at runtime.
- Constants can be declared in any scope, including the global scope.

### Code Example

```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("The maximum points allowed: {}", MAX_POINTS);
}
```

## 4. Underscores in Numeric Literals

Rust allows the use of underscores in numeric literals to improve readability.

### Theory

- Underscores can be inserted between digits in numeric literals.
- They don't affect the value of the number.
- This feature is particularly useful for large numbers.

### Code Example

```rust
fn main() {
    let large_number = 1_000_000;
    println!("A million is written as: {}", large_number);
}
```

## Conclusion

Understanding Rust's approach to variables and mutability is crucial for writing safe and efficient code. By defaulting to immutability and requiring explicit declarations for mutable variables, Rust encourages developers to think carefully about data mutability, leading to more robust and maintainable code.
