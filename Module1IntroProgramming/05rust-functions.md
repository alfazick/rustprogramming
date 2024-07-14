# Rust: Functions


## Functions in Rust

Functions are first-class citizens in Rust. The `fn` keyword is used to declare them, and the convention is to use snake_case for function names (surprisingly, even the compiler will enforce this).

### 1 Basic Function Syntax

Here's a plain function without parameters:

```rust
fn message() {
    println!("Just function");
}

fn main() {
    message();
}
```

### 2 Functions with Parameters

Functions can take parameters, similar to C++:

```rust
fn message_with_param(favnum: i32) {
    println!("My favorite number is: {}", favnum);
}

fn main() {
    message_with_param(777);
}
```

### 3 Statements and Expressions in Functions

Rust distinguishes between statements and expressions:
- Statements are instructions (e.g., `x = 5;`)
- Expressions are evaluated (e.g., `x = 5 + 10;`)

The key difference is that statements don't return a result value, while expressions do (which means expressions have a return type).

In practical terms, `x = y = 6` will not work in Rust. Also, statements are closed with a semicolon, but expressions aren't.

Here's an example demonstrating the difference:

```rust
fn main() {
    let x = 5;
    let y = {
        // Curly braces create a different scope
        let x = 3;
        x + 1 // Expression, returns the value
    };

    println!("X is: {}. Y is {}", x, y);
}
```

### 4 Functions with Return Values

Functions can return values. The return type is specified after an arrow `->`:

```rust
fn add_five(mynum: i32) -> i32 {
    mynum + 5
}

fn main() {
    println!("Calling add_five function, relying on expression return {}", add_five(100));
}
```

The return statement is usually explicitly used when you want to return from a function earlier, based on some condition. Otherwise, you can rely on the last expression in the function.

### 5 Type Safety in Functions

Rust enforces type safety in functions. Here's an example:

```rust
fn subtract_five(mynum: i32) -> u8 {
    mynum as u8 - 5 // mynum - 5 would not be correct
}

fn main() {
    println!("Calling subtract_five function, relying on expression return {}", subtract_five(6));
    // Try to pass 4 to see what happens
}
```

### 6 Comments in Rust

Rust supports both single-line and multi-line comments:

```rust
// Single line comment

/* 
Multiline comment 
or documentation comments
of course
*/
```

Understanding functions in Rust is crucial, especially if you want to learn from other people's code. The concepts of statements vs expressions and implicit returns are particularly important to grasp.
