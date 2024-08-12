//Converter from command line different sizes
// Refactor

use std::env;

enum FileSize{
    BYTE(u64),
    KB(u64),
    MB(u64),
    GB(u64),
    WrongFormat,
}


#[derive(Debug)]
struct Formatter {
    bytes:u64,
    kilobytes:u64,
    megabytes:u64,
    gigabytes:u64,
}

impl Formatter {

    fn converter(num:u64,need:FileSize,direction:bool) -> u64{
        let degree = match need {
            FileSize::BYTE(d) => d,
            FileSize::KB(d) => d,
            FileSize::GB(d) => d,
            FileSize::MB(d) => d,
            FileSize::WrongFormat => panic!("wrong format") 
        };

        match direction {
            true => num*degree,
            false => num/degree,
        }

        
    }

    fn new(num:u64,original:&str) -> Formatter {
        let original = FileSize::new(original);

        let original_bytes = Formatter::converter(num, original, true);

        
        Formatter{
            bytes: original_bytes,
            kilobytes: Formatter::converter(original_bytes,FileSize::new("kb"), false),
            megabytes: Formatter::converter(original_bytes,FileSize::new("mb"), false),
            gigabytes:Formatter::converter(original_bytes,FileSize::new("gb"), false),
        }
    }

}




impl FileSize {
    
    fn new(sizeacronym:&str) -> Self{
        match sizeacronym {
            "bytes" => FileSize::BYTE(1),
            "kb" => FileSize::KB(1_000),
            "mb" => FileSize::MB(1_000_000),
            "gb" => FileSize::GB(1_000_000_000),
            _ => FileSize::WrongFormat,
                    
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        println!("Usage is cargo run [filesize]");
        return;
        
    }

    let file_size:u64 = args[1].clone().parse().unwrap();
    let size_type = args[2].clone();
    let formatter = Formatter::new(file_size,size_type.as_str());
    

    println!("Formatter {:?}",formatter);
    
    
    
}
