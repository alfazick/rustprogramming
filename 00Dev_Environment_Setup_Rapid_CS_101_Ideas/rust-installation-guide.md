# Installing Rust and Compiling in Debug/Release Mode (GitHub Codespace)

This guide covers installing Rust in a GitHub Codespace environment and comparing debug/release builds.

## Step 1: Setup in GitHub Codespace

GitHub Codespaces typically come with Rust pre-installed. Verify the installation:

```bash
rustc --version
cargo --version
```

If Rust is not installed, install it using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

## Step 2: Create a New Rust Project

1. Create a new binary project:
```bash
cargo new my_project
cd my_project
```

2. Open `src/main.rs` and add this code:
```rust
// Instructions:
// 1. Replace STUDENTNAME with your actual name
// 2. Compile and run the program
// 3. Compare debug vs release builds
// 4. Use stat command to analyze the binaries

fn main() {
    println!("Hello STUDENTNAME!");
}
```

## Step 3: Compile and Run Debug Build

Debug mode is the default:

```bash
# Compile and run in debug mode
cargo build
./target/debug/my_project

# Check binary size and info
stat target/debug/my_project
```

## Step 4: Compile and Run Release Build

```bash
# Compile and run in release mode
cargo build --release
./target/release/my_project

# Check binary size and info
stat target/release/my_project
```

## Common Cargo Commands:

```bash
cargo new project_name    # Create new project
cargo check              # Check if code compiles
cargo build              # Build in debug mode
cargo build --release    # Build in release mode
cargo run               # Build and run in debug mode
cargo run --release     # Build and run in release mode
```