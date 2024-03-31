// Case Study: Secure coding: Zeroise sensitive data

1. Part: Intro to problem

fn main() {
     let data_address: *const u8;
     // Scope start
     {
        let sensitive_data = "password".to_string();
        data_address = sensitive_data.as_ptr();
        println!("Sensitive data in scope: {}", sensitive_data);
    }
    // Scope end

     // Simulate some operations to keep the program running
     println!("Out of scope");

     // Placeholder to prevent program from exiting immediately;
    
     std::thread::sleep(std::time::Duration::from_secs(1));
 }


// 1. cargo build
// navigate to debug version 
// 2. cd target/debug
// Starting GDB
// Open GDB with the compiled program:
// gdb ./main

// 4. Setting Breakpoints
// Set breakpoints at points of interest. 
// For example, immediately after the sensitive data is declared, 
// and after it goes out of scope:

// gdb
// break main.rs:6
// break main.rs:11
// Adjust the line numbers based on your actual code.

// 5. Running the Program in GDB
// Start the program within GDB:

// gdb
// run

// 6. Inspecting Variables and Memory
// When the first breakpoint hits, inspect the variables:

// gdb
// print &sensitive_data
// This command prints the address of sensitive_data. To see the value:

// gdb

// print sensitive_data
// Continue to the next line:

// gdb
// next
// 7. Observing Data Persistence Beyond Scope
// Once the variable goes out of scope, try to print the sensitive_data again:

// gdb

// print sensitive_data
// GDB should indicate that the variable is out of the current context. However, you can inspect the memory at the address where sensitive_data was stored:

// gdb

// x/s data_address
// This command attempts to display the string located at the memory address data_address was pointing to.

// 8. Concluding the GDB Session
// Finally, to exit GDB, use the command:

// gdb
// quit

// 2. So what is the problem ?
struct SensitiveData {
    data: String,
}

impl SensitiveData {
    fn new(length: usize) -> Self {
        let data = (b'a'..=b'z')
            .cycle()
            .map(|c| c as char)
            .take(length)
            .collect::<String>();
        SensitiveData { data }
    }


    fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

impl Drop for SensitiveData {
    fn drop(&mut self) {
        println!("SensitiveData is about to drop,but I don't promise zeroing data.");
    }
}


fn main() {
    
    let data_address: *const u8;
    // Scope start
    {
        let sensitive_data = SensitiveData::new(1000); // Creating a longer password string.
        data_address = sensitive_data.as_ptr();
        println!("Sensitive data in scope. {:?}",sensitive_data.data);
    }
    // Scope end

    println!("Sensitive data is now out of scope.");

    unsafe {
        let content_after = std::slice::from_raw_parts(data_address, 50); // Inspect the first 50 bytes.
        println!("Memory content after SensitiveData goes out of scope: {:?}", content_after);
        let content_string = String::from_utf8_lossy(content_after);
        println!("Guess : {}",content_string);
        }
    // Placeholder to prevent the program from exiting immediately
    std::thread::sleep(std::time::Duration::from_secs(1));
}

Another example to highlight the problem, even if you generate password

// [dependencies]
// rand = "0.8.5"


struct SensitiveData {
    data: String,
}
use rand::{distributions::Alphanumeric, Rng};

impl SensitiveData {
    fn new(length: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        // Generate a random password that may include ASCII alphanumeric characters and some symbols.
        let data: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(length)
            .collect();

        SensitiveData { data }
    }


    fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

impl Drop for SensitiveData {
    fn drop(&mut self) {
        println!("SensitiveData is about to drop,but I don't promise zeroing data.");
    }
}


fn main() {
    
    let data_address: *const u8;
    // Scope start
    {
        let sensitive_data = SensitiveData::new(1000); // Creating a longer password string.
        data_address = sensitive_data.as_ptr();
        println!("Sensitive data in scope. {:?}",sensitive_data.data);
    }
    // Scope end

    println!("Sensitive data is now out of scope.");

    unsafe {
        let content_after = std::slice::from_raw_parts(data_address, 50); // Inspect the first 50 bytes.
        println!("Memory content after SensitiveData goes out of scope: {:?}", content_after);
        let content_string = String::from_utf8_lossy(content_after);
        println!("Guess : {}",content_string);
        }
    // Placeholder to prevent the program from exiting immediately
    std::thread::sleep(std::time::Duration::from_secs(1));
}



3. Part: Solution Zeroize everything, especially sensitive data

// [dependencies]
// rand = "0.8.5"
// zeroize = "1.5"

struct SensitiveData {
    data: String,
}
use rand::{distributions::Alphanumeric, Rng};

impl SensitiveData {
    fn new(length: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        // Generate a random password that may include ASCII alphanumeric characters and some symbols.
        let data: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(length)
            .collect();

        SensitiveData { data }
    }


    fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
}
use zeroize::Zeroize;
impl Drop for SensitiveData {
    fn drop(&mut self) {
        // Convert the String into a Vec<u8>, zeroizing its buffer
        let mut data_bytes = std::mem::take(&mut self.data).into_bytes();
        data_bytes.zeroize();
        println!("SensitiveData has been securely zeroized.");
    }
}

fn main() {
    
    let data_address: *const u8;
    // Scope start
    {
        let sensitive_data = SensitiveData::new(1000); // Creating a longer password string.
        data_address = sensitive_data.as_ptr();
        println!("Sensitive data in scope. {:?}",sensitive_data.data);
    }
    // Scope end

    println!("Sensitive data is now out of scope.");

    unsafe {
        let content_after = std::slice::from_raw_parts(data_address, 50); // Inspect the first 50 bytes.
        println!("Memory content after SensitiveData goes out of scope: {:?}", content_after);
        let content_string = String::from_utf8_lossy(content_after);
        println!("Guess : {}",content_string);
        }
    // Placeholder to prevent the program from exiting immediately
    std::thread::sleep(std::time::Duration::from_secs(1));
}

