
// Synchronisation primitives
#1) Condvar
Condvar (short for "condition variable"): This is a way to synchronize multiple threads. It allows a thread to wait for a certain condition to be met before it continues its execution. Think of it as a waiting room, where threads can wait until they're allowed to go on.

#2) Barrier
Barrier: This is another way to synchronize threads. It allows multiple threads to wait for each other to reach a certain point, like a checkpoint. Think of it as a starting line, where everyone waits for the race to start.

// #3) Monitor (mutex + condvar)
Monitor: This is a higher-level synchronization mechanism, which combines a mutex (for mutual exclusion) and a condvar (for waiting on conditions). It helps protect shared data by only allowing one thread at a time to access it and by allowing threads to wait on specific conditions.


Producer-Consumer Pattern -> Simple Round-Robin Load Balancer


# Code Section:


// #1) Conditional Variable

fn condvar_synchornisation() {

    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    let pair = Arc::new((Mutex::new(0), Condvar::new()));
    let pair2 = pair.clone();

    let worker = thread::spawn(move || {
        let (num_lock, cvar) = &*pair2;
        let mut num = num_lock.lock().unwrap();
        *num = 5;
        cvar.notify_one();
    });

    let (num_lock, cvar) = &*pair;
    let mut num = num_lock.lock().unwrap();
    while *num != 5 {
        num = cvar.wait(num).unwrap();
    }

    println!("Value is now 5!");

    worker.join().unwrap();
}


// #2) Barrier
fn barrier_synchornisation() {

    use std::sync::{Arc, Barrier};
    use std::thread;
    
    
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = Vec::new();

    for i in 0..3 {
        let barrier_clone = barrier.clone();
        let handle = thread::spawn(move || {
            println!("Thread {} is waiting", i);
            barrier_clone.wait();
            println!("Thread {} starts running", i);
        });

        handles.push(handle);
    }
    
    // Please notice. Every time you get different order.
    // You never know how CPU schedules your code to be executed
    // But most important idea. Every execution will wait at the barrier
    // before continuying execucution.This is key idea for this synchronisation

    for handle in handles {
        handle.join().unwrap();
    }
    
}

// #3) Mutex + CONDVAR == Monitor
// There is an idea of conditional variable.
// The goal is not waiste time by spinning CPU when lock can not be acquired
// so, in order to mitigate this issue.
// We can put our thread to wait, by putting our lock inside of conditional variable

// And as soon as, one of the threads make a required progress, you will notify waiting
// thread to wake up

fn mutex_with_condvar(){
    // another name for using mutex with conditional variable is called -> "monitor".
// https://docs.rs/monitor/latest/src/monitor/monitor.rs.html#8-11
    
    #[derive(Debug)]
    struct Counter{
        cnt:i32,
    }

    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    
    let pair = Arc::new((Mutex::new(Counter{cnt:0}), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    
    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut shared_data = lock.lock().unwrap();
        
        shared_data.cnt += 1; // if I will comment this line, 
        // other thread will wait forever
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });
    
    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut shared_data = lock.lock().unwrap();
    while !(shared_data.cnt % 2 == 1) {
        shared_data = cvar.wait(shared_data).unwrap();
        println!("I am just waiting");
    }
    println!("This line will not be executed");
    println!("{:?}",shared_data);
}


// Producer-Consumer Model: Simple
fn producer_consumer_pattern(){
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    use std::time::Duration;
    
    const BUFFER_SIZE: usize = 5;
    
    #[derive(Default)]
    struct Buffer {
        data: Mutex<Vec<i32>>,
        condvar: Condvar,
    }
    
    impl Buffer {
        fn new() -> Self {
            Self {
                data: Mutex::new(Vec::with_capacity(BUFFER_SIZE)),
                condvar: Condvar::new(),
            }
        }
    
        fn produce(&self, item: i32) {
            let mut data_lock = self.data.lock().unwrap();
    
            while data_lock.len() >= BUFFER_SIZE {
                data_lock = self.condvar.wait(data_lock).unwrap();
            }
    
            data_lock.push(item);
            self.condvar.notify_one();
        }
    
        fn consume(&self) -> i32 {
            let mut data_lock = self.data.lock().unwrap();
    
            while data_lock.is_empty() {
                data_lock = self.condvar.wait(data_lock).unwrap();
            }
    
            let item = data_lock.remove(0);
            self.condvar.notify_one();
            item
        }
    }
    
    
    let buffer = Arc::new(Buffer::new());
    let producer_buffer = buffer.clone();
    let consumer_buffer = buffer.clone();

    let producer = thread::spawn(move || {
        for i in 0..10 {
            producer_buffer.produce(i);
            println!("Produced {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            let item = consumer_buffer.consume();
            println!("Consumed {}", item);
            thread::sleep(Duration::from_millis(100));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

}



// Stand-alone example: Producer-Consumer Model.
// Goal to simulate a system where multiple producer threads generate log entries, and a single consumer thread writes them to a file.


use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

const MAX_MESSAGES: usize = 10;

#[derive(Debug, Default)]
struct LogBuffer {
    messages: Mutex<VecDeque<String>>,
    cvar: Condvar,
    active_producers: Mutex<usize>,
}

impl LogBuffer {
    fn retrieve(&self) -> Option<String> {
        let mut messages_lock = self.messages.lock().unwrap();

        while messages_lock.is_empty() {
            if *self.active_producers.lock().unwrap() == 0 {
                return None;
            }
            messages_lock = self.cvar.wait(messages_lock).unwrap();
        }

        let msg = messages_lock.pop_front();
        self.cvar.notify_all();
        msg
    }

    fn post(&self, new_message: String) {
        let mut messages_lock = self.messages.lock().unwrap();

        while messages_lock.len() >= MAX_MESSAGES {
            messages_lock = self.cvar.wait(messages_lock).unwrap();
        }

        messages_lock.push_back(new_message);
        self.cvar.notify_one();
    }
}

fn producer(id: u32, buffer: &LogBuffer) {
    let messages = vec![
        format!("Producer {} - Log entry 1", id),
        format!("Producer {} - Log entry 2", id),
        format!("Producer {} - Log entry 3", id),
    ];

    for msg in messages {
        thread::sleep(Duration::from_millis(100));
        buffer.post(msg);
    }

    *buffer.active_producers.lock().unwrap() -= 1;
}

fn consumer(buffer: &LogBuffer, output_file: &str) {
    let mut file = File::create(output_file).expect("Unable to create output file");

    while let Some(msg) = buffer.retrieve() {
        writeln!(&mut file, "{}", msg).expect("Unable to write to output file");
        println!("Written to file: {}", msg);
        thread::sleep(Duration::from_millis(150));
    }
}

fn main() {
    let buffer = Arc::new(LogBuffer::default());
    let num_producers = 3;

    // Set the initial number of active producers
    *buffer.active_producers.lock().unwrap() = num_producers;

    let mut producer_threads = vec![];

    for i in 0..num_producers {
    let buffer_clone = buffer.clone();
    let handle = thread::spawn(move || {
        producer(i as u32, &buffer_clone);
    });
    producer_threads.push(handle);
}


    let buffer_clone = buffer.clone();
    let output_file = "log_output.txt";
    let consumer_thread = thread::spawn(move || {
        consumer(&buffer_clone, output_file);
    });

    for handle in producer_threads {
        handle.join().unwrap();
    }

    consumer_thread.join().unwrap();
}



// Load Balancer: StandAlone example

//Imagine you have a list of tasks that need to be performed, and a group of workers available to perform these tasks. 
//Instead of assigning all tasks to one worker or randomly choosing a worker, you would like to distribute the tasks evenly
//among the workers.

//Here's how round-robin works:

//Keep track of the next worker in line to receive a task (let's call this worker the "current" worker).
//When a new task arrives, assign the task to the current worker.
//Move to the next worker in the list, wrapping around to the first worker when you reach the end of the list.
//Repeat steps 2 and 3 for each incoming task.
//The round-robin algorithm essentially treats the workers as a circular list. It ensures that each worker is assigned a task in turn, distributing the workload evenly among them.

//In the context of the load balancer example, the workers are the backend servers, and the tasks are the incoming requests. 
//The load balancer uses the round-robin algorithm to distribute the requests evenly among the backend servers 
//to prevent any single server from becoming overloaded.

use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;

const NUM_WORKERS: usize = 3;

struct LoadBalancer {
    senders: Vec<Sender<String>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl LoadBalancer {
    fn new(num_workers: usize) -> Self {
        let mut senders = Vec::with_capacity(num_workers);
        let mut workers = Vec::with_capacity(num_workers);

        for id in 0..num_workers {
            let (sender, receiver) = mpsc::channel();
            senders.push(sender);
            let worker_handle = thread::spawn(move || Self::worker(id, receiver));
            workers.push(worker_handle);
        }

        Self { senders, workers }
    }

    fn distribute_requests(&mut self, requests: Vec<String>) {
        let mut round_robin_counter = 0;
        for request in requests {
            let sender = &self.senders[round_robin_counter];
            sender.send(request).unwrap();
            round_robin_counter = (round_robin_counter + 1) % self.senders.len();
        }
    }

    fn worker(id: usize, receiver: Receiver<String>) {
        loop {
            match receiver.recv() {
                Ok(request) => {
                    println!("Worker {} processing {}", id, request);
                    thread::sleep(Duration::from_millis(100));
                }
                Err(_) => break, // Channel is closed, stop the worker.
            }
        }
    }
}

impl Drop for LoadBalancer {
    fn drop(&mut self) {
        self.senders.clear();
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

fn generate_requests() -> Vec<String> {
    vec![
        "Request 1".to_string(),
        "Request 2".to_string(),
        "Request 3".to_string(),
        "Request 4".to_string(),
        "Request 5".to_string(),
    ]
}

fn main() {
    let mut load_balancer = LoadBalancer::new(NUM_WORKERS);
    let requests = generate_requests();
    load_balancer.distribute_requests(requests);
}




