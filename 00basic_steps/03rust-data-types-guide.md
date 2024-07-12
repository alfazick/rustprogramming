# Comprehensive Guide to Rust Data Types

Rust provides a rich set of data types that allow for precise control over how data is stored and manipulated. This guide covers the main categories of data types in Rust, along with code examples for each.

## Table of Contents
1. [Scalar Types](#scalar-types)
2. [Compound Types](#compound-types)
3. [Text Types](#text-types)
4. [Pointer Types](#pointer-types)
5. [Option Types](#option-types)
6. [Result Types](#result-types)
7. [Special Types](#special-types)

## Scalar Types

Scalar types represent single values. Rust has four primary scalar types:

### Integers

Rust provides signed and unsigned integers of various sizes:

```rust
fn main() {
    let a: i8 = -128;  // Signed 8-bit integer
    let b: u8 = 255;   // Unsigned 8-bit integer
    let c: i32 = 2_147_483_647;  // Signed 32-bit integer
    let d: u64 = 18_446_744_073_709_551_615;  // Unsigned 64-bit integer
    
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}
```

### Floating-Point Numbers

Rust has two floating-point types:

```rust
fn main() {
    let x: f32 = 3.14;  // 32-bit float
    let y: f64 = 2.71828;  // 64-bit float
    
    println!("x: {}, y: {}", x, y);
}
```

### Booleans

The boolean type in Rust can be either `true` or `false`:

```rust
fn main() {
    let is_active: bool = true;
    let is_greater = 10 > 5;
    
    println!("is_active: {}, is_greater: {}", is_active, is_greater);
}
```

### Characters

The `char` type represents a Unicode scalar value:

```rust
fn main() {
    let letter: char = 'A';
    let emoji: char = '😀';
    
    println!("letter: {}, emoji: {}", letter, emoji);
}
```

## Compound Types

Compound types can group multiple values into one type.

### Arrays

Arrays have a fixed length and elements of the same type:

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3];  // [0, 0, 0]
    
    println!("First number: {}", numbers[0]);
    println!("Array length: {}", zeros.len());
}
```

### Tuples

Tuples can have elements of different types:

```rust
fn main() {
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true);
    
    println!("Name: {}, Age: {}, Employed: {}", person.0, person.1, person.2);
}
```

### Vectors

Vectors are dynamic arrays that can grow or shrink:

```rust
fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    vec.push(4);
    
    println!("Vector: {:?}", vec);
    println!("Vector length: {}", vec.len());
}
```

## Text Types

Rust has two main string types:

### String

A growable, mutable, owned UTF-8 encoded string:

```rust
fn main() {
    let mut s: String = String::from("Hello");
    s.push_str(", world!");
    
    println!("{}", s);
}
```

### &str

An immutable reference to a string slice:

```rust
fn main() {
    let s: &str = "Hello, world!";
    
    println!("{}", s);
}
```

## Pointer Types

Rust uses pointers for memory management and safe concurrent programming.

### References

References allow you to refer to a value without taking ownership:

```rust
fn main() {
    let x = 5;
    let y = &x;
    
    println!("x: {}, y: {}", x, *y);
}
```

### Box<T>

`Box<T>` is used for heap allocation:

```rust
fn main() {
    let boxed_int: Box<i32> = Box::new(5);
    
    println!("Boxed value: {}", *boxed_int);
}
```

## Option Types

`Option<T>` represents an optional value:

```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
    
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number"),
    }
    
    match no_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number"),
    }
}
```

## Result Types

`Result<T, E>` is used for returning and propagating errors:

```rust
use std::fs::File;

fn main() {
    let file_result = File::open("nonexistent.txt");
    
    match file_result {
        Ok(file) => println!("File opened successfully"),
        Err(error) => println!("Error opening file: {:?}", error),
    }
}
```

## Special Types

### Function Pointers

Function pointers allow you to store and pass around references to functions:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let operation: fn(i32, i32) -> i32 = add;
    let result = operation(5, 3);
    
    println!("Result: {}", result);
}
```

This guide covers the main data types in Rust. Each type has its own use cases and characteristics, allowing for efficient and safe programming in various scenarios.
