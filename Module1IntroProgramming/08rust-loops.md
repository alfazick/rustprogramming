# Rust: Loops

## Loops in Rust

Loops are used to execute a block of code repeatedly. Rust provides three types of loops: `loop`, `while`, and `for`.

### 1 The `loop` Keyword

The `loop` keyword creates an infinite loop that will run until it hits a `break` statement.

```rust
fn main() {
    let mut num = 0;
    loop {
        println!("{}", num);
        num += 1;

        if num == 10 {
            break;
        }
    }
}
```

#### Returning Values from Loops

One unique feature of the `loop` construct in Rust is that it can return values. This is particularly useful when you need to use a value that was computed within the loop scope.

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is: {}", result);
    println!("Counter is: {}", counter);
}
```

### 2 While Loops

Rust also supports standard while loops:

```rust
fn main() {
    let mut counter = 10;
    while counter != 0 {
        println!("Count down : {}", counter);
        counter -= 1;
    }
}
```

### 3 For Loops

For loops in Rust are often used to iterate through collections.

#### Iterating with `iter()`

```rust
fn main() {
    let nums = [1, 2, 3, 4, 5];

    // for each loop
    for num in nums.iter() {
        println!("{} ", num);
    }
}
```

#### Classic Index-based Loop

```rust
fn main() {
    let nums = [1, 2, 3, 4, 5];

    // classic loop 
    for idx in 0..nums.len() { 
        // start inclusive ..end exclusive (it's fancy notation for range)
        println!("Element under idx {} := {}", idx, nums[idx]);
    }
}
```

Note: The `0..nums.len()` syntax creates a range that is inclusive at the start and exclusive at the end. This is a common notation for ranges in languages like Swift, Python, and Kotlin.

Remember, when in doubt about the type of any expression in Rust, you can hover your mouse over the code in most IDEs to see more information.

