# Assignment: Interactive File Operations in Rust Using Command::new() and Enums

## Objective

Create an interactive Rust program that performs basic file operations (ls, cat, create, remove, pwd) by executing system commands using Command::new(). Use enums to represent different file operations. The program should accept user input via a menu system until the user decides to exit.

## Instructions

### 1. Define the FileOperation Enum

Create an enum FileOperation with variants for each operation:

```rust
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}
```

### 2. Implement User Input Loop with Menu

Your program should:

- Run in a loop, continuously displaying a menu of options to the user.
- Allow the user to select an operation by entering a number.
- Based on the selection, prompt for additional arguments if necessary.
- Map the user input to the corresponding FileOperation variant.
- Call the function to perform the operation.
- Display the output or any messages to the user.
- Handle minimal error cases (e.g., invalid menu option).

Example menu:

```
File Operations Menu:
1. List files in a directory
2. Display file contents
3. Create a new file
4. Remove a file
5. Print working directory
0. Exit

Enter your choice (0-5):
```

### 3. Implement the perform_operation Function

Write a function perform_operation that takes a FileOperation and performs the corresponding system command using Command::new().

Function Signature:

```rust
use std::process::Command;

fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
}
```

Details:

- List:
  ```rust
  Command::new("ls").arg(directory_path).status().expect("Failed to execute ls");
  ```

- Display:
  ```rust
  Command::new("cat").arg(file_path).status().expect("Failed to execute cat");
  ```

- Create:
  ```rust
  let command = format!("echo '{}' > {}", content, file_path);
  Command::new("sh").arg("-c").arg(command).status().expect("Failed to create file");
  ```

- Remove:
  ```rust
  Command::new("rm").arg(file_path).status().expect("Failed to remove file");
  ```

- Pwd:
  ```rust
  Command::new("pwd").status().expect("Failed to execute pwd");
  ```

### 4. Example Usage of Command::new("sh").arg("-c").arg(user_command)

Here's an example of how to use this pattern for creating a file:
```rust
fn create_file(file_path: &str, content: &str) {
    let user_command = format!("echo '{}' > {}", content, file_path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&user_command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("File created successfully");
    } else {
        eprintln!("Failed to create file");
    }
}
```

This pattern is useful when you need to execute shell commands that involve piping, redirection, or other shell-specific features.

### 5. Implement Minimal Error Handling

- Handle invalid menu selections by notifying the user and prompting again.
- If a command fails, print a simple error message and continue.

### 6. Program Termination

- The program should continue running until the user selects the exit option (0).
- When exiting, display a goodbye message and terminate the program.


## Sample Interaction

```
Welcome to the File Operations Program!

File Operations Menu:
1. List files in a directory
2. Display file contents
3. Create a new file
4. Remove a file
5. Print working directory
0. Exit

Enter your choice (0-5): 5

Current working directory: /home/user/projects

File Operations Menu:
1. List files in a directory
2. Display file contents
3. Create a new file
4. Remove a file
5. Print working directory
0. Exit

Enter your choice (0-5): 1
Enter directory path: .

[Output of ls command]

File Operations Menu:
...

Enter your choice (0-5): 3
Enter file path: example.txt
Enter content: Hello, world!

File 'example.txt' created successfully.

File Operations Menu:
...

Enter your choice (0-5): 2
Enter file path: example.txt

[Output of cat command showing "Hello, world!"]

File Operations Menu:
...

Enter your choice (0-5): 4
Enter file path: example.txt

File 'example.txt' removed successfully.

File Operations Menu:
...

Enter your choice (0-5): 0

Goodbye!
```

