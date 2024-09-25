# Rust Inline Assembly for Direct System Calls

This document explains a Rust program that uses inline assembly to make direct system calls for writing to stdout and exiting the program.

## The Complete Code

```rust
use std::arch::asm;

fn main() {
    let message = b"Hello, direct syscall!\n";

    unsafe {
        // write syscall
        asm!(
            "mov rax, 1",  // syscall number for write
            "mov rdi, 1",  // file descriptor: 1 is stdout
            "syscall",
            in("rsi") message.as_ptr(),
            in("rdx") message.len(),
            out("rax") _,
            out("rcx") _,
            out("r11") _,
            clobber_abi("system")
        );

        // exit syscall
        asm!(
            "mov rax, 60", // syscall number for exit
            "xor rdi, rdi", // status code 0
            "syscall",
            options(noreturn)
        );
    }
}
```

## Detailed Explanation

### Imports and Setup

```rust
use std::arch::asm;
```
This line imports the `asm` macro, which allows us to write inline assembly in Rust.

### The `main` Function

```rust
fn main() {
    let message = b"Hello, direct syscall!\n";
```
We define our message as a byte string (`&[u8]`), which is required for the `write` system call.

### The `write` System Call

```rust
unsafe {
    asm!(
        "mov rax, 1",  // syscall number for write
        "mov rdi, 1",  // file descriptor: 1 is stdout
        "syscall",
        in("rsi") message.as_ptr(),
        in("rdx") message.len(),
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        clobber_abi("system")
    );
```

1. `unsafe`: This block is required because inline assembly is inherently unsafe.
2. `asm!`: This macro allows us to write inline x86-64 assembly.
3. `"mov rax, 1"`: Sets the system call number for `write` (1 on x86-64 Linux).
4. `"mov rdi, 1"`: Sets the first argument (file descriptor) to 1 (stdout).
5. `"syscall"`: Triggers the system call.
6. `in("rsi") message.as_ptr()`: Passes the pointer to our message as the second argument.
7. `in("rdx") message.len()`: Passes the length of our message as the third argument.
8. `out("rax") _`, `out("rcx") _`, `out("r11") _`: Indicate that these registers are clobbered (modified) by the system call.
9. `clobber_abi("system")`: Informs the compiler that this assembly follows the system ABI rules.

### The `exit` System Call

```rust
    asm!(
        "mov rax, 60", // syscall number for exit
        "xor rdi, rdi", // status code 0
        "syscall",
        options(noreturn)
    );
```

1. `"mov rax, 60"`: Sets the system call number for `exit` (60 on x86-64 Linux).
2. `"xor rdi, rdi"`: Sets the exit status to 0 (success).
3. `"syscall"`: Triggers the system call.
4. `options(noreturn)`: Tells the compiler that this system call doesn't return.

## Execution Order in the `asm!` Macro

An important aspect to understand about the `asm!` macro is that the order of elements in the macro doesn't directly correspond to the order of execution. Here's how it actually works:

1. **Assembly Instructions**: 
   The assembly instructions (provided as string literals) are executed in the order they appear.

2. **Input/Output Constraints**:
   The `in` and `out` constraints are not part of the assembly code itself. They instruct the Rust compiler on how to set up registers before the assembly code runs and how to handle them after.

3. **Actual Execution Order**:
   1. Rust sets up the registers as specified by the `in` constraints.
   2. The assembly code runs (including the `syscall` instruction).
   3. Rust handles the `out` constraints (if any).

For example, in our `write` syscall, even though `syscall` appears before the `in` constraints, the effective order is:

1. `rsi` is set to `message.as_ptr()`
2. `rdx` is set to `message.len()`
3. The assembly instructions run (`mov rax, 1`, `mov rdi, 1`, `syscall`)
4. The `out` constraints are handled (marking `rax`, `rcx`, and `r11` as clobbered)

This separation between assembly instructions and constraints allows for better integration with Rust's compilation process, making inline assembly both powerful and safer to use.

## Key Concepts

1. **System Calls**: Direct requests to the operating system kernel for services like I/O operations or process termination.
2. **Inline Assembly**: Allows embedding assembly code directly within Rust code for low-level operations.
3. **Registers**: Special storage locations in the CPU. In x86-64 Linux:
   - `rax`: Holds the system call number and return value
   - `rdi`, `rsi`, `rdx`: Used for passing arguments to system calls
   - `rcx`, `r11`: Clobbered by the `syscall` instruction
4. **Clobbered Registers**: Registers whose values are modified by an operation (like a system call).
5. **ABI (Application Binary Interface)**: A set of rules governing how functions are called and how data is organized in memory.

## Safety Considerations

- This code uses `unsafe` Rust, bypassing many of Rust's safety guarantees.
- Direct system calls can be dangerous if used incorrectly, potentially causing system instability.
- This approach skips Rust's standard library, losing its error handling and cross-platform abstractions.
- It should be used judiciously, typically only for performance-critical code or when interfacing with the OS in ways not supported by the standard library.

## Conclusion

This code demonstrates how to perform direct system calls using Rust's inline assembly feature. It provides a low-level interface to the operating system, allowing for precise control over system interactions. However, it requires careful handling and a deep understanding of both Rust and system-level programming to use safely and effectively.
