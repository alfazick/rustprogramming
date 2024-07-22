# Rust Assignment: Book Catalog

## Objective
Create a simple Book Catalog system in Rust that demonstrates struct usage and file I/O operations.

## Requirements

1. Create a `Book` struct with the following fields:
   - `title`: String
   - `author`: String
   - `year`: u16

2. Implement the following functions:
   - `save_books(books: &Vec<Book>, filename: &str)`: Saves all books to a file.
   - `load_books(filename: &str) -> Vec<Book>`: Loads books from a file.

3. In the `main` function:
   - Create a few `Book` instances
   - Save the books to a file
   - Load the books from the file and print them

## Starter Code

```rust
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}
```

## Tasks

1. Complete the `save_books()` function to write all books to a file. Each book should be on a separate line with fields separated by commas.
2. Implement the `load_books()` function to read books from the file and return a `Vec<Book>`.
3. Run the program and verify that it correctly saves and loads the books.

