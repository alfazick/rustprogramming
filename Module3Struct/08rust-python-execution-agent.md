# Rust Agent for Python Script Execution

Here's a more advanced Rust agent capable of executing Python scripts with various options:

```rust
use std::process::Command;
use std::env;

struct PythonAgent {
    python_path: String,
    script_dir: String,
}

impl PythonAgent {
    fn new() -> Self {
        PythonAgent {
            python_path: String::from("python"), // Default to system Python
            script_dir: env::current_dir().unwrap().to_str().unwrap().to_string(),
        }
    }

    fn set_python_path(&mut self, path: &str) {
        self.python_path = path.to_string();
    }

    fn set_script_dir(&mut self, dir: &str) {
        self.script_dir = dir.to_string();
    }

    fn execute_script(&self, script_name: &str, args: &[&str]) -> Result<String, String> {
        let script_path = format!("{}/{}", self.script_dir, script_name);
        let output = Command::new(&self.python_path)
            .arg(&script_path)
            .args(args)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute Python script: {}", e)),
        }
    }
}

fn main() {
    let mut agent = PythonAgent::new();
    
    // Optionally set custom Python path
    // agent.set_python_path("/usr/bin/python3");
    
    // Optionally set custom script directory
    // agent.set_script_dir("/path/to/scripts");

    match agent.execute_script("generate_dashboard.py", &[]) {
        Ok(output) => println!("Script output: {}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

This agent provides more flexibility:
1. It allows setting a custom Python interpreter path.
2. It allows specifying a directory for Python scripts.
3. It can pass arguments to the Python script.
4. It handles both successful executions and errors, returning the output or error message.

To use this agent, you would create an instance, optionally configure it, and then execute Python scripts as needed. This provides a more robust way to integrate Python scripts into your Rust applications.
