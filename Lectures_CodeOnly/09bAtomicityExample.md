use std::sync::atomic::{AtomicI32, Ordering};
use rand::Rng;
use std::sync::Arc;
use std::thread;

struct Account {
    balance: AtomicI32,
}

impl Account {
    fn new(initial_balance: i32) -> Self {
        Account {
            balance: AtomicI32::new(initial_balance),
        }
    }

    fn deposit(&self, amount: i32) -> bool {
        let mut rng = rand::thread_rng();

        // Simulating a failure 10% of the time.
        if rng.gen_range(0..100) < 10 {
            return false;
        }

        let current_balance = self.balance.load(Ordering::SeqCst);
        self.balance.store(current_balance + amount, Ordering::SeqCst);
        true
    }

    fn withdraw(&self, amount: i32) -> bool {
        let current_balance = self.balance.load(Ordering::SeqCst);

        if current_balance < amount {
            return false;
        }

        self.balance.store(current_balance - amount, Ordering::SeqCst);
        true
    }
}

fn transfer(source: &Account, destination: &Account, amount: i32) -> bool {
    if source.withdraw(amount) {
        if destination.deposit(amount) {
            true
        } else {
            println!("Deposit to destination failed! Rolling back...");
            source.deposit(amount);  // Revert the withdrawal.
            false
        }
    } else {
        println!("Withdrawal from source failed!");
        false
    }
}

fn main() {
    let source = Arc::new(Account::new(1000));
    let destination = Arc::new(Account::new(1000));

    let mut handles = vec![];

    for _ in 0..10 {
        let source_clone = source.clone();
        let destination_clone = destination.clone();

        let handle = thread::spawn(move || {
            if !transfer(&source_clone, &destination_clone, 100) {
                println!("Transfer failed!");
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    //The Ordering enum in Rust's standard library helps us specify the 
    //desired ordering guarantees  operations
    
    // When using SeqCst, Rust guarantees that:

    //--> All operations appear to execute in the same order for all threads.
    //--> It provides the strongest ordering guarantees.
    
    //load(Ordering::SeqCst) is a method you can call on an atomic type to read its value
    

    println!("Final balance of source: {}", source.balance.load(Ordering::SeqCst));
    println!("Final balance of destination: {}", destination.balance.load(Ordering::SeqCst));
}


// More robust 
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use rand::Rng;

struct Account {
    balance: AtomicI32,
}

impl Account {
    fn new(initial_balance: i32) -> Self {
        Account {
            balance: AtomicI32::new(initial_balance),
        }
    }

    // Using an atomic operation to add to the balance
    fn deposit(&self, amount: i32) -> bool {
        let mut rng = rand::thread_rng();

        // Simulating a failure 10% of the time.
        if rng.gen_range(0..100) < 10 {
            return false;
        }

        // `fetch_add` is an atomic operation that loads and updates the balance in one step.
        self.balance.fetch_add(amount, Ordering::SeqCst);
        true
    }

    // Using atomic operations to withdraw from the balance
    fn withdraw(&self, amount: i32) -> bool {
        loop {
            // Start by loading the current balance.
            let current_balance = self.balance.load(Ordering::SeqCst);

            // Check if the balance is sufficient for the withdrawal.
            if current_balance < amount {
                return false;
            }

            // Attempt to atomically update the balance.
            // `compare_exchange` will only write if the current value is what we expect.
            // If the balance has been updated by another thread, we retry.
            match self.balance.compare_exchange(current_balance, current_balance - amount, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return true,  // The withdrawal succeeded.
                Err(_) => continue,   // The withdrawal failed because the current balance was not what we expected. Retry.
            }
        }
    }
}

fn transfer(source: &Account, destination: &Account, amount: i32) -> bool {
    if source.withdraw(amount) {
        if destination.deposit(amount) {
            true
        } else {
            // If the deposit fails, the withdrawal has already happened,
            // and we should not attempt to deposit back the withdrawn amount
            // without ensuring atomicity of the whole transfer.
            // This would require a more complex transaction mechanism,
            // potentially with compensating transactions or a retry logic.
            println!("Deposit to destination failed! The transfer is incomplete and requires manual reconciliation.");
            false
        }
    } else {
        println!("Withdrawal from source failed!");
        false
    }
}

fn main() {
    let source = Arc::new(Account::new(1000));
    let destination = Arc::new(Account::new(1000));

    let mut handles = vec![];

    for _ in 0..10 {
        let source_clone = source.clone();
        let destination_clone = destination.clone();

        let handle = thread::spawn(move || {
            if !transfer(&source_clone, &destination_clone, 100) {
                println!("Transfer failed!");
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final balance of source: {}", source.balance.load(Ordering::SeqCst));
    println!("Final balance of destination: {}", destination.balance.load(Ordering::SeqCst));
}
