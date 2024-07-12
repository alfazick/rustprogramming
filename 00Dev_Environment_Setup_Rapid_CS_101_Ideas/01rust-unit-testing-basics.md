# Rust Unit Testing Basics

Unit testing is a crucial part of software development in Rust. Here are the fundamental concepts:

## Test Module

Tests in Rust are typically organized in a test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test functions go here
}
```

The `#[cfg(test)]` attribute ensures this module is only compiled when running tests.

## Test Functions

Individual test functions are marked with the `#[test]` attribute:

```rust
#[test]
fn test_function_name() {
    // Test code here
}
```

## Assertions

Rust provides several assertion macros for testing:

- `assert!(expression)`: Fails if the expression is false
- `assert_eq!(left, right)`: Fails if the two expressions are not equal
- `assert_ne!(left, right)`: Fails if the two expressions are equal

Example:

```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}
```

## Running Tests

Use the `cargo test` command to run all tests in your project.
