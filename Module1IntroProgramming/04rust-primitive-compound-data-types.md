# Rust: Compound Data Types


## 2 Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. While Rust also supports more complex data structures like HashMaps, Vectors (dynamic arrays), and other collections, we'll focus on these two fundamental types for now.

### 2.1 Tuples

A tuple is a general way of grouping together a number of values with potentially different types into one compound type. Tuples have a fixed length and can't be resized after they're declared.

Key characteristics of tuples:
- Immutable by default
- Can store elements of different types
- Fixed length
- Useful for returning multiple values from a function

#### Creating and Using Tuples

```rust
fn main() {
    // Creating a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Destructuring (pattern matching)
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    // Accessing tuple elements using dot notation
    println!("First: {}, Second: {}, Third: {}", tup.0, tup.1, tup.2);

    // Tuple as a return type
    let (product, sum) = calculate(3, 4);
    println!("Product: {}, Sum: {}", product, sum);
}

fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x * y, x + y)
}
```

#### Tuple Use Cases

1. Returning multiple values from a function
2. Representing a concept with multiple properties but no need for named fields
3. Quick grouping of related data without creating a full struct

### 2.2 Arrays

An array is a collection of multiple values of the same type with a fixed length. Unlike some other languages, arrays in Rust have a fixed size once declared.

Key characteristics of arrays:
- Fixed length
- All elements must be of the same type
- Allocated on the stack (fast access, fixed size known at compile time)
- Useful when you want your data allocated on the stack rather than the heap

#### Creating and Using Arrays

```rust
fn main() {
    // Creating an array
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    
    // Accessing array elements
    let first = months[0];
    let second = months[1];
    println!("First month: {}, Second month: {}", first, second);
    
    // Array with explicit type and size
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Array with repeated values
    let repeated = [3; 5]; // Equivalent to [3, 3, 3, 3, 3]
    
    println!("Third number: {}, First repeated: {}", numbers[2], repeated[0]);

    // Printing entire array (debug format)
    println!("Numbers: {:?}", numbers);

    // Demonstrating bounds checking
    let index = 10;
    // Uncommenting the next line would cause a runtime panic
    // let element = numbers[index];
}
```

#### Array Bounds Checking

Rust performs bounds checking at runtime when accessing an array element using indexing. If you try to access an element at an index that doesn't exist, Rust will panic and stop execution, preventing buffer overread vulnerabilities common in languages like C and C++.

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let index: usize = 10;

    // This will compile, but panic at runtime
    // let element = arr[index];

    // Safe way to access elements
    if index < arr.len() {
        println!("Element at index {}: {}", index, arr[index]);
    } else {
        println!("Index out of bounds!");
    }
}
```

#### Use Cases for Arrays

1. When you have a fixed number of elements of the same type
2. When you want to ensure that the data is allocated on the stack for performance reasons
3. In systems programming where you need to interact with fixed-size buffers

### Comparison: Tuples vs Arrays

| Feature           | Tuples                    | Arrays                   |
|-------------------|---------------------------|--------------------------|
| Length            | Fixed                     | Fixed                    |
| Element types     | Can be different          | Must be the same         |
| Mutability        | Immutable by default      | Can be mutable           |
| Access syntax     | Dot notation (tuple.0)    | Square brackets (arr[0]) |
| Type declaration  | (T1, T2, ...)             | [T; N]                   |
| Use case          | Grouping heterogeneous data | Homogeneous collections |

## Conclusion

Understanding Rust's primitive compound types - tuples and arrays - is crucial for effective Rust programming. These types provide ways to group data efficiently, each with its own strengths. Tuples offer flexibility in types but are immutable, while arrays provide fast, stack-allocated storage for items of the same type. As you progress in Rust, you'll encounter more complex data structures like vectors and hashmaps, which build upon these fundamental concepts.
