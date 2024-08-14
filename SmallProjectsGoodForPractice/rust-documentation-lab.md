# Rust Documentation Practice Lab

## Objective
In this lab, you'll create a simple Rust library, write documentation for it, generate documentation files, and serve them using Actix-web.

## Prerequisites
- Rust and Cargo installed
- Basic knowledge of Rust syntax

## Lab Steps

### 1. Create a New Rust Library Project

```bash
cargo new rust_doc_lab --lib
cd rust_doc_lab
```

### 2. Write a Simple Rust Library

Edit `src/lib.rs`:

```rust
/// Performs arithmetic operations
pub mod math {
    /// Adds two numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_doc_lab::math::add;
    /// assert_eq!(add(2, 3), 5);
    /// ```
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Multiplies two numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_doc_lab::math::multiply;
    /// assert_eq!(multiply(2, 3), 6);
    /// ```
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

/// Performs string operations
pub mod strings {
    /// Concatenates two strings
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_doc_lab::strings::concatenate;
    /// assert_eq!(concatenate("Hello, ", "World!"), "Hello, World!");
    /// ```
    pub fn concatenate(a: &str, b: &str) -> String {
        format!("{}{}", a, b)
    }
}
```

### 3. Add Crate-Level Documentation

At the top of `src/lib.rs`, add:

```rust
//! # Rust Documentation Lab Library
//!
//! This library provides simple arithmetic and string operations.
//!
//! ## Modules
//!
//! - `math`: Provides basic arithmetic operations
//! - `strings`: Provides string manipulation functions
```

### 4. Generate Documentation

Run:

```bash
cargo doc --no-deps --open
```

This command generates the documentation and opens it in your default web browser.

### 5. Create a Web Server to Serve Documentation

Update `Cargo.toml`:

```toml
[package]
name = "rust_doc_lab"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.3"
actix-files = "0.6"

[[bin]]
name = "doc_server"
path = "src/main.rs"
```

Create `src/main.rs`:

```rust
use actix_web::{App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Documentation server running at http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new().service(
            fs::Files::new("/", "./target/doc")
                .show_files_listing()
                .index_file("index.html"),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

### 6. Run the Documentation Server

```bash
cargo run --bin doc_server
```

Visit `http://0.0.0.0:8080` in your web browser to view the documentation.

## Practice Exercises

1. Add a new function `subtract` to the `math` module. Include a doctest that demonstrates its use.

2. Create a new module called `utils` with a function `is_palindrome` that checks if a string is a palindrome. Write comprehensive documentation for this function, including examples and edge cases.

3. Add a section to the crate-level documentation explaining how to use the library in a project.

4. Modify the web server to log each request to the console.



## Conclusion

In this lab, you've learned how to:
- Write documentation comments in Rust
- Use doctests to provide examples
- Generate HTML documentation
- Serve documentation using a web server

These skills are crucial for creating well-documented and user-friendly Rust libraries.

