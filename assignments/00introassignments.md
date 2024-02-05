Ok it's time for coding assignments.

For this class, we will just use built-in automated testing capabilties in Rust

I assume you hvae Rust installed, either in cloud or local machine
If you have not done it yet please visit

https://www.rust-lang.org/tools/install


Grading will be done automatically.

General rules: Your code should compile,otherwise your grade is 0
For each passed tests you will be given points, depending on problem complexity.



Your goal is to work in main.rs file only.
Tests will be located in implement.rs file. Don't touch it. Your submission will be executed against a copy of this file.

In main.rs file, you should fill out your full_name and class variables.

Let's look at simple example of how test capabilities work in rust.

'''
pub fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn main(){
    println!("Test");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(add(2,3), 5);
        
    }
}
'''

In rust project folder, execute command

cargo test

To learn your full score, you can run in Linux



cargo test 2>&1 | grep "test result" | awk '{print $4, "tests passed. Score:", $4*20 "/100"}'

but overall you can just manually add your points


Ok. so now close to real:


main.rs file is just you can manually test and print that you are getting what you are expecting

main.rs 
'''
mod implement;

fn main(){
    println!("Test");
    let result = implement::add(10,20);
    println!("{}",result);

}
'''

implement.rs
'''
pub fn add(a:i32, b:i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(add(2,3), 5);
        
    }
}
'''

execute cargo run and cargo test









