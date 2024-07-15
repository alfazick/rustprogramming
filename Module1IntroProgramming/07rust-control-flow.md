# Rust: Control Flow

## Control Flow in Rust

Control flow in Rust allows you to control the execution of your program based on conditions. This section covers if-else expressions and demonstrates how they can be used as expressions to assign values.

### 1 If-Else Expressions

Rust uses if-else expressions for conditional execution. Here's an example of a basic if-else structure:

```rust
fn main() {
    let num = 5;
    
    if num % 3 == 0 && num % 5 == 0 {
        println!("FizzBuzz");
    } else if num % 3 == 0 {
        println!("Buzz");
    } else if num % 5 == 0 {
        println!("Fizz");
    } else {
        println!("{}", num);
    }
}
```

This is a standard if-else expression structure. However, it's important to note that in Rust, these are actually expressions, not just statements.

### 2 If as an Expression

In Rust, `if` is an expression, which means it returns a value. This allows for concise code that's similar to a ternary expression in other languages, but more readable.

Here's an example:

```rust
fn main() {
    let number = 5;
    
    let divisible_by_two = if number % 2 == 0 {
        "even" // no semicolon, because then it becomes an expression
    } else {
        "odd" // both arms or branches should evaluate to the same type
    };
    
    println!("Number {} is a type of {}", number, divisible_by_two);
}
```

In this example, the `if` expression is used to assign a value to `divisible_by_two`. This is similar to a ternary expression in languages like C++:

```cpp
string result = (n % 2 == 0) ? "even" : "odd";
```

But the Rust version is considered more readable, especially for more complex conditions.

Key points to remember:
1. The `if` expression doesn't need parentheses around the condition.
2. Both branches of the `if` expression must return the same type.
3. The last expression in each branch is returned (note the lack of semicolon).

While this covers the basics of control flow with `if` expressions, Rust has more advanced control flow constructs (like `match`) that we'll cover later when discussing Rust-specific features.

