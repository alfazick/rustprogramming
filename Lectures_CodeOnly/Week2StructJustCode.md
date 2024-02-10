

// PLAN

// 1) Demonstrating How Structs Work: A Funny Example

fn simple_struct(){

    struct Cat {
        name: String,
        age: u8,
    }

    let my_cat = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };

    println!("My cat's name is {} and it is {} years old.", my_cat.name, my_cat.age);

}

// 2. Explaining Impl Block, &self, and &mut self
// Now, let's add methods to our Cat struct to see how the impl block works, 
// including methods with &self and &mut self.

fn struct_impl_reference(){

    struct Cat {
        name: String,
        age: u8,
    }
    
    impl Cat {
        // A method that borrows self immutably
        fn introduce(&self) {
            println!("Meow! My name is {} and I am {} years old.", self.name, self.age);
        }
    
        // A method that borrows self mutably to update age
        fn have_birthday(&mut self) {
            self.age += 1;
            println!("{} is now {} years old. Happy Birthday!", self.name, self.age);
        }
    }
    
    
    let mut my_cat = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };

    my_cat.introduce();
    my_cat.have_birthday();
    

}


fn creating_and_writing_to_file(){
    use std::fs::File;
    use std::io::Write;

    
    let mut file = File::create("hello.txt").expect("Could not create file");
    writeln!(file, "Hello, file!").expect("Could not write to file");

}

fn executing_os_commands_linux(){

    use std::process::Command;

    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));

}


fn main() {
    simple_struct();
    struct_impl_reference();

    creating_and_writing_to_file();
    executing_os_commands_linux();
    
}

//ALL TOGETHER :) Enjoy coding

Integration Explanation:

Rust Program: Processes logs and demonstrates the benefits of using structs for organized and maintainable code. It creates a summary file and then executes the Python script for visualization.

Python Script: Reads the summary file and generates a matplotlib bar chart. This script should be saved as generate_dashboard.py in the same directory as the Rust executable.

Execution: The Rust program executes the Python script with Command::new("python").arg("generate_dashboard.py").output(). Ensure Python and matplotlib are installed and accessible in your environment.

This approach clearly separates the log processing and visualization tasks, showcasing how Rust can be used for efficient data processing and Python for flexible data visualization.

Instructions

Create a new generate_dashboard.py on same level as Cargo.toml


'''
import matplotlib.pyplot as plt
def generate_dashboard(summary_file_path='log_summary.txt'):
    counts = {'INFO': 0, 'WARN': 0, 'ERROR': 0}
    with open(summary_file_path, 'r') as file:
        for line in file:
            level, count = line.strip().split(": ")
            counts[level] = int(count)

    levels = list(counts.keys())
    occurrences = list(counts.values())

    plt.figure(figsize=(10, 6))
    plt.bar(levels, occurrences, color=['blue', 'orange', 'red'])
    plt.title('Log Level Occurrences')
    plt.xlabel('Log Level')
    plt.ylabel('Occurrences')
    plt.xticks(levels)
    plt.savefig('dashboard.png')
    plt.show()

if __name__ == "__main__":
    generate_dashboard()


'''





Final Rust main.rs

'''

use std::fs::File;
use std::io::Write;
use std::process::Command;

struct LogSummary {
    info_count: u32,
    warn_count: u32,
    error_count: u32,
}

impl LogSummary {
    fn new() -> LogSummary {
        LogSummary {
            info_count: 0,
            warn_count: 0,
            error_count: 0,
        }
    }

    fn process_log(&mut self, log: &str) {
        if log.contains("INFO") {
            self.info_count += 1;
        } else if log.contains("WARN") {
            self.warn_count += 1;
        } else if log.contains("ERROR") {
            self.error_count += 1;
        }
    }

    fn save_to_file(&self) {
        let mut file = File::create("log_summary.txt").unwrap();
        writeln!(file, "INFO: {}", self.info_count).unwrap();
        writeln!(file, "WARN: {}", self.warn_count).unwrap();
        writeln!(file, "ERROR: {}", self.error_count).unwrap();
    }

    fn execute_python_script(&self) {
        Command::new("python")
            .arg("generate_dashboard.py") // Adjust the path based on your project structure
            .status() // Changed to `status` to avoid capturing output, simplifying further
            .unwrap();
    }
}

fn main() {
    let logs = [
        "INFO: Operation successful",
        "ERROR: Failed to connect",
        "WARN: Low battery",
        "INFO: Data synced",
        "ERROR: Timeout occurred",
    ];

    let mut summary = LogSummary::new();
    for log in logs.iter() {
        summary.process_log(log);
    }
    summary.save_to_file();
    summary.execute_python_script();
}

'''
