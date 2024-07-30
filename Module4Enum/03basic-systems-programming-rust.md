# Basic Systems Programming with Rust: File Operations and Process Management

This guide demonstrates basic systems programming concepts using Rust's enums and pattern matching, focusing on file operations and process management using only the Rust standard library.

## 1. File Operations

```rust
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

enum FileOperation {
    Read,
    Write(String),
    Append(String),
}

fn perform_file_operation(path: &str, operation: FileOperation) {
    match operation {
        FileOperation::Read => {
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("File contents: {}", contents);
        },
        FileOperation::Write(content) => {
            let mut file = File::create(path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            println!("Content written to file.");
        },
        FileOperation::Append(content) => {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap();
            file.write_all(content.as_bytes()).unwrap();
            println!("Content appended to file.");
        },
    }
}

fn main() {
    let path = "test.txt";

    let operations = vec![
        FileOperation::Write(String::from("Hello, World!\n")),
        FileOperation::Append(String::from("This is a test.\n")),
        FileOperation::Read,
    ];

    for op in operations {
        perform_file_operation(path, op);
    }
}
```

This example demonstrates basic file operations using Rust's standard library. It uses an enum `FileOperation` to represent different file operations (Read, Write, Append) and pattern matching to execute the appropriate action for each operation.

## 2. Process Management

```rust
use std::process::{Command, Stdio};

enum ProcessOperation {
    Start(String),
    Pipe(String, String),
}

fn perform_process_operation(operation: ProcessOperation) {
    match operation {
        ProcessOperation::Start(cmd) => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .unwrap();

            println!("Process output: {}", String::from_utf8_lossy(&output.stdout));
        },
        ProcessOperation::Pipe(cmd1, cmd2) => {
            let process1 = Command::new("sh")
                .arg("-c")
                .arg(&cmd1)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd2)
                .stdin(Stdio::from(process1.stdout.unwrap()))
                .output()
                .unwrap();

            println!("Piped output: {}", String::from_utf8_lossy(&output.stdout));
        }
    }
}

fn main() {
    let operations = vec![
        ProcessOperation::Start(String::from("ls -l")),
        ProcessOperation::Pipe(String::from("echo 'Hello, World!'"), String::from("wc -w")),
    ];

    for op in operations {
        perform_process_operation(op);
    }
}
```

This example shows basic process management in Rust. It uses an enum `ProcessOperation` to represent different types of process operations (Start a single process, Pipe output between two processes) and pattern matching to execute the appropriate action for each operation.

These examples showcase the use of Rust's enums and pattern matching in systems programming contexts, demonstrating file operations and process management using only the Rust standard library.
