
fn variable_assignment_immutability(){
    //Point#1 Variable assignments
    let msg = "Hello World";
    let msg1 = "Hello World".to_string();
    //let num = 10 as i32;
    let num = 8;
    let num1 = 10i16;
    

    // problem let result = num / num1 ;

    println!("{} : {}",msg,msg1);
    println!("{} := {}",num,num1);

    // Point#2 "by default everything is immutable, 
    // indicate with mut that you will change"
    // let num:i32 = 100;
    // this will not work num += 100;

    let mut num:i32 = 100;
    num += 100;

    println!("{}",num);

}

fn basic_control_flow(){
    let status = 500;

    if status == 200 {
        println!("Status code: OK");
    } else if status == 500 {
        println!("Status code: Internal Server Error");
    } 
    else {
        println!("Page not found");
    }
}

fn concept_of_shadowing(){
    let num = 10;
    let num = num + 10; // like breaking mutability, but NO
    println!("{}",num);


    let num = 10;
    {
        let num = num + 10;
        println!("{}",num);
    }
    println!("{}",num);

}

fn concept_of_shadowing_mixed_up(){
    // sometimes you will see next usage pattern
    fn get_score() -> i32{
        return 100;
    }

    let result = get_score();
    print!("Student scored: {}. ",result);
    let result = result > 90;
    println!("IsPassed: {}",result);
    

}

fn everything_is_expression(){
    fn get_num(x:i32) -> & 'static str{
        
        if x == 25{
            return "Quarter";
        } else if x == 10 {
            return "Dime";
        } else if x == 5 {
            return "Nickel";
        }  else {
            return "Penny";
        }
    }

    let coin_type = get_num(10);
    println!("{}",coin_type);
    // alternative will may look like next, notice lack of semicolons
    // this is a feature of expressed based languages
    // evaluation first, then assignments
    let coin_type = {
        let x = 5;
        if x == 25{
            "Quarter"
        } else if x == 10 {
            "Dime"
        } else if x == 5 {
            "Nickel"
        }  else {
            "Penny"
        }
    };

    println!("{}",coin_type);


}

fn infinite_loop(){
    let mut x = 0;
    loop { // alternative while true
        println!("{}",x);
        x +=1;

        if x == 5{ 
            break;
        }
    }
    println!("{}",x);
}

fn regular_while(){
    let mut x= 0;
    while x < 5{
        println!("{}",x);
        x +=1;
    }
}




fn for_loops_world(){
    for i in 1..5{
        if i == 3 {
            continue;
        }
        println!("{}",i);
    }

    for i in (1..=5).rev() {
        println!("{}",i);
    }
    
    //but be careful
    let nums = vec![1,2,3,4,5];
    let mut total = 0;
    for i in 0..5{ // pay attention to data type
        println!("{}",nums[i]);
        total += i as i32;
    }
    println!("{}",total);
    

}

fn pattern_match_simple(){
    let num = 3;

    let letter = match num {
        1 => 'A',
        2 => 'B',
        3 => {
            (64 + 1+2 as u8) as char
        },
        _ => '#', // rust will not guess
    };

    println!("{}",letter);
}



fn unit_function(){

    fn greet(word :&str) -> () { // () indicates, no return can be omitted techincally Unit function
        println!("Hello, {}",word)
    }

    greet("John");

}

fn return_function(){
    fn compose(word1:&str,word2:&str) -> String{
        format!("{} {}",word1,word2)
    }

    let concat = compose("Hello", "World");
    println!("{}",concat);
}



fn main() {
    println!("Basics Ideas,Which May Look Weird");
    variable_assignment_immutability();
    basic_control_flow();
    concept_of_shadowing();
    concept_of_shadowing_mixed_up();
    everything_is_expression();
    println!("Loops and Controls");
    infinite_loop();
    regular_while();
    for_loops_world();
    pattern_match_simple();
    println!("Simple functions world");
    unit_function();
    return_function();
}
