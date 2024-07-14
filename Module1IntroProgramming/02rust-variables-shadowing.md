# Rust: Shadowing

## Shadowing

Shadowing is a powerful feature in Rust that allows you to declare a new variable with the same name as a previous variable. This new variable shadows the previous one, effectively taking its place in the scope.

### Theory

- Shadowing is achieved by using the `let` keyword to declare a new variable with the same name.
- It allows you to change the type of the value but reuse the same name.
- Unlike mutation, shadowing creates a new variable, which can have a different type.
- Shadowing is particularly useful for transforming a value without needing to come up with different names.

### Advantages of Shadowing

1. **Type Transformation**: You can change the type of a variable while reusing its name, which is not possible with `mut`.
2. **Clarity**: It can make code more readable when a value undergoes a series of transformations.
3. **Scope Control**: The shadowed variable only exists within its scope, after which the original variable "reappears".

### Code Examples

#### Example 1: Type Conversion

```rust
fn main() {
    // Converting a string to a number
    let num = "25";
    let num: u32 = num.parse()
        .expect("Please provide a valid number!");
    
    println!("Parsed number: {}", num);
    
    // Further transformations
    let num = num + 25;
    let num = num * 2;
    
    println!("Final value of num: {}", num);
}
```

In this example, `num` starts as a string, is converted to a `u32`, and then undergoes arithmetic operations. Each step creates a new variable that shadows the previous one.

#### Example 2: String Length

```rust
fn main() {
    let word = "UTRGV";
    let word = word.len();
    
    println!("Length of the word: {}", word);
}
```

Here, `word` initially holds a string, but is then shadowed by its length (an integer).

### Shadowing vs. Mutation

It's important to understand the difference between shadowing and mutation:

```rust
fn main() {
    // Shadowing
    let x = 5;
    let x = x + 1;  // Creates a new variable
    
    // Mutation
    let mut y = 5;
    y = y + 1;  // Modifies the existing variable
    
    println!("x: {}, y: {}", x, y);
}
```

Shadowing creates a new variable each time, while mutation changes the value of the existing variable.

### Limitations and Considerations

1. Shadowing doesn't work with `mut` variables when changing types:

```rust
fn main() {
    let mut word = "UTRGV";
    // word = word.len();  // This would cause a compilation error
    
    // Instead, you would shadow the variable:
    let word = word.len();
    
    println!("Length: {}", word);
}
```

2. Shadowing occurs at compile-time, so there's no runtime performance impact.

3. While powerful, excessive use of shadowing can make code harder to follow. Use it judiciously to enhance readability, not complicate it.

## Conclusion

Shadowing in Rust provides a flexible way to reuse variable names, especially when transforming data. It complements Rust's strong type system and immutability-by-default approach, offering developers a tool to write cleaner, more expressive code. Understanding when to use shadowing versus mutation is key to mastering Rust's variable management system.
