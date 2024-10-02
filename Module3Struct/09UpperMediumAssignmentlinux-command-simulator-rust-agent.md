# Linux Command Simulator Agent - Rust Assignment

## Objective
Create a Rust program that simulates a Linux command-line interface. The program should accept Linux commands from the user, execute them in the background, save the commands and their outputs to a file, and display the history of commands when the user chooses to stop.

## Requirements
1. Accept Linux commands from the user.
2. Execute commands in the background using `std::process::Command`.
3. Save both the command and its output to a file.
4. Continue accepting commands until the user chooses to stop.
5. Display all previously entered commands and their results upon stopping.

## Starter Code Structure

```rust
use std::process::Command;
use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct LinuxAgent {
    path: String,
}

impl LinuxAgent {
    fn new() -> LinuxAgent {
        // TODO: Implement constructor
        // Initialize the LinuxAgent and create the file for storing command history
    }

    fn executing_os_commands_linux(&self, command_full: &str) {
        // TODO: Implement command execution
        // 1. Parse the command and its arguments
        // 2. Execute the command using std::process::Command
        // 3. Capture the output
        // 4. Save the results
    }

    fn accept_linux_command_from_user() -> String {
        // TODO: Implement user input
        // Prompt the user for a Linux command and return it
    }

    fn save_results(&self, content: String) {
        // TODO: Implement saving results
        // Append the command and its output to the file
    }

    fn show_results(&self) {
        // TODO: Implement showing results
        // Read and display the contents of the command history file
    }
}

fn main() {
    // TODO: Implement the main program loop
    // 1. Create a LinuxAgent instance
    // 2. Enter a loop to accept and execute commands
    // 3. Break the loop when the user wants to stop
    // 4. Show the results
}
```

## Implementation Tasks

1. **Constructor (new function)**
   - Initialize the LinuxAgent with the path to the command history file.
   - Create the file if it doesn't exist.

2. **Command Execution (executing_os_commands_linux function)**
   - Parse the input command string to separate the command and its arguments.
   - Use `std::process::Command` to execute the command.
   - Capture both stdout and stderr.
   - Handle potential errors in command execution.

3. **User Input (accept_linux_command_from_user function)**
   - Prompt the user to enter a Linux command.
   - Read the input from stdin.
   - Return the command as a String.

4. **Saving Results (save_results function)**
   - Open the file in append mode.
   - Write the command and its output to the file.
   - Include timestamps or sequential numbering for better readability.

5. **Displaying Results (show_results function)**
   - Open and read the command history file.
   - Print the contents to the console in a readable format.

6. **Main Program Loop**
   - Create an instance of LinuxAgent.
   - Enter a loop that:
     a. Accepts a command from the user.
     b. Checks if the user wants to exit.
     c. Executes the command and saves the result if not exiting.
   - After the loop ends, display all results.


## Bonus Challenges
- Implement command history navigation (up/down arrows to cycle through previous commands).


## Testing
- Test your program with various Linux commands.
- Ensure it correctly handles commands with different outputs, including multi-line outputs.
- Test error scenarios (e.g., invalid commands, file I/O errors).


## Submission
- Push your Rust source code to github. Submit the link
- Provide a sample output file showing the results of a session with multiple commands executed. Take a screenshot.


