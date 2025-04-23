# Rust Threading Assignments

Assignments focused on multithreading concepts in Rust, complete with starter code and helpful hints. 
These assignments will help you understand and implement common threading patterns in Rust.

## Assignment 1: Spawning Threads and Joining Them

**Objective:**
- Spawn 3 threads
- Each thread should print its identifier
- The main thread should wait for all threads to complete
- After all threads have finished, the main thread should print "All threads completed."

**Starter Code:**
```rust
use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 3 threads
    for i in 1..=3 {
        // TODO: Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            // Simulate some work
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        
        // TODO: Store the handle
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    
    
    println!("All threads completed.");
}
```

**Hints:**
- Look at the `JoinHandle` object returned by `thread::spawn`
- The `join()` method on a thread handle will wait for that thread to finish
- You can use a `for` loop to iterate through all the handles in your vector
- Don't forget to use `unwrap()` or error handling when joining threads

## Assignment 2: Sharing Counter Data Between Threads

**Objective:**
- Define a shared counter that starts from zero
- Spawn 5 threads where each thread increments the counter by 1, 10 times
- Use Arc and Mutex to share and safely update the counter across threads
- The main thread should print the final value of the counter after all threads have completed

**Starter Code:**
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Create a shared counter using Arc and Mutex
    
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for i in 1..=5 {
        // TODO: Clone the Arc for the thread
        
        
        // TODO: Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            // TODO: Increment counter 10 times
            
            
        });
        
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    
    
    // TODO: Print the final value of the counter
    
}
```

**Hints:**
- Start by initializing your counter with `let counter = Arc::new(Mutex::new(0));`
- Remember to clone the Arc before moving it into each thread with `Arc::clone(&counter)`
- Inside each thread, you'll need to:
  - Use a loop to repeat the increment 10 times
  - Lock the mutex with `let mut num = counter_clone.lock().unwrap();` 
  - Modify the value with `*num += 1;`
- The lock is automatically released when the `MutexGuard` variable goes out of scope
- Don't forget to join all threads before printing the final value
- To access the counter value at the end: `*counter.lock().unwrap()`

## Assignment 3: Thread Pool Implementation

**Objective:**
- Create a simple thread pool with a configurable number of worker threads
- Submit at least 10 tasks to the thread pool
- Each worker thread should print its identifier and the task it's processing
- Implement a clean shutdown mechanism for the thread pool
- The main thread should wait for all tasks to complete before exiting

**Starter Code:**
```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // TODO: Create a channel for sending jobs
        
        
        // TODO: Create and store workers
        
        
        // TODO: Return the ThreadPool
        
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Create a job from the closure and send it to a worker
        
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: Send terminate message to all workers
        
        
        // TODO: Wait for all workers to finish
        
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Create a thread that loops and receives jobs from the channel
        
        
        // TODO: Return the Worker
        
    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}
```

**Hints:**
- For the `ThreadPool::new` method:
  - Create a channel using `let (sender, receiver) = mpsc::channel();`
  - Wrap the receiver in `Arc<Mutex<...>>` to share it among workers
  - Create workers in a loop and store them in a vector
  
- For the `execute` method:
  - Box the closure: `let job = Box::new(f);`
  - Send it as a message: `self.sender.send(Message::NewJob(job))`
  
- For the `Worker::new` method:
  - Create a thread using `thread::spawn`
  - Inside the thread, use a loop that receives messages
  - Use pattern matching on the message: `match message {...}`
  - Break the loop when receiving a `Terminate` message
  
- For the `Drop` implementation:
  - Send a `Terminate` message to each worker
  - Join each worker's thread using `if let Some(thread) = worker.thread.take() {...}`
  
- The `Option` in `thread: Option<JoinHandle<()>>` lets you take ownership of the thread to join it

## Assignment 4: Producer-Consumer Pattern with Termination Signal

**Objective:**
- Implement a producer-consumer pattern using a single channel
- Create 2 producer threads that generate random numbers and send them to a channel
- Create 3 consumer threads that receive numbers from the channel and process them
- After producing the required number of items, send a special termination value to signal consumers to exit
- Each thread should identify itself when printing results
- Use proper synchronization to ensure clean shutdown

**Starter Code:**
```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    
    
    // TODO: Create 2 producer threads
    
    
    // TODO: Create 3 consumer threads
    
    
    // TODO: Wait for all threads to finish
    
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
}
```

**Hints:**
- Create a single mpsc channel for data transfer: `let (tx, rx) = mpsc::channel();`
- Use `Arc<Mutex<Receiver<i32>>>` to share the receiver among consumer threads
- After producers have sent all their data items, the main thread should:
  - Send the termination signal (TERMINATION_SIGNAL) once for each consumer
  - Example: `for _ in 0..num_consumers { tx.send(TERMINATION_SIGNAL).unwrap(); }`
- In the consumer function:
  - Check if received value equals TERMINATION_SIGNAL
  - If it does, break the loop and exit the thread
  - If not, process the value normally
- Make sure to wait for all threads to complete before exiting the program

## Key Threading Concepts

Here's a quick summary of the key concepts covered in these assignments:

1. **Basic Threading**
   - `thread::spawn()` creates a new thread
   - `JoinHandle.join()` waits for a thread to complete
   - Using handles to manage thread lifecycle

2. **Thread Synchronization**
   - `Arc` (Atomic Reference Counting) for sharing data between threads
   - `Mutex` for exclusive access to shared data
   - Using channels for thread communication
   - `mpsc` for Multiple Producer, Single Consumer channels

3. **Thread Pool Pattern**
   - Managing a fixed set of worker threads
   - Using channels to distribute work
   - Implementing clean shutdown mechanisms

4. **Producer-Consumer Pattern**
   - Separating data production and consumption
   - Using channels for thread coordination
   - Special values for signaling termination
   - Shared receivers with Arc and Mutex

