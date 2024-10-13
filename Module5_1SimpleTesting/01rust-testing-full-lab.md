# Comprehensive Rust Testing Lab: Using main.rs

## Introduction
This lab will guide you through the process of writing and running tests in Rust, focusing exclusively on using `main.rs`. By the end, you'll have a solid understanding of basic testing concepts in Rust.

## Setup
1. Open your terminal.
2. Create a new Rust project:
   ```
   cargo new rust_testing_lab
   cd rust_testing_lab
   ```
3. Open `src/main.rs` in your preferred text editor.

## Lab 1: Your First Test

1. Replace the contents of `src/main.rs` with:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

2. Run your tests:
   ```
   cargo test
   ```
3. You should see output indicating that the test passed.

## Lab 2: Testing Edge Cases

1. Add these tests to the `tests` module:

```rust
#[test]
fn test_add_negative() {
    assert_eq!(add(-2, -2), -4);
}

#[test]
fn test_add_zero() {
    assert_eq!(add(0, 0), 0);
}
```

2. Run the tests. They should all pass.

## Lab 3: Introducing assert! and assert_ne!

1. Add a new function to `main.rs`:

```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0
}
```

2. Add these tests:

```rust
#[test]
fn test_is_even() {
    assert!(is_even(2));
    assert!(!is_even(3));
}

#[test]
fn test_not_equal() {
    assert_ne!(add(2, 2), 5);
}
```

3. Run the tests. They should all pass.

## Lab 4: Testing for Panics

1. Add a new function:

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}
```

2. Add this test:

```rust
#[test]
#[should_panic(expected = "Cannot divide by zero")]
fn test_divide_by_zero() {
    divide(10, 0);
}
```

3. Run the tests. They should all pass.

## Lab 5: Parameterized Tests

1. Add this test to explore multiple cases:

```rust
#[test]
fn test_add_multiple() {
    let test_cases = vec![
        (1, 1, 2),
        (0, 0, 0),
        (-1, 1, 0),
        (100, -50, 50)
    ];
    
    for (a, b, expected) in test_cases {
        assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
    }
}
```

2. Run the tests. They should all pass.

## Lab 6: Custom Failure Messages

1. Add this test to demonstrate custom failure messages:

```rust
#[test]
fn test_is_even_verbose() {
    let number = 5;
    assert!(
        is_even(number),
        "Expected {} to be even, but it was odd",
        number
    );
}
```

2. Run the tests. This new test should fail with your custom message.

## Lab 7: Organizing Tests

1. Add this nested module to your `tests` module:

```rust
mod arithmetic_tests {
    use super::*;

    #[test]
    fn test_complex_add() {
        assert_eq!(add(add(1, 2), add(3, 4)), 10);
    }

    #[test]
    fn test_even_arithmetic() {
        assert!(is_even(add(2, 2)));
        assert!(!is_even(add(2, 3)));
    }
}
```

2. Run the tests. All tests, including those in the nested module, should run.

## Final main.rs

Your final `main.rs` should look like this:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
    println!("Is 4 even? {}", is_even(4));
    println!("10 / 2 = {}", divide(10, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -2), -4);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(!is_even(3));
    }

    #[test]
    fn test_not_equal() {
        assert_ne!(add(2, 2), 5);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }

    #[test]
    fn test_add_multiple() {
        let test_cases = vec![
            (1, 1, 2),
            (0, 0, 0),
            (-1, 1, 0),
            (100, -50, 50)
        ];
        
        for (a, b, expected) in test_cases {
            assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
        }
    }

    #[test]
    fn test_is_even_verbose() {
        let number = 5;
        assert!(
            is_even(number),
            "Expected {} to be even, but it was odd",
            number
        );
    }

    mod arithmetic_tests {
        use super::*;

        #[test]
        fn test_complex_add() {
            assert_eq!(add(add(1, 2), add(3, 4)), 10);
        }

        #[test]
        fn test_even_arithmetic() {
            assert!(is_even(add(2, 2)));
            assert!(!is_even(add(2, 3)));
        }
    }
}
```

## Conclusion

You've now explored various aspects of testing in Rust:
- Basic assertions with `assert_eq!`, `assert!`, and `assert_ne!`
- Testing for panics with `#[should_panic]`
- Parameterized tests
- Custom failure messages
- Organizing tests with nested modules

All of this was done within a single `main.rs` file, demonstrating how you can start testing in Rust without needing to set up complex project structures.
