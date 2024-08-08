# Higher-Order Functions in Rust: Blended Example (Strictly No Closures)

## Introduction

Higher-order functions in Rust embody the principle of "separation of concerns." They allow us to define "what needs to be done" while leaving the specifics of "how it's done" to be decided later. This document presents a blended example that demonstrates this concept using function pointers and strictly avoiding closures.

## The Basic Example

Let's create a scenario where we process data in different ways. We'll define a higher-order function that outlines the general process, and then provide specific functions to do the actual work.

```rust
// Define our data structure
struct Data {
    value: i32,
}

// Higher-order function: defines what needs to be done
fn process_data(data: &mut [Data], operation: fn(&mut Data)) {
    for item in data.iter_mut() {
        operation(item);
    }
}

// Specific operations: actual functions which do the work
fn double_value(data: &mut Data) {
    data.value *= 2;
}

fn square_value(data: &mut Data) {
    data.value = data.value * data.value;
}

// Helper function to print values without closures
fn print_values(items: &[Data]) {
    print!("Values: ");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", item.value);
    }
    println!();
}

fn main() {
    let mut items = vec![
        Data { value: 1 },
        Data { value: 2 },
        Data { value: 3 },
        Data { value: 4 },
        Data { value: 5 },
    ];
    
    // The specific operation is decided here
    print!("Original ");
    print_values(&items);
    
    process_data(&mut items, double_value);
    print!("After doubling: ");
    print_values(&items);
    
    // We can easily switch to a different operation
    process_data(&mut items, square_value);
    print!("After squaring: ");
    print_values(&items);
}
```

## Explanation

Let's break down this example to understand how it demonstrates higher-order functions without any closures:

1. **Data Structure**:
   ```rust
   struct Data {
       value: i32,
   }
   ```
   - We define a simple `Data` struct to hold our values.

2. **Higher-Order Function**:
   ```rust
   fn process_data(data: &mut [Data], operation: fn(&mut Data))
   ```
   - This is our higher-order function. It defines "what needs to be done":
     - Iterate over a slice of `Data`
     - Apply an operation to each item
   - The `operation` parameter is a function pointer that takes a mutable reference to `Data`
   - This function doesn't know or care about the specific operation; it just applies it

3. **Specific Operations**:
   ```rust
   fn double_value(data: &mut Data) {
       data.value *= 2;
   }

   fn square_value(data: &mut Data) {
       data.value = data.value * data.value;
   }
   ```
   - These functions define the "actual work" that will be done
   - They match the signature required by `process_data`: they take a `&mut Data` and return nothing
   - These are decided later and can be easily changed or extended

4. **Helper Function for Printing**:
   ```rust
   fn print_values(items: &[Data]) {
       print!("Values: ");
       for (i, item) in items.iter().enumerate() {
           if i > 0 {
               print!(", ");
           }
           print!("{}", item.value);
       }
       println!();
   }
   ```
   - This function prints the values of our `Data` items without using any closures


5. **Using the Higher-Order Function**:
   ```rust
   let mut items = vec![
       Data { value: 1 },
       Data { value: 2 },
       Data { value: 3 },
       Data { value: 4 },
       Data { value: 5 },
   ];
   
   print_values(&items);
   process_data(&mut items, double_value);
   print_values(&items);
   process_data(&mut items, square_value);
   print_values(&items);
   ```
   - We create a vector of `Data` items
   - We call `process_data` twice:
     - First with `double_value`
     - Then with `square_value`
   - We use `print_values` to display the results after each operation

## Key Points

1. **Strictly No Closures**: This example completely avoids closures, using only function pointers and regular functions.

2. **Separation of Concerns**: The `process_data` function is concerned only with iterating over data and applying an operation. It doesn't need to know what that operation does.

3. **Flexibility**: We can easily add new operations (like `triple_value` or `add_ten`) without changing the `process_data` function.

4. **Reusability**: The same `process_data` function can be used with any function that matches the required signature.

5. **Late Binding**: The specific operation is "decided later" - at the point where we call `process_data`, not when we define it.

This pattern is particularly useful in systems programming scenarios, where you might define standard interfaces (like `process_data`) that can work with a variety of specific implementations provided later.



