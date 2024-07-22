# Executing OS Commands in Rust

```rust
use std::process::Command;

fn executing_os_commands_linux() {
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}
```

This example demonstrates how to execute a system command (in this case, the `ls -l` command on a Unix-like system) and capture its output.
