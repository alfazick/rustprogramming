# Rust Assignment: File Operations with Enums

## Objective
Create a Rust program that allows users to either create a new text file or rename an existing file. This assignment will help you practice using enums, pattern matching, and basic file operations in Rust.

## Requirements
1. Create an enum `FileOperation` with two variants:
   - `Create(String)` for creating a new file (String represents the file name)
   - `Rename(String, String)` for renaming a file (first String is the old name, second is the new name)

2. Implement a function `perform_operation` that takes a `FileOperation` and performs the corresponding action.

3. In the `main` function, prompt the user to choose an operation (create or rename) and input the necessary information.

4. Use pattern matching to handle the user's choice and call the `perform_operation` function with the appropriate `FileOperation` variant.

5. Use `unwrap()` for error handling to simplify the code.

## Starter Code

```rust
use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
            // TODO: Implement file creation logic
            println!("File '{}' created successfully.", filename);
        }
        FileOperation::Rename(old_name, new_name) => {
            // TODO: Implement file renaming logic
            println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
        }
    }
}

fn main() {
    println!("Choose an operation:");
    println!("1. Create a new file");
    println!("2. Rename an existing file");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            // TODO: Prompt for new filename and call perform_operation
        }
        "2" => {
            // TODO: Prompt for old and new filenames and call perform_operation
        }
        _ => println!("Invalid choice"),
    }
}
```

## Tasks
1. Complete the `perform_operation` function to actually create or rename files using the `std::fs` module. Use `unwrap()` for any operations that might fail.
2. Complete the `main` function to prompt for filenames and call `perform_operation` with the appropriate `FileOperation` variant.
3. Use `unwrap()` when reading user input to handle potential errors.
4. Before performing operations, check if files exist using `Path::new(filename).exists()`.

## Implementation Hints
- To check if a file exists: `Path::new("filename.txt").exists()`
- For file creation: `fs::File::create("filename.txt").unwrap();`
- For file renaming: `fs::rename("old_name.txt", "new_name.txt").unwrap();`
- To read user input: `io::stdin().read_line(&mut input_string).unwrap();`

## Bonus Challenges
1. Add a new variant to `FileOperation` for deleting files, and implement the corresponding functionality.
2. Implement a loop in `main` so the user can perform multiple operations without restarting the program.
3. Enhance the program's behavior based on file existence:
   - For creation: if the file already exists, ask the user if they want to overwrite it.
   - For renaming: if the old file doesn't exist or if the new filename already exists, ask the user if they want to proceed.

## Note on Error Handling
In this assignment, we're using `unwrap()` for simplicity. However, in real-world applications, it's generally better to handle errors more gracefully. As we progress in Rust, you'll learn about better error handling techniques using `Result`, `?` operator, and custom error types.

