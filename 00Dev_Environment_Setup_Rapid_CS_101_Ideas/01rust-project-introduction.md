# Introduction to Rust Projects with Cargo

## What is Cargo?
- Cargo is Rust's package manager and build tool
- Handles tasks like dependency management, testing, and package management
- Streamlines creating, building, testing, and distributing Rust applications and libraries

## Creating a New Rust Project
- Command: `cargo new <project>`
- Creates a folder with the project name and the following structure:
  ```
  <project>
  ├── src
      ├── main.rs
  ├── Cargo.toml
  ```

### Key Files
1. `src/main.rs`: 
   - Entry point of the program
   - Contains the `main()` function

2. `Cargo.toml`:
   - Specifies project information
   - Lists dependencies and their versions

## Running the Project
1. Build the project: `cargo build`
2. Run the project: `cargo run`

Note: `cargo run` can be used directly as it includes the build step

## Building the Project
Two build modes:

1. Debug Build (Default):
   - Minimal optimization
   - Faster build time
   - Easier debugging

2. Release Build:
   - Command: `cargo build --release`
   - Aggressive compiler optimizations
   - Generates highly efficient machine code
   - Longer compilation time

## Useful Cargo Commands

| Command | Purpose |
|---------|---------|
| `cargo new <project>` | Creates a new project |
| `cargo build` | Builds the project |
| `cargo run` | Runs the project |
| `cargo test <test_name>` | Runs a specific test |
| `cargo docs` | Creates project documentation |
| `cargo fmt` | Formats all project code |
| `cargo update` | Updates dependencies in Cargo.toml |

