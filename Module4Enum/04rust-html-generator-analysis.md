
Example: HTML Generator

```rust
//HTML page generator. Enjoy coding.

enum ContentType {
    Heading(String),
    Paragraph(String),
    Link(String, String), // URL, Text
}

struct HtmlElement {
    content_type: ContentType,
}

impl HtmlElement {
    fn new(content_type: ContentType) -> Self {
        HtmlElement { content_type }
    }

    fn render(&self) -> String {
        match &self.content_type {
            ContentType::Heading(text) => format!("<h1>{}</h1>", text),
            ContentType::Paragraph(text) => format!("<p>{}</p>", text),
            ContentType::Link(href, text) => format!("<a href='{}'>{}</a>", href, text),
        }
    }
}

struct HtmlPage {
    title: String,
    elements: Vec<HtmlElement>,
}

impl HtmlPage {
    fn new(title: String) -> Self {
        HtmlPage {
            title,
            elements: Vec::new(),
        }
    }

    fn add_element(&mut self, element: HtmlElement) {
        self.elements.push(element);
    }

    fn generate(&self) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", self.title));
        html.push_str("</head>\n<body>\n");

        for element in &self.elements {
            html.push_str(&element.render());
            html.push('\n');
        }

        html.push_str("</body>\n</html>");
        html
    }
}


use std::fs::File;
use std::io::{self, Write, stdin, stdout};

fn read_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut page = HtmlPage::new(read_user_input("Enter the title of your page:"));
    
    loop {
        println!("\nWhat would you like to add to your page?");
        println!("1: Heading");
        println!("2: Paragraph");
        println!("3: Link");
        println!("4: Generate Page and Exit");

        let choice = read_user_input("Choose an option (1-4):");

        match choice.as_str() {
            "1" => {
                let heading = read_user_input("Enter heading text:");
                page.add_element(HtmlElement::new(ContentType::Heading(heading)));
            },
            "2" => {
                let paragraph = read_user_input("Enter paragraph text:");
                page.add_element(HtmlElement::new(ContentType::Paragraph(paragraph)));
            },
            "3" => {
                let text = read_user_input("Enter link text:");
                let url = read_user_input("Enter link URL:");
                page.add_element(HtmlElement::new(ContentType::Link(url, text)));
            },
            "4" => break,
            _ => println!("Invalid choice, please enter a number between 1 and 4."),
        }
    }

    let html = page.generate();
    let mut file = File::create("interactive_output.html").expect("Could not create file");
    writeln!(file, "{}", html).expect("Could not write to file");

    println!("HTML page generated successfully.");
}
```

# Analysis of Rust HTML Page Generator: Enums, Structs, and Best Practices

This document provides a comprehensive analysis of the Rust HTML page generator code, focusing on the use of enums, structs, and other Rust features.

## 1. Enum Usage: ContentType

```rust
enum ContentType {
    Heading(String),
    Paragraph(String),
    Link(String, String), // URL, Text
}
```

Key Points:
- This enum represents different types of HTML content.
- Each variant holds associated data:
  - `Heading` and `Paragraph` hold a single `String`.
  - `Link` holds two `String` values (URL and text).
- This design allows for type-safe representation of different HTML elements.

Best Practice: Use enums with associated data to represent variants of a concept that each have their own data requirements.

## 2. Struct and Implementation: HtmlElement

```rust
struct HtmlElement {
    content_type: ContentType,
}

impl HtmlElement {
    fn new(content_type: ContentType) -> Self {
        HtmlElement { content_type }
    }

    fn render(&self) -> String {
        match &self.content_type {
            ContentType::Heading(text) => format!("<h1>{}</h1>", text),
            ContentType::Paragraph(text) => format!("<p>{}</p>", text),
            ContentType::Link(href, text) => format!("<a href='{}'>{}</a>", href, text),
        }
    }
}
```

Key Points:
- `HtmlElement` struct wraps a `ContentType`.
- `new` method creates a new `HtmlElement` from a `ContentType`.
- `render` method uses pattern matching on `ContentType` to generate appropriate HTML.

Best Practices:
- Use structs to group related data (here, an HTML element's content).
- Implement methods on structs for associated functionality.
- Use pattern matching with enums to handle different cases concisely.

## 3. Struct and Implementation: HtmlPage

```rust
struct HtmlPage {
    title: String,
    elements: Vec<HtmlElement>,
}

impl HtmlPage {
    fn new(title: String) -> Self {
        HtmlPage {
            title,
            elements: Vec::new(),
        }
    }

    fn add_element(&mut self, element: HtmlElement) {
        self.elements.push(element);
    }

    fn generate(&self) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", self.title));
        html.push_str("</head>\n<body>\n");
        for element in &self.elements {
            html.push_str(&element.render());
            html.push('\n');
        }
        html.push_str("</body>\n</html>");
        html
    }
}
```

Key Points:
- `HtmlPage` struct represents a full HTML page with a title and a vector of `HtmlElement`s.
- `new` method initializes a new `HtmlPage` with a given title and an empty vector of elements.
- `add_element` method allows adding new elements to the page.
- `generate` method builds the complete HTML string.

Best Practices:
- Use `Vec` for collections of elements that can grow.
- Implement methods for initialization (`new`) and modification (`add_element`).
- Use string manipulation methods like `push_str` for efficient string building.

## 4. User Input Handling

```rust
fn read_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
```

Key Points:
- This function handles reading user input from the console.
- It uses `stdin()` to read a line of input.
- The input is trimmed to remove any trailing newline characters.

Best Practice: Separate input handling into its own function for reusability and cleaner code.

## 5. Main Function and Program Flow

```rust
fn main() {
    let mut page = HtmlPage::new(read_user_input("Enter the title of your page:"));
    
    loop {
        // ... (menu printing)
        let choice = read_user_input("Choose an option (1-4):");
        match choice.as_str() {
            "1" => {
                let heading = read_user_input("Enter heading text:");
                page.add_element(HtmlElement::new(ContentType::Heading(heading)));
            },
            "2" => {
                let paragraph = read_user_input("Enter paragraph text:");
                page.add_element(HtmlElement::new(ContentType::Paragraph(paragraph)));
            },
            "3" => {
                let text = read_user_input("Enter link text:");
                let url = read_user_input("Enter link URL:");
                page.add_element(HtmlElement::new(ContentType::Link(url, text)));
            },
            "4" => break,
            _ => println!("Invalid choice, please enter a number between 1 and 4."),
        }
    }

    let html = page.generate();
    let mut file = File::create("interactive_output.html").expect("Could not create file");
    writeln!(file, "{}", html).expect("Could not write to file");
    println!("HTML page generated successfully.");
}
```

Key Points:
- The main function orchestrates the entire program flow.
- It uses a loop to repeatedly prompt the user for input.
- Pattern matching is used to handle different user choices.
- The program creates `HtmlElement`s based on user input and adds them to the `HtmlPage`.
- Finally, it generates the HTML and writes it to a file.

Best Practices:
- Use `loop` for repeated actions with an unknown number of iterations.
- Utilize pattern matching for handling different cases in user input.
- Handle potential errors using `expect` or more advanced error handling techniques.

## Conclusion

This HTML generator demonstrates several Rust best practices and features:

1. Using enums with associated data to represent different types of content.
2. Implementing methods on structs to encapsulate behavior.
3. Using pattern matching to handle different enum variants.
4. Managing collections of elements with `Vec`.
5. Separating concerns (e.g., input handling, HTML generation) into different functions and methods.
6. Using Rust's ownership system to manage resources efficiently.

The program showcases how Rust's type system, particularly enums and pattern matching, can be used to create a type-safe and expressive domain model for generating HTML content. It also demonstrates good practices in structuring a Rust program, from handling user input to writing output to a file.
