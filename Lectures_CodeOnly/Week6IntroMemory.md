
// Safe memory manipulation
// Rust std::mem 

fn mem_std_replace(){
    // Replaces the value at a mutable location with a new value, 
    // returning the old value. This function is particularly useful 
    // for ownership transfer without violating Rust's borrowing rules.
    use std::mem;

    let mut some_string = String::from("Hello, Rust");
    let new_string = String::from("Goodbye, Rust!");

    // Replace 'some_string' with 'new_string', returning the old value of 'some_string'

    let old_string = mem::replace(&mut some_string, new_string);

    println!("{}",some_string);
    // println!("{}",new_string); // caveat is new_string ownership is moved

    

    assert_eq!(old_string, "Hello, Rust");
    assert_eq!(some_string, "Goodbye, Rust!");


}


fn mem_std_swap(){
    // Swaps the values of two mutable references.
    use std::mem;

    let mut x = 5;
    let mut y = 10;

    // Swap the values of x and y
    mem::swap(&mut x, &mut y);

    assert_eq!(x, 10);
    assert_eq!(y, 5);
}

fn mem_std_take(){
    //Replaces the value at a mutable location with the default value for that type, 
    //returning the old value.

    use std::mem;
    let mut option = Some("Rust");

    // Take the value out of 'option', leaving 'None' in its place
    let taken_value = mem::take(&mut option);

    println!("{:?} {:?}",option,taken_value);

    assert_eq!(taken_value, Some("Rust"));
    assert_eq!(option, None); 
}


fn main(){
   mem_std_replace();
   mem_std_swap();
   mem_std_take();

}

