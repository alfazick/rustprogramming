
fn reference_counting_simple(){
    use std::rc::Rc;
    let num = 10;

    let num_in_heap = Rc::new(num);

    let _copy2_of_num = Rc::clone(&num_in_heap);
    
    let _copy3_of_num = Rc::clone(&num_in_heap);
    
    let _copy4_of_num = Rc::clone(&num_in_heap);

    println!("num in heap has:{} refernces", 
             Rc::strong_count(&num_in_heap));
}


fn sharing_resource_rc_count() {
    use std::rc::Rc;

    struct FamilyMember {
        tv: Rc<TV>,
    }

    struct TV;

    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(TV);

        // curious things, tv_is_on variable, will be dropped, if it was box pointer(TV will be gone as well), but since we wrapping our TV struct with Smart Pointer

        // start watching tv
        FamilyMember {
            tv: Rc::clone(&tv_is_on), 
        }
       // tv object will be dropped, only when there will be no reference to it, or techincally peoples watching it.
    }

    // let's turn on TV 
    let dad = member_start_watch_tv();
    println!("How many people watching {}", Rc::strong_count(&dad.tv));
    // later mom joins
    let mom = FamilyMember { tv: Rc::clone(&dad.tv),}; // share same tv
    println!("How many people watching {}", Rc::strong_count(&dad.tv));  // this produce result 2
    println!("How many people watching {}", Rc::strong_count(&mom.tv)); // this produce result 2 as well.

    let me = FamilyMember { tv: Rc::clone(&dad.tv),};
    println!("How many people watching {}", Rc::strong_count(&me.tv));

    // ok dad stops watching and me stops watching
    drop(dad);
    drop(me);

    println!("How many people watching {}", Rc::strong_count(&mom.tv));

    // so tv will be dropped only the last reference to it be gone

    // And this is a Reference counter smart pointer
    
}


fn ref_cell_simple(){
    use std::cell::RefCell;


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


fn interior_mutability() {
    
    use std::rc::Rc;
    use std::cell::RefCell;

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


fn sharing_resource_refcell_count() {
    use std::rc::Rc;
    use std::cell::RefCell;

    struct FamilyMember {
        tv: Rc<RefCell<TV>>,
    }

    #[derive(Debug)]
    struct TV {
        channel: String,
    }

    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(RefCell::new(TV{channel:"BBC".to_string()}));

        // curious things, tv_is_on variable, will be dropped, if it was box pointer(TV will be gone as well), but since we wrapping our TV struct with Smart Pointer

        // start watching tv
        FamilyMember {
            tv: tv_is_on, 
        }
       // tv object will be dropped, only when there will be no reference to it, or techincally peoples watching it.
    }

    // let's turn on TV 
    let dad = member_start_watch_tv();
    let mom = FamilyMember { tv: Rc::clone(&dad.tv),};
    println!("TV channel for mom {:?}", mom.tv);
    let mut remote_control = dad.tv.borrow_mut();
    println!("TV channel {:?}",remote_control);

    // Due to nature RefCell it forces borrow checker rule
    // let mut remote_control_mom = mom.tv.borrow_mut();
    
    println!("TV channel for mom {:?}", mom.tv);
    remote_control.channel = format!("MTV");
    println!("TV channel {:?}",remote_control);
    drop(remote_control);
    println!("TV channel for mom {:?}", mom.tv);
    
}


fn joint_bank_account_example(){
    use std::rc::Rc;
    use std::cell::RefCell;
    
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
fn main(){
    reference_counting_simple();
    sharing_resource_rc_count();


    ref_cell_simple();
    interior_mutability();

    sharing_resource_refcell_count();
    joint_bank_account_example();


}