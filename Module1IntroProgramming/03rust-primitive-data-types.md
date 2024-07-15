# Rust: Primitive Data Types

## Data Types in Rust

Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. However, in cases when many types are possible, we must add a type annotation.

### Theory

- Rust has two main categories of data types: scalar and compound.
- The compiler can often infer types, but explicit type annotations are sometimes necessary.
- Rust's strong type system and compile-time checks prevent many common errors found in other languages.

### 1 Scalar Types

Scalar types represent a single value. Rust has four primary scalar types:

#### 1.1 Integers

Integers are numbers without a fractional component. Rust provides several integer types with different sizes and signedness:

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

Notes:
- `isize` and `usize` depend on the architecture of the computer your program is running on (64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture).
- Integer overflow is checked in debug builds and causes a panic. In release builds, overflow wraps around.

```rust
fn main() {
    let a: i32 = 98_222;    // Decimal
    let b: i32 = 0xff;      // Hex
    let c: i32 = 0o77;      // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A';       // Byte (u8 only)

    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    // Overflow example (uncomment to see the panic in debug mode)
    // let mut money: u8 = 0;
    // money -= 1;
}
```

#### 1.2 Floating-Point Numbers

Rust has two floating-point types: `f32` and `f64` (default).

```rust
fn main() {
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32

    println!("x: {}, y: {}", x, y);

    // Precision example
    let precise: f64 = 1.4567891016797080012345678998764;
    let less_precise: f32 = 1.4567891016797080012345678998764;

    println!("f64: {}", precise);
    println!("f32: {}", less_precise);
}
```

#### 1.3 Booleans

The boolean type in Rust has two possible values: `true` and `false`.

```rust
fn main() {
    let t = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);
}
```

#### 1.4 Characters

Rust's `char` type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, cat: {}", c, z, heart_eyed_cat);
}
```


## Conclusion

Understanding Rust's type system is crucial for writing efficient and safe code. The language's strong static typing, combined with its ownership system (which we'll cover later), provides powerful guarantees about program correctness at compile time. This section covered the basic scalar  in Rust, but there's much more to explore, including enums, structs, and more complex compound types.
