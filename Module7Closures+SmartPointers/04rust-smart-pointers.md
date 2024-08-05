# Rust Smart Pointers: Rc and RefCell

This document explores two important smart pointers in Rust: `Rc` (Reference Counted) and `RefCell`. These types allow for shared ownership and interior mutability, respectively, providing powerful tools for managing complex data structures and relationships in Rust programs.

## Table of Contents
1. [Rc (Reference Counting)](#rc-reference-counting)
   - [Basic Usage](#basic-usage)
   - [Sharing Resources](#sharing-resources)
2. [RefCell (Interior Mutability)](#refcell-interior-mutability)
   - [Basic Usage](#basic-usage-1)
   - [Interior Mutability Pattern](#interior-mutability-pattern)
3. [Combining Rc and RefCell](#combining-rc-and-refcell)
   - [Shared Mutable State](#shared-mutable-state)
   - [Real-world Example: Joint Bank Account](#real-world-example-joint-bank-account)

## Rc (Reference Counting)

`Rc` is a smart pointer that allows multiple ownership of the same data. It keeps track of the number of references to a value to determine when the value is no longer in use and can be cleaned up.

### Basic Usage

```rust
use std::rc::Rc;

fn reference_counting_simple() {
    let num = 10;
    let num_in_heap = Rc::new(num);

    let _copy2_of_num = Rc::clone(&num_in_heap);
    let _copy3_of_num = Rc::clone(&num_in_heap);
    let _copy4_of_num = Rc::clone(&num_in_heap);

    println!("num in heap has: {} references", 
             Rc::strong_count(&num_in_heap));
}
```

In this example, we create an `Rc` pointing to a number and then create multiple references to it. The `Rc::strong_count` function allows us to see how many references exist.

### Sharing Resources

`Rc` is particularly useful when you need to share ownership of a resource among multiple parts of your program:

```rust
use std::rc::Rc;

struct FamilyMember {
    tv: Rc<TV>,
}

struct TV;

fn sharing_resource_rc_count() {
    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(TV);
        FamilyMember {
            tv: Rc::clone(&tv_is_on),
        }
    }

    let dad = member_start_watch_tv();
    println!("How many people watching {}", Rc::strong_count(&dad.tv));

    let mom = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("How many people watching {}", Rc::strong_count(&dad.tv));

    let me = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("How many people watching {}", Rc::strong_count(&me.tv));

    drop(dad);
    drop(me);

    println!("How many people watching {}", Rc::strong_count(&mom.tv));
}
```

This example demonstrates how `Rc` can be used to share a resource (TV) among multiple family members. The TV will only be dropped when the last reference to it is gone.

## RefCell (Interior Mutability)

`RefCell` provides interior mutability, a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.

### Basic Usage

```rust
use std::cell::RefCell;

fn ref_cell_simple() {
    let num = 10;
    let data = RefCell::new(num);
    
    // Borrow the data immutably
    let data_ref = data.borrow();
    println!("Data: {}", data_ref);

    // Drop the immutable borrow so we can borrow mutably
    drop(data_ref);

    println!("Data: {:?}", data);

    // Borrow the data mutably
    let mut data_mut = data.borrow_mut();
    *data_mut += 1;
    println!("Data: {}", data_mut);
}
```

This example shows how `RefCell` allows us to borrow its contents immutably and mutably, enforcing Rust's borrowing rules at runtime.

### Interior Mutability Pattern

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn interior_mutability() {
    #[derive(Debug)]
    struct MyData {
        data: f64
    }

    let base: Rc<RefCell<MyData>> = Rc::new(RefCell::new(
        MyData {
            data: 70.00
        }
    ));

    println!("base: {:?}", base);
    
    {
        let mut base_2 = base.borrow_mut();
        base_2.data -= 10.00;
        println!("base_2: {:?}", base_2);
    }
 
    println!("base: {:?}", base);
 
    let mut base_3 = base.borrow_mut();
    base_3.data += 30.00;
 
    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
```

This example demonstrates how `RefCell` can be used to mutate data within an `Rc`, allowing for shared mutable state.

## Combining Rc and RefCell

Combining `Rc` and `RefCell` allows for shared ownership of mutable data, a powerful pattern in Rust programming.

### Shared Mutable State

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn sharing_resource_refcell_count() {
    struct FamilyMember {
        tv: Rc<RefCell<TV>>,
    }

    #[derive(Debug)]
    struct TV {
        channel: String,
    }

    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(RefCell::new(TV{channel:"BBC".to_string()}));
        FamilyMember {
            tv: tv_is_on, 
        }
    }

    let dad = member_start_watch_tv();
    let mom = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("TV channel for mom {:?}", mom.tv);

    let mut remote_control = dad.tv.borrow_mut();
    println!("TV channel {:?}", remote_control);

    remote_control.channel = "MTV".to_string();
    println!("TV channel {:?}", remote_control);
    drop(remote_control);
    println!("TV channel for mom {:?}", mom.tv);
}
```

This example shows how `Rc<RefCell<T>>` can be used to share mutable state (a TV) between multiple owners (family members).

### Real-world Example: Joint Bank Account

Here's a more practical example demonstrating the use of `Rc` and `RefCell` for a joint bank account:

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn joint_bank_account_example() {
    #[derive(Debug)]
    struct BankAccount {
        balance: RefCell<f64>,
    }
    
    impl BankAccount {
        fn new(initial_balance: f64) -> Rc<Self> {
            Rc::new(BankAccount {
                balance: RefCell::new(initial_balance),
            })
        }
    
        fn deposit(&self, amount: f64) {
            let mut balance = self.balance.borrow_mut();
            *balance += amount;
            println!("Deposited ${:.2}, new balance: ${:.2}", amount, *balance);
        }
    
        fn withdraw(&self, amount: f64) {
            let mut balance = self.balance.borrow_mut();
            if *balance >= amount {
                *balance -= amount;
                println!("Withdrew ${:.2}, new balance: ${:.2}", amount, *balance);
            } else {
                println!("Insufficient funds. Current balance: ${:.2}", *balance);
            }
        }
    }
    
    let account = BankAccount::new(1000.0);
    let joint_account = Rc::clone(&account);

    account.deposit(500.0);
    joint_account.withdraw(200.0);
    account.withdraw(1500.0);
}
```

This example demonstrates how `Rc<RefCell<T>>` can be used to implement a joint bank account where multiple parties can perform operations on a shared balance.

By combining `Rc` and `RefCell`, Rust provides powerful tools for managing shared, mutable state while maintaining memory safety and preventing data races at runtime.
