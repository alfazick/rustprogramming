# Rust Pattern Matching Example

The following code demonstrates a simple pattern matching example in Rust using the `match` expression:

```rust
fn pattern_match_simple() {
    let num = 3;
    let letter = match num {
        1 => 'A',
        2 => 'B',
        3 => {
            (64 + 1 + 2 as u8) as char
        },
        _ => '#', // rust will not guess
    };
    println!("{}", letter);
}
```

Let's break down this function:

1. We start by declaring a variable `num` and assigning it the value 3.

2. We then use a `match` expression to assign a value to `letter` based on the value of `num`.

3. The `match` expression in Rust is exhaustive, meaning it must cover all possible values of the matched variable.

4. Each arm of the `match` expression consists of a pattern and the code to run if the pattern matches, separated by `=>`.

5. In this example:
   - If `num` is 1, `letter` will be assigned 'A'.
   - If `num` is 2, `letter` will be assigned 'B'.
   - If `num` is 3, a block of code is executed that performs some arithmetic and type casting:
     - `64 + 1 + 2` is evaluated (resulting in 67)
     - This is cast to a `u8` (unsigned 8-bit integer)
     - The result is then cast to a `char`
     - 67 as a char is 'C' in ASCII
   - The `_` is a catch-all pattern that matches any value not specifically handled above. In this case, it assigns '#' to `letter`.

6. Finally, we print the value of `letter`.

In this specific example, since `num` is 3, the third arm of the `match` expression will be executed, and `letter` will be assigned 'C'. The program will then print "C".

This example showcases how `match` can be used for simple value matching, executing blocks of code, and providing a default case. It's a powerful feature in Rust for control flow and pattern matching.

