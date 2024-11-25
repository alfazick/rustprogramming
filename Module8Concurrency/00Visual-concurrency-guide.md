# Introduction to Concurrency

```mermaid
graph TD
    subgraph "Single Process, Single Thread"
        A[Process] --> B[Code Section]
        A --> C[Heap Section]
        A --> D[Single Stack]
    end

    subgraph "Single Process, Multiple Threads"
        E[Process] --> F[Shared Code Section]
        E --> G[Shared Heap Section]
        E --> H[Stack - Thread 1]
        E --> I[Stack - Thread 2]
        E --> J[Stack - Thread 3]
    end

    subgraph "Thread Components"
        K[Thread] --> L[Program Counter]
        K --> M[Private Registers]
        K --> N[Thread Stack]
    end

    subgraph "Critical Section Problem"
        O[Shared Resource] --> |Thread 1| P[Critical Section]
        O --> |Thread 2| P
        P --> Q{Race Condition}
        Q --> R[Mutex Lock]
        R --> S[Safe Access]
    end
```

## Real-World Analogy
Imagine a scenario with 3 banknotes and 3 people:
- When people don't communicate and randomly try to pick up notes
- This can lead to conflicts (like race conditions in programming)
- This simple example perfectly illustrates concurrent programming challenges

## Understanding Threads
A thread is a fundamental unit of CPU execution that represents an independent path of execution within a process.

### Key Characteristics:
1. **Shared Resources**
   - All threads within a process share:
     - Code section
     - Heap memory
     - Global variables
   - Each thread has its own:
     - Program counter
     - Register set
     - Stack

2. **Thread vs Process**
   - Threads are "lightweight" compared to processes
   - Thread creation is faster than process creation
   - Context switching between threads is typically faster

## Process and Thread Architecture

```mermaid
graph TB
    subgraph "Memory Layout"
        direction TB
        A[Process Memory Space] --> B[Code]
        A --> C[Data]
        A --> D[Heap]
        A --> E1[Stack - Thread 1]
        A --> E2[Stack - Thread 2]
        A --> E3[Stack - Thread 3]
        
        style A fill:#f9f,stroke:#333,stroke-width:4px
        style B fill:#bbf,stroke:#333
        style C fill:#bbf,stroke:#333
        style D fill:#bbf,stroke:#333
        style E1 fill:#bfb,stroke:#333
        style E2 fill:#bfb,stroke:#333
        style E3 fill:#bfb,stroke:#333
    end
```

## Why Use Threads?

### 1. Parallelism
Example: Processing 10,000 arrays
- **Single-threaded**: Process one array at a time
- **Multi-threaded**: Divide work among multiple threads
  - 10 threads → 1,000 arrays each
  - Significant performance improvement

### 2. Improved Interactivity
- Prevents blocking on I/O operations
- Better CPU utilization
- Enhanced responsiveness

## Thread Operations in Rust

### Basic Thread Creation
```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Create and spawn a new thread
    let handle = thread::spawn(|| {
        // Thread work here
        for i in 1..10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for thread to complete
    handle.join().unwrap();
}
```

## Concurrency Challenges

```mermaid
graph LR
    A[Shared Resource] --> B{Race Condition}
    B --> C[Thread 1]
    B --> D[Thread 2]
    B --> E[Thread 3]
    
    C --> F[Critical Section]
    D --> F
    E --> F
    
    F --> G[Mutex]
    G --> H[Safe Access]
    
    style B fill:#ff9999
    style F fill:#ffff99
    style G fill:#99ff99
    style H fill:#99ff99
```

### 1. Race Conditions
- Occurs when multiple threads access shared data simultaneously
- Result is non-deterministic
- Example of a race condition:
  ```rust
  // Dangerous in concurrent context
  a = a + 1  // Can be interrupted between read and write
  ```

### 2. Critical Sections
- Portions of code that access shared resources
- Must be protected to ensure thread safety

### 3. Solutions

#### Mutex (Mutual Exclusion)
- Ensures only one thread can access protected data at a time
- Key concept: Lock and unlock mechanism
- Prevents race conditions

#### Atomic Operations
- Operations that complete in a single step
- Cannot be interrupted
- Example: Bank transactions (withdrawal + deposit)

#### Condition Variables
- Enable thread synchronization
- Allow threads to wait for specific conditions
- Used for coordinating thread execution

## Best Practices
1. Minimize critical sections
2. Use appropriate synchronization mechanisms
3. Avoid shared state when possible
4. Consider using message passing
5. Always handle thread joins properly

## Key Concepts Summary

```mermaid
mindmap
  root((Concurrency))
    Threads
      Program Counter
      Private Registers
      Stack
    Shared Resources
      Code
      Heap
      Global Variables
    Problems
      Race Conditions
      Deadlocks
      Resource Contention
    Solutions
      Mutex
      Atomic Operations
      Condition Variables
    Best Practices
      Minimize Critical Sections
      Handle Thread Joins
      Use Message Passing
```

### Final Takeaways
- Critical Sections: Regions accessing shared resources
- Race Conditions: Concurrent access leading to undefined behavior
- Deterministic vs Non-deterministic Execution
- Mutual Exclusion: Controlled access to shared resources
