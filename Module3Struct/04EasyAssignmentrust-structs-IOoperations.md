
## Struct Usage: I/O Operations

In this section, we'll explore more advanced uses of structs in Rust, particularly in conjunction with I/O operations. We'll cover reading input from the console and from a file, demonstrating how structs can be used to organize and manage data in these scenarios.

### Reading from Console
Here's an example of how to use a struct to store user input from the console:

```rust
use std::io::{self, Read, Write};

struct Person {
    name: String,
    age: u32,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("How old are you? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let age = buffer.trim().parse().unwrap();

    let person = Person { name, age };
    println!("Hi {}, you are {} years old!", person.name, person.age);
}
```

### Reading from File

Next, let's look at how to use a struct to represent configuration data read from a file:

```rust
use std::fs::File;
use std::io::prelude::*;

struct Config {
    username: String,
    api_key: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let username = lines.next().unwrap().to_string();
        let api_key = lines.next().unwrap().to_string();
        let port = lines.next().unwrap().parse().unwrap();

        Config { username, api_key, port }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("username: {}", config.username);
    println!("api key: {}", config.api_key);
    println!("port: {}", config.port);
}
```

To use these functions, you would call them in your `main` function:

```rust
fn main() {
    reading_from_console();
    reading_from_file();
}
```

Note: For the `reading_from_file` function to work, you need to create a `config.txt` file in the same directory as your Rust program, with the following content:

```
YourUsername
YourAPIKey
8080
```

Replace the values with your actual configuration data.

## Conclusion

Structs in Rust provide a powerful way to organize and encapsulate related data and behavior. By using structs and implementing methods, you can create clean, modular, and efficient code that closely models the problem domain you're working with.
