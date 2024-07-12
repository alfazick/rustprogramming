# Rust Programming Fundamentals

This guide covers essential Rust programming concepts, including variables, control flow, and data types. Each section includes explanations and code examples to illustrate the concepts.

## Table of Contents
1. [Constants and Static Memory](#constants-and-static-memory)
2. [Working with Variables](#working-with-variables)
3. [Control Flow](#control-flow)
4. [Arrays](#arrays)
5. [Working with Dynamic Variables](#working-with-dynamic-variables)

## Constants and Static Memory

Constants in Rust are immutable values that are stored in static memory (the stack). They are defined using the `const` keyword and must have their type explicitly declared.

```rust
const OUR_COURSE: &str = "Rust Programming Fundamentals";

fn main() {
    println!("Welcome to {}", OUR_COURSE);
}
```

Constants are useful for values that remain unchanged throughout the program's execution.

## Working with Variables

Variables in Rust can be either mutable or immutable. By default, variables are immutable unless declared with the `mut` keyword.

```rust
fn main() {
    // Immutable variable
    let x: i32 = 5;
    
    // Mutable variable
    let mut y: f64 = 3.14;
    y = 2.71;
    
    // Type inference
    let is_true = true;
    
    // Character type
    let my_char: char = 'Z';
    
    println!("x: {}, y: {}, is_true: {}, my_char: {}", x, y, is_true, my_char);
}
```

Rust supports various primitive types, including integers (`i32`, `u64`, etc.), floating-point numbers (`f32`, `f64`), booleans (`bool`), and characters (`char`).

## Control Flow

Rust provides several constructs for control flow, including loops and conditional statements.

### Loops

```rust
fn main() {
    // For loop with range
    for i in 0..5 {
        println!("i is: {}", i);
    }
    
    // For loop with inclusive range
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();
    
    // While loop
    let mut counter = 0;
    while counter < 5 {
        println!("counter is: {}", counter);
        counter += 1;
    }
    
    // Loop with break
    let mut sum = 0;
    loop {
        sum += 1;
        if sum > 10 {
            break;
        }
    }
    println!("Final sum: {}", sum);
}
```

### Conditional Statements

```rust
fn main() {
    let number = 7;
    
    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is equal to 5");
    } else {
        println!("Number is greater than 5");
    }
    
    // Match statement
    match number {
        1 => println!("One"),
        2 | 3 | 4 => println!("Two, three, or four"),
        5..=10 => println!("Five to ten"),
        _ => println!("Something else"),
    }
}
```

## Arrays

Arrays in Rust have a fixed size and store elements of the same type.

```rust
fn main() {
    // Fixed-size array initialization
    let int_array: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Array with repeated values
    let zeros: [i32; 3] = [0; 3];
    
    // Accessing array elements
    println!("First element: {}", int_array[0]);
    
    // Iterating over an array
    for num in int_array.iter() {
        print!("{} ", num);
    }
    println!();
    
    // Array length
    println!("Array length: {}", int_array.len());
}
```

## Working with Dynamic Variables

Rust provides dynamic data structures like `String` and `Vec` for cases where the size may change at runtime.

### Strings

```rust
fn main() {
    // Creating a String
    let mut name = String::from("Rust");
    
    // Appending to a String
    name.push_str(" Programming");
    
    // Converting &str to String
    let static_str: &str = "Hello, ";
    let dynamic_string: String = static_str.to_string() + &name;
    
    println!("{}", dynamic_string);
}
```

### Vectors

```rust
fn main() {
    // Creating a vector
    let mut numbers: Vec<i32> = Vec::new();
    
    // Adding elements
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    
    // Accessing elements
    println!("First element: {}", numbers[0]);
    
    // Iterating over a vector
    for num in &numbers {
        print!("{} ", num);
    }
    println!();
    
    // Vector with initial values
    let mut chars: Vec<char> = vec!['a', 'b', 'c'];
    chars.push('d');
    
    println!("Chars: {:?}", chars);
}
```

This guide covers the fundamental concepts of Rust programming. As you progress, you'll encounter more advanced topics like ownership, borrowing, and lifetimes, which are crucial for Rust's memory safety guarantees.
