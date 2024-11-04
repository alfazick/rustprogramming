// Intro to  Concurrency: Base Examples

fn simple_spawn_join() {
    println!("Intro to Concurrency");
    let steps =  Box::new(5);
    let thread = std::thread::spawn(move ||{
        // important to notice usage of closure
        // it captures the environment, and steps
        // variable becomes available in our new thread
        for step in 1..=*steps {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("Thread step {}",step);
        }
        "Goodbye!" // important thread could return values
    });

    println!("Spawned a thread!");

    // Very important moment to understand closure captures
    // the environment
    
    //println!("steps now unavailable {}", steps);
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    // Now we join our spawned thread with it's returned value, if we don't our function just returns
    // without waiting for spawned thread
    let result = thread.join().unwrap(); // we need to unwrap result enum,because potentially thread could panick and we end up with err

    println!("Thread returned: {:?}", result);
    
}

fn share_data_between_threads_with_arc() {
    use std::sync::Arc; // atomic reference counter(smart pointer)
    
    println!("Intro to Concurrency");
    let steps =  Arc::new(5);
    let thread = {
        let steps = steps.clone();
        std::thread::spawn(move ||{
            for step in 1..=*steps {
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}",step);
            }
            "Goodbye!" // important thread could return values
        })
    };

    println!("Spawned a thread!");

    // Very important moment to understand closure captures
    // the environment
    
    println!("steps now available {}", steps);
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    // Now we join our spawned thread with it's returned value, if we don't our function just returns
    // without waiting for spawned thread
    let result = thread.join().unwrap(); // we need to unwrap result enum,because potentially thread could panick and we end up with err

    println!("Thread returned: {:?}", result);
}


fn share_data_between_threads_through_channel() {
    // multiple producer, single consumer
    use std::sync::mpsc;

    println!("Concurrency");
    let (sender,receiver) = mpsc::channel(); // notice tuple unpacking

    let thread = {
        std::thread::spawn(move ||{
            let steps = receiver.recv().unwrap();
            for step in 1..=steps {
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}",step);
            }
            "Goodbye!" // important thread could return values
        })
    };

    println!("Spawned a thread!");
    sender.send(5).unwrap();// if message did not reach reciever, you get err
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    let result = thread.join().unwrap();
    println!("Thread returned: {:?}", result);
    
}

fn share_data_between_threads_and_mutate() {
    use std::sync::Arc; // atomic reference counter(smart pointer)
    use std::sync::Mutex; // mutex -> mutual exclusive
    // smart pointer which guarantess that only one thread with lock
    // acquired will be able to mutate the value inside
    
    println!("Intro to Concurrency");
    let steps =  Arc::new(Mutex::new(5));
    let thread = {
        let steps = steps.clone();
        std::thread::spawn(move ||{
            while *steps.lock().unwrap() > 0{
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}",steps.lock().unwrap());
                *steps.lock().unwrap() -=1 ;
            }
            "Goodbye!" // important thread could return values
        })
    };

    println!("Spawned a thread!");

    // Very important moment to understand closure captures
    // the environment
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    
    let result = thread.join().unwrap(); 
    println!("Thread returned: {:?}", result);
}

fn main() {
    simple_spawn_join();
    share_data_between_threads_with_arc();
    share_data_between_threads_through_channel();
    share_data_between_threads_and_mutate();
}



// Heavier examples:
fn arc_example() {
// Arc by itself is good to keep immutable data
    use std::thread;
    use std::sync::Arc;
    
    let some_resource = Arc::new("Hello World".to_string());
    
    
    let thread_a = {
        let some_resource = some_resource.clone();
        thread::spawn(move || {
            println!("Thread A says: {}",some_resource);
        })
    };
    
    let thread_b = {
        let some_resource = some_resource.clone();
        thread::spawn(move || {
            println!("Thread B says: {}",some_resource);
        })
    };
    
    thread_a.join().unwrap();
    thread_b.join().unwrap();
    
}

fn mutex_example() {
    //Mutex guarantees mutual exclusion, so putting your data inside of mutex
    // allows to use mechanisms for lock and unlock, to guarantee that no other threads will be able to mutate your data.
    
    // use std::sync::Mutex;
    // use std::thread;
    
    struct Message(String);
    
    let msg = Message("Hello".to_string());
    
    let mutex = std::sync::Mutex::new(msg);
    let arc = std::sync::Arc::new(mutex);
    let child; // I want to be able to join my thread, later
    
    { 
        let arc = arc.clone();
        child = std::thread::spawn(move || { 
            let mut guard = arc.lock().unwrap();
            guard.0 = "World!".to_string();
            println!("{}",guard.0);
            // unlock will be called at the moment your variable scope ends!!
            // otherwise you gonna hold your mutex locked;
        });
    }
    
    {
        let guard = arc.lock().unwrap();
        println!("{}",guard.0);
        
    }
    
    child.join().unwrap();
    
}



Spawning Threads and Joining Them

Spawns 3 threads.
Each thread should print its identifier (e.g., Thread 1, Thread 2, etc.).
The main thread should wait for all threads to complete their execution.
After all threads have finished, the main thread should print "All threads completed."

Assignment 2: Sharing Counter Data Between Threads
Define a shared counter that starts from zero.
Spawn 5 threads where each thread increments the counter by 1, 10 times (just a for loop), which should result in the counter having the value of 50 at the end.
Use Arc and Mutex to share and safely update the counter across threads.
The main thread should print the final value of the counter after all threads have completed their execution.

fn sending_data_across_threads() {
    extern crate rand; // 0.8.5

    use std::thread;
    // multiproducer, single consumer
    use std::sync::mpsc::channel;

    let (sender,reciever) = channel();

    for i in 0..10 {
        let sender = sender.clone();
        thread::spawn(move || {
            println!("sending: {}",i);
            sender.send(i).unwrap(); // any data could be passed to reciever
            // as well as sending could fail
        });
    }

    for _ in 0..10 {
        let msg = reciever.recv().unwrap();
        println!(" recieved {}", msg );
    }
    // what is important to notice, data will be send and recieved in random order
    // but you will get them in exact order, just be aware of potential queue

    // basically CPU whim

}

fn fortune_cookie() {
    extern crate rand;
    use rand::Rng;
    use std::thread;
    // multiproducer, single consumer
    use std::sync::mpsc::channel;
    
    use std::time;

    let ten_millis = time::Duration::from_millis(1000);
    
    const DISCONNECT: &str = "Come back tomorrow!";
    
    let (sender,reciever) = channel();
    
    
    
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            
            let msg = match rng.gen_range(0..5)  {
                0 => "Fortune favors the brave.",
                1 => DISCONNECT,
                2 => "You will travel to many exotic places in your lifetime.",
                3 => "You can make your own happiness.",
                4 => "You are very talented in many ways.",
                _ => unreachable!(),
            };
            
            println!("Sending cookie: {}",msg);
            //thread::sleep(ten_millis);
            sender.send(msg).unwrap();
            if msg == DISCONNECT {
                break;
            }
        }
    });
    
    for recieved_msg in reciever {
        println!("What a day. Your fortune cookie : {}",recieved_msg);
        thread::sleep(ten_millis);
        
    }
    
}

fn rw_locks() {
    // comparing with mutex which does not separate with reading and writing
    // to the data inside of mutex Read and Write lock, allows to separate that logic
    // like many readers and single writer, very close to RefCell.
    // but this idea is not new or unique to Rust, this idea existed before dawn
    // in Java and C++
    
    use std::sync::{Arc, RwLock};
    use std::thread;
    
    let data = Arc::new(RwLock::new("Hello World".to_string()));
    use std::time;
    let ten_millis = time::Duration::from_millis(10);
    let twenty_millis = time::Duration::from_millis(40);
    
    let reader_a = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let data_for_read = data.read().unwrap();
                println!("Data from reader_A: {} ",data_for_read);
                thread::sleep(ten_millis);
            }
        })
    };
    
    let reader_b = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let data_for_read = data.read().unwrap();
                println!("Data from reader_B: {} ",data_for_read);
                thread::sleep(ten_millis);
            }
        })
    };
    
    let writer = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let mut data_to_write = data.write().unwrap();
                data_to_write.push('!');
                println!("Updating data {} ",data_to_write);
                thread::sleep(twenty_millis);
                
            }
        })
        };
        
    reader_a.join().unwrap();
    reader_b.join().unwrap();
    writer.join().unwrap();
    
    println!("{:?}",data);
    
}

fn main() {
    arc_example();
    mutex_example();
    sending_data_across_threads();
    fortune_cookie();
    rw_locks();
    
}


