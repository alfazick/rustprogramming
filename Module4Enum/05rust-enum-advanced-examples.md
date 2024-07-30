# Advanced Enum Examples in Rust: Practical Use Cases

This guide provides in-depth examples of enum usage in Rust for specific use cases, demonstrating practical applications and best practices.

## 1. Status Codes

This example shows how to use enums to represent HTTP status codes and handle responses accordingly.

```rust
#[derive(Debug,Clone,Copy)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
}

struct HttpResponse {
    status: HttpStatus,
    body: String,
}

impl HttpResponse {
    fn new(status: HttpStatus, body: String) -> Self {
        HttpResponse { status, body }
    }

    fn send(&self) {
        println!("Status: {:?} ({})", self.status, self.status as i32);
        println!("Body: {}", self.body);
        match self.status {
            HttpStatus::Ok => println!("Request successful"),
            HttpStatus::NotFound => println!("Resource not found"),
            HttpStatus::InternalServerError => println!("Server error occurred"),
            HttpStatus::BadRequest => println!("Invalid request"),
            HttpStatus::Unauthorized => println!("Authentication required"),
            HttpStatus::Forbidden => println!("Access denied"),
        }
    }
}

fn main() {
    let responses = vec![
        HttpResponse::new(HttpStatus::Ok, "Hello, World!".to_string()),
        HttpResponse::new(HttpStatus::NotFound, "Page not found".to_string()),
        HttpResponse::new(HttpStatus::InternalServerError, "Server error".to_string()),
    ];

    for response in responses {
        response.send();
        println!("---");
    }
}
```

## 2. State Machines

This example demonstrates a more complex state machine for a vending machine using enums.

```rust
#[derive(Debug)]
enum VendingMachineState {
    Idle,
    ItemSelected { item: String },
    PaymentPending { item: String, amount: f64 },
    Dispensing { item: String },
    Refunding { amount: f64 },
}

struct VendingMachine {
    state: VendingMachineState,
    balance: f64,
}

impl VendingMachine {
    fn new() -> Self {
        VendingMachine {
            state: VendingMachineState::Idle,
            balance: 0.0,
        }
    }

    fn select_item(&mut self, item: String) {
        match self.state {
            VendingMachineState::Idle => {
                println!("Item selected: {}", item);
                self.state = VendingMachineState::ItemSelected { item };
            }
            _ => println!("Please wait, operation in progress"),
        }
    }

    fn insert_money(&mut self, amount: f64) {
        match &self.state {
            VendingMachineState::ItemSelected { item } => {
                self.balance += amount;
                println!("Money inserted: ${:.2}", amount);
                self.state = VendingMachineState::PaymentPending {
                    item: item.clone(),
                    amount: self.balance,
                };
            }
            VendingMachineState::PaymentPending { item, amount } => {
                self.balance += amount;
                println!("Additional money inserted: ${:.2}", amount);
                self.state = VendingMachineState::PaymentPending {
                    item: item.clone(),
                    amount: self.balance,
                };
            }
            _ => println!("Please select an item first"),
        }
    }

    fn dispense(&mut self) {
        match &self.state {
            VendingMachineState::PaymentPending { item, amount } => {
                if *amount >= 1.0 {
                    println!("Dispensing: {}", item);
                    self.state = VendingMachineState::Dispensing { item: item.clone() };
                    self.balance -= 1.0;
                } else {
                    println!("Insufficient funds");
                }
            }
            _ => println!("No item selected or payment not made"),
        }
    }

    fn refund(&mut self) {
        if self.balance > 0.0 {
            println!("Refunding ${:.2}", self.balance);
            self.state = VendingMachineState::Refunding { amount: self.balance };
            self.balance = 0.0;
        } else {
            println!("No money to refund");
        }
    }

    fn complete_transaction(&mut self) {
        self.state = VendingMachineState::Idle;
        println!("Transaction completed. Machine is now idle.");
    }
}

fn main() {
    let mut machine = VendingMachine::new();
    machine.select_item("Soda".to_string());
    machine.insert_money(0.75);
    machine.insert_money(0.50);
    machine.dispense();
    machine.refund();
    machine.complete_transaction();
}
```

## 3. Configuration Options

This example shows how to use enums for application configuration, including serialization and deserialization.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Theme {
    Light,
    Dark,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    theme: Theme,
    log_level: LogLevel,
    max_connections: u32,
}

impl AppConfig {
    fn new() -> Self {
        AppConfig {
            theme: Theme::Light,
            log_level: LogLevel::Info,
            max_connections: 100,
        }
    }

    fn apply(&self) {
        println!("Applying configuration:");
        match self.theme {
            Theme::Light => println!("  Theme: Light mode"),
            Theme::Dark => println!("  Theme: Dark mode"),
            Theme::Custom(ref color) => println!("  Theme: Custom color - {}", color),
        }
        println!("  Log Level: {:?}", self.log_level);
        println!("  Max Connections: {}", self.max_connections);
    }
}

fn main() {
    let config = AppConfig::new();
    config.apply();

    // Serialize to JSON
    let serialized = serde_json::to_string_pretty(&config).unwrap();
    println!("\nSerialized Config:\n{}", serialized);

    // Deserialize from JSON
    let deserialized: AppConfig = serde_json::from_str(&serialized).unwrap();
    println!("\nDeserialized Config:");
    deserialized.apply();
}
```

## 4. Command Types

This example demonstrates using enums to represent different command types in a command-line application.

```rust
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Add { name: String, phone: String },
    Remove { name: String },
    List,
    Quit,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts.as_slice() {
            ["add", name, phone] => Ok(Command::Add { 
                name: name.to_string(), 
                phone: phone.to_string() 
            }),
            ["remove", name] => Ok(Command::Remove { 
                name: name.to_string() 
            }),
            ["list"] => Ok(Command::List),
            ["quit"] => Ok(Command::Quit),
            _ => Err("Invalid command".to_string()),
        }
    }
}

fn execute_command(command: &Command) {
    match command {
        Command::Add { name, phone } => 
            println!("Adding {} with phone number {}", name, phone),
        Command::Remove { name } => 
            println!("Removing {}", name),
        Command::List => 
            println!("Listing all entries"),
        Command::Quit => 
            println!("Quitting the application"),
    }
}

fn main() {
    let commands = vec![
        "add John 123-456-7890",
        "list",
        "remove John",
        "invalid command",
        "quit",
    ];

    for cmd_str in commands {
        match Command::from_str(cmd_str) {
            Ok(command) => execute_command(&command),
            Err(e) => println!("Error: {}", e),
        }
    }
}
```

## 5. Categorizing Data

This example shows how to use enums for categorizing products in an e-commerce system.

```rust
#[derive(Debug)]
enum Category {
    Electronics(ElectronicsType),
    Clothing(ClothingSize, String), // Size and Material
    Books(String), // Genre
    Food(bool), // Perishable
}

#[derive(Debug)]
enum ElectronicsType {
    Smartphone,
    Laptop,
    Television,
}

#[derive(Debug)]
enum ClothingSize {
    Small,
    Medium,
    Large,
}

struct Product {
    name: String,
    price: f64,
    category: Category,
}

impl Product {
    fn new(name: String, price: f64, category: Category) -> Self {
        Product { name, price, category }
    }

    fn display(&self) {
        println!("Product: {}", self.name);
        println!("Price: ${:.2}", self.price);
        println!("Category: {:?}", self.category);
        match &self.category {
            Category::Electronics(etype) => println!("Type: {:?}", etype),
            Category::Clothing(size, material) => println!("Size: {:?}, Material: {}", size, material),
            Category::Books(genre) => println!("Genre: {}", genre),
            Category::Food(perishable) => println!("Perishable: {}", perishable),
        }
        println!("---");
    }
}

fn main() {
    let products = vec![
        Product::new("Smartphone".to_string(), 599.99, Category::Electronics(ElectronicsType::Smartphone)),
        Product::new("T-Shirt".to_string(), 19.99, Category::Clothing(ClothingSize::Medium, "Cotton".to_string())),
        Product::new("Mystery Novel".to_string(), 9.99, Category::Books("Mystery".to_string())),
        Product::new("Fresh Apples".to_string(), 3.99, Category::Food(true)),
    ];

    for product in products {
        product.display();
    }
}
```

## 6. Flags and Options

This example demonstrates using enums as flags for file permissions, showing how they can be combined and checked.

```rust
use std::ops::{BitOr, BitAnd};

#[derive(Debug, Clone, Copy)]
enum FilePermission {
    Read = 0b100,
    Write = 0b010,
    Execute = 0b001,
}

impl BitOr for FilePermission {
    type Output = FilePermissions;

    fn bitor(self, rhs: Self) -> FilePermissions {
        FilePermissions { bits: self as u8 | rhs as u8 }
    }
}

#[derive(Debug, Clone, Copy)]
struct FilePermissions {
    bits: u8,
}

impl FilePermissions {
    fn new() -> Self {
        FilePermissions { bits: 0 }
    }

    fn add(&mut self, perm: FilePermission) {
        self.bits |= perm as u8;
    }

    fn remove(&mut self, perm: FilePermission) {
        self.bits &= !(perm as u8);
    }

    fn has(&self, perm: FilePermission) -> bool {
        self.bits & (perm as u8) != 0
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push(if self.has(FilePermission::Read) { 'r' } else { '-' });
        result.push(if self.has(FilePermission::Write) { 'w' } else { '-' });
        result.push(if self.has(FilePermission::Execute) { 'x' } else { '-' });
        result
    }
}

impl BitOr<FilePermission> for FilePermissions {
    type Output = FilePermissions;

    fn bitor(self, rhs: FilePermission) -> FilePermissions {
        FilePermissions { bits: self.bits | rhs as u8 }
    }
}

impl BitAnd<FilePermission> for FilePermissions {
    type Output = bool;

    fn bitand(self, rhs: FilePermission) -> bool {
        self.has(rhs)
    }
}

fn main() {
    let mut perms = FilePermissions::new();
    perms.add(FilePermission::Read);
    perms.add(FilePermission::Write);

    println!("Initial permissions: {}", perms.to_string());

    perms = perms | FilePermission::Execute;
    println!("After adding execute: {}", perms.to_string());

    perms.remove(FilePermission::Write);
    println!("After removing write: {}", perms.to_string());

    println!("Has read permission: {}", perms & FilePermission::Read);
    println!("Has write permission: {}", perms & FilePermission::Write);
}
```

## 7. Event Types

This example shows how to use enums to represent different types of events in an event-driven system.

```rust
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
enum ApiEvent {
    UserRegistered { user_id: u64, email: String },
    UserLoggedIn { user_id: u64 },
    ProductViewed { user_id: u64, product_id: u64 },
    ProductAddedToCart { user_id: u64, product_id: u64, quantity: u32 },
    OrderPlaced { user_id: u64, order_id: u64, total_amount: f64 },
    PaymentProcessed { order_id: u64, payment_method: PaymentMethod, amount: f64 },
    Error { code: u32, message: String },
}

#[derive(Debug, Serialize, Deserialize)]
enum PaymentMethod {
    CreditCard,
    PayPal,
    BankTransfer,
}

struct EventLog {
    timestamp: DateTime<Utc>,
    event: ApiEvent,
}

impl EventLog {
    fn new(event: ApiEvent) -> Self {
        EventLog {
            timestamp: Utc::now(),
            event,
        }
    }

    fn log(&self) {
        let event_json = serde_json::to_string_pretty(&self.event).unwrap();
        println!("Timestamp: {}", self.timestamp);
        println!("Event: {}", event_json);
        println!("---");
    }
}

struct ApiEventHandler;

impl ApiEventHandler {
    fn handle_event(&self, event: ApiEvent) {
        let log = EventLog::new(event);
        log.log();

        match log.event {
            ApiEvent::UserRegistered { user_id, email } => {
                println!("New user registered: ID {} with email {}", user_id, email);
                // Here you might send a welcome email, initialize user's data, etc.
            }
            ApiEvent::UserLoggedIn { user_id } => {
                println!("User {} logged in", user_id);
                // You might update last login timestamp, initialize session, etc.
            }
            ApiEvent::ProductViewed { user_id, product_id } => {
                println!("User {} viewed product {}", user_id, product_id);
                // You might update product view counts, user's recently viewed items, etc.
            }
            ApiEvent::ProductAddedToCart { user_id, product_id, quantity } => {
                println!("User {} added {} units of product {} to cart", user_id, quantity, product_id);
                // Update user's cart in the database
            }
            ApiEvent::OrderPlaced { user_id, order_id, total_amount } => {
                println!("User {} placed order {} with total amount ${:.2}", user_id, order_id, total_amount);
                // Process the order, update inventory, etc.
            }
            ApiEvent::PaymentProcessed { order_id, payment_method, amount } => {
                println!("Payment processed for order {}: ${:.2} via {:?}", order_id, amount, payment_method);
                // Update order status, trigger shipment process, etc.
            }
            ApiEvent::Error { code, message } => {
                println!("Error occurred: Code {} - {}", code, message);
                // Log error, possibly trigger alerts for critical errors
            }
        }
    }
}

fn main() {
    let handler = ApiEventHandler;

    let events = vec![
        ApiEvent::UserRegistered { user_id: 1001, email: "john@example.com".to_string() },
        ApiEvent::UserLoggedIn { user_id: 1001 },
        ApiEvent::ProductViewed { user_id: 1001, product_id: 5001 },
        ApiEvent::ProductAddedToCart { user_id: 1001, product_id: 5001, quantity: 2 },
        ApiEvent::OrderPlaced { user_id: 1001, order_id: 10001, total_amount: 129.99 },
        ApiEvent::PaymentProcessed { order_id: 10001, payment_method: PaymentMethod::CreditCard, amount: 129.99 },
        ApiEvent::Error { code: 404, message: "Product not found".to_string() },
    ];

    for event in events {
        handler.handle_event(event);
    }
}

```