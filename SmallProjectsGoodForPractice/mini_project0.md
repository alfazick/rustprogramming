Reading a file and printing its contents. 
You are tasked with extending the application to allow users to specify the file to be read as a command-line argument. 

The end result will be a GitHub repository containing the complete code for the enhanced file reader application.

Goal: Simulate Linux:  cat filename 

Learning Objectives:

Understand how to read and process files in Rust using the standard library.

Practice error handling techniques, such as matching on different error cases.

Learn how to organize code and structure a Rust application.

Starter Code


use std::env;

fn main(){
    println!("Mini project: file reader");
    let args: Vec<String> = env::args().collect();

    for arg in args{
        println!("Argument: {}", arg);
    }
}


fn file_reader_handle_error(){
    let filename = "message.txt";
    let file = File::open(filename);

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening the file: {}", error)
                }
            }
        }
    };


    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

}

