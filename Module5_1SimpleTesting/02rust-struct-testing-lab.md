# Rust Struct Testing Lab: Two-File Approach

## Introduction
In this lab, we'll create a Rust project that demonstrates how to test structs. We'll use two files: one for the struct definition and its tests, and another for the main program.

## Setup

1. Create a new Rust project:
   ```
   cargo new rust_struct_testing
   cd rust_struct_testing
   ```

2. Create a new file named `rectangle.rs` in the `src` directory:
   ```
   touch src/rectangle.rs
   ```

## Lab Steps

### Step 1: Define the Struct

Open `src/rectangle.rs` and add the following code:

```rust
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_creation() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.width, 5);
        assert_eq!(rect.height, 10);
    }

    #[test]
    fn test_area() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.area(), 50);
    }

    #[test]
    fn test_is_square() {
        let square = Rectangle::new(5, 5);
        let rectangle = Rectangle::new(5, 10);
        assert!(square.is_square());
        assert!(!rectangle.is_square());
    }
}
```

### Step 2: Update main.rs

Open `src/main.rs` and replace its contents with:

```rust
mod rectangle;

use rectangle::Rectangle;

fn main() {
    let rect = Rectangle::new(5, 10);
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Is square? {}", rect.is_square());
}
```

### Step 3: Run the Tests

Run the tests using:

```
cargo test
```

You should see output indicating that all tests passed.

### Step 4: Add More Tests

Let's add more tests to `src/rectangle.rs`. Add these tests to the `tests` module:

```rust
#[test]
fn test_zero_width() {
    let rect = Rectangle::new(0, 10);
    assert_eq!(rect.area(), 0);
}

#[test]
fn test_zero_height() {
    let rect = Rectangle::new(10, 0);
    assert_eq!(rect.area(), 0);
}

#[test]
fn test_equality() {
    let rect1 = Rectangle::new(5, 10);
    let rect2 = Rectangle::new(5, 10);
    let rect3 = Rectangle::new(10, 5);
    assert_eq!(rect1, rect2);
    assert_ne!(rect1, rect3);
}
```

Run the tests again to ensure they all pass.

### Step 5: Add a New Method and Test

Add a new method to the `Rectangle` impl in `src/rectangle.rs`:

```rust
pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
}
```

Now add a test for this new method:

```rust
#[test]
fn test_can_hold() {
    let larger = Rectangle::new(8, 7);
    let smaller = Rectangle::new(5, 1);
    assert!(larger.can_hold(&smaller));
    assert!(!smaller.can_hold(&larger));
}
```

Run the tests again to make sure the new test passes.

### Step 6: Update main.rs to Use New Method

Update `src/main.rs` to demonstrate the new `can_hold` method:

```rust
mod rectangle;

use rectangle::Rectangle;

fn main() {
    let rect1 = Rectangle::new(8, 7);
    let rect2 = Rectangle::new(5, 1);
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
}
```

### Step 7: Run the Program

Run the program to see the output:

```
cargo run
```

## Conclusion

In this lab, you've learned how to:
1. Define a struct and its methods in a separate file
2. Write tests for struct methods
3. Use different types of assertions in your tests
4. Add new methods and corresponding tests
5. Keep your main.rs clean by moving struct logic and tests to a separate file

This approach of separating your struct definition and tests into their own file helps keep your code organized and makes it easier to manage as your project grows.
