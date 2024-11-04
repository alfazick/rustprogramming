Part I
"Introduction to Concurrency"

Example:
3 100 banknote
3 people

Don't communicate with each other.
Move to pick up.

Process of distribution is a task.

Can fail, if people not communicate
or could be slower, if one person will take one note at a time

Simple, basic, but accurate image of concurrency programming.

Introduction:

=> New abstraction: Thread
Our program will have many point of execution

Before it was one single thread.

Each thread is like a separate process.
Key idea: It shares the same address space and the same data

=> State of the single thread:

Program Counter
Private registers(context switch still works)

[Single Process:
"Thread1" (individual thread control block),
"Thread2",
"Thread3",]

Distinction with processes.
All threads share the same address space as a parent process

In addition, in order to distinguish threads, each one 
will have individual stack

Single Process, Single Thread
[Code,
Heap,
Stack]

Single Process, Multiple Thread
[Code,
Heap
Stack1,
Stack2,
Stack3]

Stack becomes thread-local storage and keeps
all variable, parameters and return values

Why use Threads?

1) Parallelism
Imagine you need to process 10000 arrays.
And you want a find total sum
single threaded program one array at a time

multi threaded program you can assign arrays let's say between 10 threads
and assign each thread for 1000 arrays.
This is a parallelization.

2) Interactivity.

So when you program run on CPU
if at some point instructions says open the file
you gave up CPU and become blocked, and this can be slow.

So instead of relinquishing CPU on every IO operation, you want to have multiple threads, to utilize the usage of CPU by your program.

=> Thread Creation Concepts

->1 create a thread:-> spawn
->2 wait thread to finish:->join


=> Bad example
use std::thread;
use std::time::Duration;

'''
fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

'''
=> Correct Example
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
'''

Problems:
Shared data.
So we observed that data which lives on the heap will be shared by all threads.
And here starts all the problems.

Namely it's called Data Race - Race Condition.
In Rust it does not exist. Bu we need to discuss.
To understand things at depth.

=> Uncontrolled Scheduling.
a = a + 1 

1) move value of a to register // load
2) perform adding one (bad scenario arises when time interrupt occurs , exactly at this moment) // update
3) Put back // store

Which technically means if two threads try to update same shared data result will be not deteremnistic.

So this code which involves updating shared data on heap, refers as a critical section.

And solution is so called mutual exclusion.
Edsger Dijkstra genious. Come up with this ideas. in 80s already.

Mutex -> Mutual exclusion
Key idea: Allow to change the data only to one thread at a time

Concept of Atomic Operation:
Perform all or nothing.
Simple example is a bank transaction.
Operation Send Consists of Withdraw and Add technically.
So it's a two operation and you want the execute either both or none.


And everything we want to achieve in multithreaded programs is to access critical sections in synchronised and controlled maner.

Condition Variables
So another idea.
If we want to synchronize our threads.
Let's say we cannot continue of execution of one thread
till previous did not finish.

There is a need for sleeping and waking aprropriate threads
Solution is Condition Variables

Example problem
https://leetcode.com/problems/print-zero-even-odd/

Key Takeways:

=> A critical section
=> Race condition
=> Deterministic and indeterministic
=> Mutual exclusion Mutex















