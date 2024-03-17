
fn intro_to_idea(){
    
    pub struct Rectangle{
        pub width: f64,
        pub height: f64,
     }

    impl Rectangle {
         fn get_area(&self) -> f64 {
             self.width * self.height
         }
    }


    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
         fn get_area(&self) -> f64 {
             self.radius * self.radius * 3.14 as f64
         }
     }

     let rec = Rectangle {width:5.0,height:8.0};
     let circle = Circle {radius: 5.0};

     println!("Area of a rectangle {}", rec.get_area());
     println!("Area of a circle {}", circle.get_area());

    // let shapes: Vec<????> = vec![rec, circle]; 
    // unfortunately doesn't work
}

fn same_method_same_logical_entity(){

    // this is a big idea.
    // bind different data types with same behaviour
    // as one logical unit
    pub trait AreaInfo {
        fn get_area(&self) -> f64;
    }
    

    pub struct Rectangle{
        pub width: f64,
        pub height: f64,
    }

    impl AreaInfo for Rectangle {
        fn get_area(&self) -> f64 {
            self.width * self.height
        }
    }


    pub struct Circle {
        pub radius: f64,
    }

    impl AreaInfo for Circle {
        fn get_area(&self) -> f64 {
            self.radius * self.radius * 3.14 as f64
        }
    }

    // You could say, it's almost the same, well what is same for you is not the same for the compiler.

    // Make sure nothing is broken

    let rec = Rectangle {width:5.0,height:8.0};
    let circle = Circle {radius: 5.0};

    println!("Area of a rectangle {}", rec.get_area());
    println!("Area of a circle {}", circle.get_area());

    // And now the Magic or cheating, I don't know how to call it
    
    let shapes: Vec<&dyn AreaInfo> = vec![&rec,&circle];

    // dyn -> dynamic keyword 
    // https://doc.rust-lang.org/std/keyword.dyn.html

    // Dynamically dispatch the type of object
    for shape in shapes.iter() {
        println!("{}", shape.get_area());
    }
}

fn benefits_of_logical_entity(){
        
    pub trait Summary { // Trait should be public if we want to allow others to implement it
        fn summarize(&self) -> String; // no body just declaration like interface
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle { 
        fn summarize(&self) -> String { 
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    let tw = Tweet {
             username: String::from("Elon"),
             content: String::from("to the Moon"),
             reply: false,
             retweet: false,
         };
    println!("{}",tw.summarize());
        
    let article = NewsArticle {
             headline: String::from("Elon sells Bitcoin"),
             location: String::from("Menlo Park, CA, USA"),
             author: String::from("CNN"),
             content: String::from("FBI investigates"),
         };
    
    println!("{}", article.summarize());

    // Real reason we need to use traits or interfaces
    // Change you coding paradigm, start to programm to behavior!

    pub fn notify_sugar_syntax(item: impl Summary) { // your function will accept any parameter that implements Summary trait. (so all parameters will have the common behavior)
        println!("Breaking news! {}", item.summarize());
    }

    // Same logic as above but explicit, this is refereed to the idea TRAIT BOUNDS
    // or in simple language, sometimes you want to accept parameters, which implement more than one trait(have more than one common method to call on it)
    
    pub fn notify_real_syntax<T: Summary>(item: T){ // please notice generics you are saying. My function will accept as a parameter a generic type T which implements Summary trait, because I just want to make sure that I can call the methods safely.
        
        println!("Breaking news! {}", item.summarize());
    }


    notify_real_syntax(tw);
    notify_sugar_syntax(article);

}


fn implemeting_traits_on_native_data_type() {
    // every Rust data type is a struct technically,
    // which means you can implement your own trait on any of them

    trait Double {
        fn double(&self) -> Self;
    }

    impl Double for i32 {
        fn double(&self) -> Self {
            self * 2
        }
    }

    impl Double for String {
        fn double(&self) -> Self {
            format!("{}:{}",&self,&self)
        }
    }

    println!("double 5_i32 == {}", 5_i32.double());
    println!("double hello == {}", "hello".to_string().double());
}



fn last_lecture_problem_fixing(){
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { 
        let mut largest = list[0]; // we need Copy trait to achieve that operation
        
        for &item in list.iter() {
            if item > largest { // we need PartialOrd trait to be able to compare elements
                largest = item;
            }
        }
        largest
    }

    // where clause
    fn largest_2<T>(list: &[T]) -> T 
        where T: PartialOrd + Copy,
        {
        let mut largest = list[0]; // we need Copy trait to achieve that operation
        
        for &item in list.iter() {
            if item > largest { // we need PartialOrd trait to be able to compare elements
                largest = item;
            }
        }
        largest
    }


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_2(&char_list);
    println!("The largest char is {}", result);
}


fn main(){

    // Main Motivation
    intro_to_idea();
    same_method_same_logical_entity();

    // Same idea, but with different conclusion
    benefits_of_logical_entity();

    implemeting_traits_on_native_data_type();

    // If more than one trait use where clause
    last_lecture_problem_fixing();

}