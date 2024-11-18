# Rust Closures Practice Guide

## Overview
This guide introduces closures in Rust through practical exercises, ranging from basic operations to advanced use cases like lazy computation. Each task includes a concept explanation, examples, and assignments to help you practice effectively.

## Task 1: Basic Closure

### Concept: Closures for Arithmetic Operations
Closures are anonymous functions that can capture variables from their environment. They're useful for compact, inline logic.

### Example
```rust
let add = |x: i32, y: i32| x + y;
println!("5 + 3 = {}", add(5, 3)); // Output: 5 + 3 = 8
```

### Assignment
Write a closure named operation that multiplies two integers and returns the result. Test it with 10 * 5 and print the result.

### Code Stub
```rust
fn main() {
    let operation = |a: i32, b: i32| {
        // Your implementation here
    };

    println!("Result: {}", operation(10, 5));
}
```

## Task 2: Environment Capture

### Concept: Capturing and Modifying State
Closures can capture and modify variables from their environment, allowing state to persist between calls.

### Example
```rust
let mut total = 0;
let mut accumulate = || {
    total += 5;
    println!("Total: {}", total);
};
accumulate(); // Output: Total: 5
accumulate(); // Output: Total: 10
```

### Assignment
Write a closure named update inside a function track_changes. The closure should increment and print a counter each time it is called.

### Code Stub
```rust
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
    };

    update();
    update();
}

fn main() {
    track_changes();
}
```

## Task 3: Vector Transformation

### Concept: Applying Closures to Transform Vectors
Closures can be passed as arguments to functions for transforming elements in a vector.

### Example
Using map and collect:
```rust
fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn main() {
    let numbers = vec![1, 2, 3];
    let doubled = process_vector_with_map(numbers.clone(), |x| { /* your transform */ });
    let replaced = process_vector_with_map(numbers, |x| { /* your transform */ });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}
```

Using a for loop with a closure:
```rust
fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); // Apply the closure
    }
    result
}
```

### Assignment
Write a function `process_vector` that applies a closure to transform each element of a vector. Implement it in both ways:
1. Using map and collect
2. Using a for loop

### Code Stub
```rust
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Your implementation here
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}
```

## Task 5: Lazy Computation

### Concept: Deferring and Caching Expensive Computations
Closures can be used for lazy evaluation, where computation is deferred until needed. Results can also be cached for efficiency.

### Example
```rust
use std::{thread, time::Duration};

struct NumberCache<T>
where 
    T: Fn() -> i32,
{
    calculation: T,
    value: Option<i32>,
}

impl<T> NumberCache<T>
where 
    T: Fn() -> i32,
{
    fn new(calculation: T) -> Self {
        NumberCache {
            calculation,
            value: None,
        }
    }

    fn get_value(&mut self) -> i32 {
        match self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v
            }
            None => {
                println!("Computing (this will take 2 seconds)...");
                thread::sleep(Duration::from_secs(2));
                let v = (self.calculation)();
                self.value = Some(v);
                v
            }
        }
    }
}
```

### Assignment
Write a struct ComputeCache that accepts a closure during initialization. Cache the result after the first computation. Use thread::sleep to simulate an expensive computation.

### Code Stub
```rust
use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}
```

### Expected Output Format
```
First call:
Computing (this will take 2 seconds)...
Result: Hello, world!

Second call:
Retrieved from cache instantly!
Result (cached): Hello, world!
```

### Notes:
- Simulation of Expensive Computation: Use thread::sleep(Duration::from_secs(2)) to simulate a delay
- Caching Mechanism: Store the result in an Option and check if it's Some or None before computing
- User Feedback: Print messages indicating whether the computation is being performed or if the result is retrieved from the cache

## Summary of Key Concepts
- Closure Basics:
  - Anonymous functions that can capture variables from their environment
  - Useful for concise, inline operations
- State Capture:
  - Closures can capture and modify variables from their environment
  - Enables persistent state across multiple calls to the closure
- Vector Transformation:
  - Passing closures as arguments to functions allows for flexible transformations
  - Can implement using iterator methods (map, collect) or explicit loops (for)
- Lazy Evaluation:
  - Computations can be deferred until the value is actually needed
  - Caching results avoids unnecessary recomputation, improving efficiency
  - Simulating expensive operations helps demonstrate the benefits of caching

Reminder: Solve each task independently and use examples only for guidance. Practice is key to mastering closures in Rust!
