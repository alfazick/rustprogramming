
// Single server.
// Works fine but problem is it can serve only one client at a time.
// So he basically blocks the server, which is not good.

// Intro to problem

// // Section1 Code Start
// use std::io::prelude::*;
// use std::io::BufReader; 
// use std::net::TcpListener;
// use std::net::TcpStream;

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         handle_connection(stream); 
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     use std::fs;
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     // everything is a same, but rewiritten with match
//     // and addition of sleep, if the client tries to request sleep service
    
//     let (status_line, filename) = match &request_line[..] {
//         "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
//         "GET /sleep HTTP/1.1" => {
//             thread::sleep(Duration::from_secs(5));
//             ("HTTP/1.1 200 OK", "hello.html")
//         }
//         _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
//     };

    

//     let contents = fs::read_to_string(filename).unwrap();
//     let length = contents.len();

//     let response =
//         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();

// }

// Section1 Code END

// Ok so for now you should have an intuition that you need to process streams, in some kind of non-blocking fashion. One is with use of event-based concurrency another with creating multiple threads.
// But keep in mind general logic is still the same. You want to avoid
// of blocking your event-loop.

// Even though code may become intimidating at some point. It just serves one purpose -> "NON_BLOCKING CODE".

// Section2 Code Start. Simple Solution: Spawn thread for each request

// use std::io::prelude::*;
// use std::io::BufReader; 
// use std::net::TcpListener;
// use std::net::TcpStream;

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         // spawn a new thread to handle every connection
//         thread::spawn(|| {
//             handle_connection(stream);
//         });
//     }
// }


fn handle_connection(mut stream: TcpStream) {
    use std::fs;
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // everything is a same, but rewiritten with match
    // and addition of sleep, if the client tries to request sleep service
    
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}

// Section2 Code End.

// Well above solution is almost fine, but it's kind of problematic
// to spawn a thread allocate a separate stack and then clean it up for next client, it's expensive.

// You may have wish, just limit the amount of spawned threads, and keep the alive for while till next request arrive and then just to hande him the job he needs to do.

// So you have a need to manage many threads in efficient manner.
// And this is the idea for thread pool come.

// You want to have a data structure for threads. Which just provide you with some kind of API, making your life simpler.

// so, basically you want to have something like that

// -> ThreadPool::new(N) <- struct for managing the spawned threads
// where you can specify number of active threads at most N

// Non-blocking event loop. Where each active thread handles each incoming request independently

// for stream in server.incoming() {
//     ThreadPool.execute(|| {handle_connection(stream)});
// }


// Section3 Code Start.


// struct ThreadPool {
//     threads: Vec<Worker>, // Worker is a struct which is responsible for
//     // managing single thread
// }

// struct Worker {
//     id: usize,
//     thread: thread::JoinHandle<()>, // -> every worker only and only responsible for one active thread, after spawining we end up with JoinHandle on which we can block later to finish it's work.
// }

// impl Worker {
//     fn new (id:usize) -> Worker {
//         let empty_thread = thread::spawn(|| {}); // well initially we have nothing to do
//         Worker {id, thread:empty_thread}
//     }
    
// }

// impl ThreadPool {
//     fn new(size: usize) -> ThreadPool {
//         // we need to create a container, were we gonna keep our threads

//         let mut threads = Vec::with_capacity(size);

//         for id in 0..size {
//             // now we need to actually fill out our vector with new active  threads which we are about to spawn. Even though they may not have a job to do yet. Important to understand. And this leads us to need to create another struct actual WORKER:)
//             threads.push(Worker::new(id));

//         }

//         ThreadPool{threads} // last expression will be returned
//         // so no semicolon
//     }

//     fn execute<TASK>(&self, f: TASK)
//         where TASK: FnOnce() + Send + 'static,
//         // here interesting things happen
//         // thread spawn accepts a closure, and we need to specify
//         // what kind of closure we gonna pass to execute method, since at some point he will call spawn thread.
//         // FnOnce() because we will pass stream and it wil take ownership of stream
//         // Send, because we want to be able to send it to some thread
//         // 'static , we don't know when thread will finish work and we will no longer need the stream and can drop it
//         {
//             // do something !
//         }
// }

// Section3 Code End.

// So far so good,so know we are left with most challenging problem
// how we will send job to run from our main thread to thread pool local worker?

// So it requires us to make an upgrade our code, and as you may already noticed the BOOK advocates communication between threads to channels.

// So it boils down to the fact, that each worker should keep a channel,
// through which we will send tasks for him to do, right?

// Excerpt from the BOOK
// "The ThreadPool will create a channel and hold on to the sender.
// Each Worker will hold on to the receiver.
// We’ll create a new Job struct that will hold the closures we want to send down the channel.
// The execute method will send the job it wants to execute through the sender.
// In its thread, the Worker will loop over its receiver and execute the closures of any jobs it receives."


// Section 4 Code Start

use std::io::prelude::*;
use std::io::BufReader; 
use std::net::TcpListener;
use std::net::TcpStream;

use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // spawn a new thread to handle every connection
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


use std::sync::mpsc; // multiple reciever single consumer model.
// we will send jobs for each thread and thread pool will recieve result of work back


struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

//struct Job; // will hold a closure to be passed to single thread to execute. This is a subtle moment to realize. It could happen that all threads, workers will be busy. and you do need to keep this job to run in the future, when some of the threads become available.

// Type Aliasing trick

type Job = Box<dyn FnOnce() + Send + 'static>; // exact syntax of closure we gonna pass to a thread, but wrapping it with Box, because we don't know the size of it at compile time, because it will take all environmental variable with itself inside a different thread, polymorphism for heap usage helps ))


use std::sync::Arc;
use std::sync:: Mutex;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {

        let (sender, receiver) = mpsc::channel();

        // so here comes another subtle momement
        // since reciever will go with worker together, it will take ownership of it, techincally, but the pronlem is, it will be used between threads, so you have a need to escape data race and have some mutual exclusion. all ideas reagrding multithreaded programs comes together.
        // That's why you need to pass a atomic reference to reciever
        // for every thread you will spawn, when Worker::new() will be called

        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id,Arc::clone(&receiver)));

        }

        ThreadPool{threads, sender} 
    }

    fn execute<TASK>(&self, f: TASK)
        where TASK: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap(); // we pass our task to execute to thread
        }
}

// main update happens on worker struct

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new (id:usize,receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");
            
            // here job is a variable which refers to closure.
            // which means that since function can be assigned to a variable
            // you can use that variable name to call the function later
            // functional programming in work :)
            job();
        });
        Worker {id, thread}
    }
    
}

// Section 4 Code END

// I know this code is a little bit overwhelming, but
// Welcome to the world of programming, there are many moving pieces, each of them individually more or less feasible to understand, but when they start to work together,
// Complexity grows exponentially.

