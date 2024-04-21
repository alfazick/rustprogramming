
// Concurrency Problems
// When concurrency could go wrong

#1) Deadlock: A deadlock occurs when two or more threads are blocked indefinitely, waiting for resources held by each other. 
This situation is like a stand-off where neither thread can make progress because they are each waiting for the other to release the resource it needs.

#2) Livelock: A livelock is a situation where two or more threads are stuck in a loop, repeatedly changing their states in response to the actions of other threads, without making any progress. 
Although the threads are technically still running, they are unable to complete their tasks
Imagine family, where one person saves money, another spends them.
This is a livelock in real life

#3) Starvation: Starvation occurs when a thread is continually denied access to the resources it needs to make progress, while other threads are allowed to proceed. 
Starvation can happen when a scheduling algorithm or resource allocation policy consistently favors some threads over others, causing the disadvantaged threads to wait indefinitely.


fn deadlock_base_struct() {
    println!("Base struct logic");
    struct Friend {
        name:String,
        
    }
    
    impl Friend {
    
        fn get_name(&'_ self) -> &'_ str  {
            &self.name
        }
        fn bow(&self,bower:&Friend) {
            println!("{} : {} has bowed to me!",self.get_name(),bower.get_name());
            bower.bow_back(&self);
        }
        
        fn bow_back(&self,bower:&Friend) {
            println!("{} : {} has bowed back to me!",self.get_name(),bower.get_name());
        }
    }
    
    let alphonse = Friend{name:"Alphonse".to_string()};
    println!("{}",alphonse.get_name());
    
    let gaston = Friend{name:"Gaston".to_string()};
    println!("{}",gaston.get_name());
    
    alphonse.bow(&gaston);
    gaston.bow(&alphonse);
    
    
}

fn deadlock_arc_base_struct() {
    use std::thread;
    use std::sync::Arc;
    
    println!(" Struct Wrapped with Atomic reference counter and thread");
    struct Friend {
        name:String,
        
    }
    
    impl Friend {
        
        
        fn get_name(&'_ self) -> &'_ str  {
            &self.name
        }
        fn bow(&self,bower:&Friend) {
            println!("{} : {} has bowed to me!",self.get_name(),bower.get_name());
            bower.bow_back(&self);
        }
        
        fn bow_back(&self,bower:&Friend) {
            println!("{} : {} has bowed back to me!",self.get_name(),bower.get_name());
        }
    }
    
    let alphonse = Arc::new(Friend{name:"Alphonse".to_string()});
    let gaston = Arc::new(Friend{name:"Gaston".to_string()});
    
    let thread_1 = {
        let alphonse = alphonse.clone();
        let gaston = gaston.clone();
        thread::spawn(move || {
            alphonse.bow(&gaston);
        })
    };
    
    let thread_2 = {
        let alphonse = alphonse.clone();
        let gaston = gaston.clone();
        thread::spawn(move || {
            gaston.bow(&alphonse);
        })
    };
    
    thread_2.join().unwrap();
    thread_1.join().unwrap();
    
    
}

fn deadlock_struct_mutex() {
    use std::thread;
    use std::sync::{Arc,Mutex};
    
    println!(" Struct With Mutex");
    
    struct Friend {
        name:String,
        
    }
    
    impl Friend {
        
        
        fn get_name(&'_ self) -> &'_ str  {
            &self.name
        }
        fn bow(&self,bower:&Friend) {
            println!("{} : {} has bowed to me!",self.get_name(),bower.get_name());
            bower.bow_back(&self);
        }
        
        fn bow_back(&self,bower:&Friend) {
            println!("{} : {} has bowed back to me!",self.get_name(),bower.get_name());
        }
    }
    
    let alphonse = Arc::new(Mutex::new(Friend{name:"Alphonse".to_string()}));
    let gaston = Arc::new(Mutex::new(Friend{name:"Gaston".to_string()}));
    
    
    
    let thread_1 = {
        let alphonse = alphonse.clone();
        let gaston = gaston.clone();
        thread::spawn(move || {
            let alphonse = alphonse.lock().unwrap();
            let gaston = gaston.lock().unwrap();
            alphonse.bow(&gaston);
        })
    };
    
    let thread_2 = {
        let alphonse = alphonse.clone();
        let gaston = gaston.clone();
        thread::spawn(move || {
            let alphonse = alphonse.lock().unwrap();
            let gaston = gaston.lock().unwrap();
            gaston.bow(&alphonse);
        })
    };
    
    
    thread_1.join().unwrap();
    thread_2.join().unwrap();
    
    
}

fn create_manually_deadlock() {
    
    use std::sync::{Arc,Mutex};
    use std::{thread, time};

    println!("Welcome to DeadLock");
    
    struct Friend {
        name:String,
        
    }
    
    impl Friend {
        
        
        fn get_name(&'_ self) -> &'_ str  {
            &self.name
        }
        fn bow(&self,bower:&Friend) {
            println!("{} : {} has bowed to me!",self.get_name(),bower.get_name());
            bower.bow_back(&self);
        }
        
        fn bow_back(&self,bower:&Friend) {
            println!("{} : {} has bowed back to me!",self.get_name(),bower.get_name());
        }
    }
    
    let alphonse = Arc::new(Mutex::new(Friend{name:"Alphonse".to_string()}));
    let gaston = Arc::new(Mutex::new(Friend{name:"Gaston".to_string()}));
    
    let thread_1 = {
        let alphonse = alphonse.clone();
        let gaston = gaston.clone();
        thread::spawn(move || {
            let alphonse = alphonse.lock().unwrap();
            // if we manually add sleeping statement
            // we intentianlly create deadlock
            // otherwise it will happen by chance
            //thread::sleep(time::Duration::from_millis(10));
            let gaston = gaston.lock().unwrap();
            alphonse.bow(&gaston);
        })
    };
    
    let thread_2 = {
        let alphonse = alphonse.clone();
        let gaston = gaston.clone();
        thread::spawn(move || {
            let gaston = gaston.lock().unwrap();
            //thread::sleep(time::Duration::from_millis(10));
            let alphonse = alphonse.lock().unwrap();
            gaston.bow(&alphonse);
        })
    };
    
    
    thread_1.join().unwrap();
    thread_2.join().unwrap();
    
    
}



fn livelock() {
    
    use std::sync::{Arc,Mutex};
    use std::{thread, time};

    println!("LiveLock");
    
    struct Line {
        people:i32,
        
    }
    
    impl Line {
        
        fn add_person(&mut self){
            self.people +=1;
            println!("One person added, line now has : {}",&self.people);
            
        }
        
        fn serve_person(&mut self) {
            self.people -=1;
            println!("One person served, line now has : {}",&self.people);
            
        }
    }
    
    let line = Arc::new(Mutex::new(Line{people:0}));
    
    
    let thread_1 = {
        let line = line.clone();
        
        thread::spawn(move || {
        loop {
            if line.lock().unwrap().people == 10 {
                continue;
            } else
            { 
                let mut line = line.lock().unwrap();
                line.add_person();
            }
        }
        })
    };
    
    let thread_2 = {
        let line = line.clone();
        
        thread::spawn(move || {
        
        loop {
           
                let mut line1 = line.lock().unwrap();
                if line1.people == 0 {
                    continue
                }
                line1.serve_person();
                thread::sleep(time::Duration::from_millis(10));
                drop(line1);
                let line = line.lock().unwrap();
                if line.people == 0 {
                    thread::sleep(time::Duration::from_millis(10));
                }
            }
        }
        )
    };
    
    
    
    //thread_1.join().unwrap();
    thread_2.join().unwrap();
    
}


// Final: Wrap everything together

// Dining Philosophers: Classic CS Problems
// https://en.wikipedia.org/wiki/Dining_philosophers_problem

// Good example:
// Key ideas to focus
// use Mutex as unstructured lock
// deadlock -> accurate thinking in multithreaded programming
// ->  starvation, you never know


fn dining_philosophers() {

    use std::{thread,time};
    use std::sync::{Mutex, Arc};
    
    struct Table {
        forks: Vec<Mutex<()>>,
    }
    
    struct Philosopher {
        name: String,
        left: usize,
        right: usize,
    }
    
    
    
    impl Philosopher {
        fn new(name: &str, left: usize, right: usize) -> Philosopher {
            Philosopher {
                name: name.to_string(),
                left: left,
                right: right,
            }
        }
        
        fn eat(&self, table: &Table) {
            let _left = table.forks[self.left].lock().unwrap(); // acquire left fork
            thread::sleep(time::Duration::from_millis(10));
            let _right = table.forks[self.right].lock().unwrap(); // acquire right fork
            
            
            // if two forks left and right are locked I can eat
            
            println!("{} is eating", self.name);
            thread::sleep(time::Duration::from_millis(10));
            println!("{} is done eating", self.name);
            
        }
    }
    
    let table = Arc::new(Table{forks:vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        ]});
        
    
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("John Locke", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4), // key idea for last philosopher switch order
        //Philosopher::new("Michel Foucault", 4, 0), // this will result in deadlock
        
        // imagine making just tiny mistake in order 
        // in concurrency, and you are basically done
    ];
    
    let handles: Vec<_> = philosophers.into_iter().map(|philosopher| {
        let table = table.clone();
        
        thread::spawn(move | | {
            philosopher.eat(&table);
        })
    }).collect();
    
    
    for t in handles {
        t.join().unwrap();
    }
    
}


fn main() {
    // Deadlock + LiveLock
    deadlock_base_struct();
    deadlock_arc_base_struct();
    deadlock_struct_mutex();
    create_manually_deadlock();
    livelock();
    
    // Classic -> Starvation is not Solved
    dining_philosophers();
}
