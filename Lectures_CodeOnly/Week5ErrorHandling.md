// Week5: Error Handling

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn panic_examples() {
    println!("Everything is good");
    // panic!("Crash the program, stop running, clean the memory");
    // println!("This won't be printed.");
    // let v = vec![1, 2, 3];
    // println!("{:?}", v[99]); // This will cause a panic because the index is out of bounds
}

fn open_file_with_match() {
    let f = File::open("exam.txt");
    println!("{:?}", f);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("exam.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

fn open_file_shortcuts() {
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2ver() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_3ver() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_4ver() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    panic_examples();
    open_file_with_match();
    open_file_shortcuts();
    read_username_from_file();
    read_username_from_file_2ver();
    read_username_from_file_3ver();
    read_username_from_file_4ver();
}
