fn borrow_ref_to_values(){
    let x = 5;
    let y = &x; // Borrowing a reference to 'x'
    println!("{}",y); // prints value to which y is point to, dereference happens implicitly
    println!("{:p} == {:p}",y,&x); // y and x has exact same address
}


fn borrowing_doesnot_move_ownership(){
    let word = "UTRGV".to_string();

    let borrow_word = &word;

    println!("{} == {}", word, borrow_word);
}

fn if_no_update_borrow_to_read_best_option(){


    fn my_print(word:&String){
        println!("{}", word);
    }
    let word = "UTRGV".to_string();
    my_print(&word);

}

fn borrow_to_mut_watchout(){
    let mut word = "UT".to_string(); 
    // try to delete mut, you are not allowed to obtain mut reference, to immutable value

    fn update(word:&mut String){
        word.push_str("RGV");
    }

    update(&mut word);
    println!("{word}")
}

fn explicit_deref_to_obtain_value(){
    let x: i32 = 5;
    let y = &x;
    let z: &i32 = y;
    println!("{}",z); // notice the fact that you are see 5 it's because a dereference on the fly
    let z: i32 = *y;
    println!("{}",z);
}

#[allow(unused_assignments)]
#[allow(unused_variables)]
fn reference_has_limited_life(){
    let word_reference: &String;

    {
        let word:String = "UTRGV".to_string();
        word_reference = &word;
        // `word` goes out of scope here, and `word_reference` can no longer be used safely
    }
    // println!("{}", word_reference); // This would be unsafe and is prevented by Rust's compiler
}

// RUST BORROWING RULES
#[allow(unused_variables)]
#[allow(unused_mut)]
fn you_cannot_mutate_a_value_if_there_is_a_reference(){
    let mut x = 5;
    let y = &x;
    // x += 1; // This would cause a compile error because `x` is borrowed

}

fn you_can_have_multiple_immutable_references(){
    let x = 5;
    let y = &x;
    let z = &x; // Multiple immutable references are allowed
    println!("{} and {}", y, z);
}

fn only_one_mutable_reference_at_atime(){
    let mut x = 5;
    let y = &mut x; // Single mutable reference
    // let z = &x; // This would cause a compile error
    *y += 1;
    println!("{}", x); // `x` is now `6`
}


#[allow(unused_variables)]
fn you_can_create_immutable_reference_from_mutable_reference(){
    let mut x = 5;
    let mut_ref = &mut x;
    let immut_ref = &*mut_ref; // Creating an immutable reference from a mutable reference
    // let back_to_mut: &mut i32 = &*immut_ref; // This is not allowed

}
fn main(){
    println!("Ownership and borrowing");
    borrow_ref_to_values();
    borrowing_doesnot_move_ownership();
    if_no_update_borrow_to_read_best_option();
    borrow_to_mut_watchout();
    explicit_deref_to_obtain_value();
    reference_has_limited_life();

    // Rust Rules Around Mutation and References. borrow vs borrow_mut
    println!("Get used to this Rules")
    you_cannot_mutate_a_value_if_there_is_a_reference();
    you_can_have_multiple_immutable_references();
    only_one_mutable_reference_at_atime();
    you_can_create_immutable_reference_from_mutable_reference();

}

// In-Class Practice


Warm-up
Problem #1
String Concatenation with Borrowing:

Write a function that concatenates two strings without taking ownership, i.e., by borrowing.

fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}

Problem#2

Clone and Modify:

Given a string, clone it and modify the cloned string by appending a new word. Print both the original string and the cloned, modified string to show that the original has not been changed.

fn clone_and_modify(s: &String) -> String {
    // Your code here
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}

Problem#3

//Write a modified sum function that takes a mutable reference 
//for the destination of the sum from low to high.


#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    0
}

fn main(){
    // create necessary variables and test your function for low 0 high 100 total should be 5050
}

