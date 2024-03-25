//Closures 

fn essence_example_closure() {
    // Our goal for this week is to understand how it works
    let x = 21;

    let get_answer = |y: i32| x + y;// this is a closure an anonymous(lambda) function which captures the environment

    println!("{:?}", get_answer(21));  
}


#[allow(unused_variables)]
fn using_function_as_variable() {
    // Regular function declaration
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let f = add; // we are creating a function pointer

    let f = |x: i32,y: i32| {x+y}; // same as function add with closure syntax and it's anonymous function

    let f = |x:i32,y:i32| x+y; // simplified closure
    //because it's a single line expression, we can skip curly brackets

    let f = |x,y| x+y; // closure with inferred parameter types
    
    let result = f(1,2);
    println!("{}", result);
}


fn using_function_as_parameter() {
    // in general, using function as a parameter to another function is a paradigm of functional programming
    fn add(x: i32, y:i32) -> i32 {
        x + y
    }

    fn calculator(x:i32,y:i32, operation: fn(i32,i32) -> i32) {
        let result = operation(x,y);
        println!("result of operation {}", result);    
    }

    calculator(1,2,add);
    // or 
    calculator(1,2,|x,y| x + y);
    // or easy to add functionality on the fly
    // one of the reason why closures is widely used
    calculator(1,2,|x,y| x - y);
    calculator(1,2,|x,y| x * y);

    // though what is important to note it's not a full closure because it doesn't capture the environment
    // and fn(i32,i32) is indication of it. 
}

// So functional programming starts, when you use functions to pass as a parameter
// to another function.


fn box_pointer_intro(){
    // Simplest form of pointers
    // Goal: explicitly allocate data into the heap

    // Note: Box often holds Option<your data>, to make sure that data will not hold unchecked null(None)

    // Box and Option recommended to use together so you don't have runtime problems(null pointer exception)

    // Though there is another potential problem: If OS will not allocate memory for your Box

    // You can encounter creating a Box with try_new() method, which return Result enum. (nightly feature, not yet fully supported)
    //https://doc.rust-lang.org/std/boxed/struct.Box.html

    
    // Default is new() that will panic if allocation fails.

    let box_default = Box::new(100);
    println!("{}",box_default);
    
}


fn box_polymorphism(){
    // on the usage of box smart pointer, it allows you to achieve
    // runtime polymorphsim
    use core::fmt::Debug;
    
    trait Animal: Debug {
        fn sound(&self) -> String;
    }
    
    #[derive(Debug)]
    struct Dog;
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            "Wof wof".to_string()
        }
    }
    
    #[derive(Debug)]
    struct Cat;
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            "Mur mur".to_string()
        }
    }
    
    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    //let mut zoo =  Vec::new(); try to uncomment
    
    zoo.push(Box::new(Dog{}));
    zoo.push(Box::new(Cat{}));
    
    
    for animal in zoo {
        println!("{:?} says {}",animal,animal.sound());
    }
    
}


// In order to avoid writing full signature when passing a function as a parameter, convenient way is to use a closures.
// But, you still need to be more specific, because closures came in different flavors
// And Box pointers will help us

fn using_closure_as_parameter_and_capture_environment() {
    
    fn add(x: i32, y:i32) -> i32 {
        x + y
    }

    // story here changes dramatically.
    // Fn is a trait, which is needed to be dispatched at the runtime
    // Box puts that function into heap
    fn calculator(x:i32,y:i32, operation: Box<dyn Fn(i32,i32) -> i32 + '_>) {
        let result = operation(x,y);
        println!("result of operation {}", result);    
    }

    calculator(1,2,Box::new(add)); 
    calculator(1,2,Box::new(|x,y| x + y)); // works as expected

    let z = 3;
    calculator(1,2,Box::new(|x,y| x + y + z)); 
    // z is an unexpected guess in our closure, because it's not passed,
    // between pipes, but due to the nature of closures which captures the environment I can stil access it and need to make sure to incdicate it's lifetime.

}


fn capture_modify_environment() {
    let mut result = 0;

    let mut calculator = |x,y| {result = x + y};
    calculator(1,2);
    println!("{}", result);
    
    
    // Fn Mut trait
    // techincally borrows mutable reference
    let mut calculator: Box<dyn FnMut(i32,i32)> = Box::new(|x,y|{result = x + y});

    calculator(1,2);
    drop(calculator); // try to comment, you can't use result because it was dragged inside of closure
    println!("{}", result);
}


#[allow(dead_code)]
fn trait_fnmut_again() {
    // sorting good example, it mutates internal state of captured values, 
    // but don't move them out
    
    #[derive(Debug)]
    struct Student {
        name: String,
        grescore: u16,
    }
    
    let mut students = vec![
        Student {name:"John".to_string(),grescore:315},
        Student {name:"Alice".to_string(),grescore:320},
        Student {name:"Bob".to_string(),grescore:315},
    ];
    
    println!("{:#?}", students);
    students.sort_by_key(|s| s.grescore);
    println!("{:#?}", students);
    
    students.sort_by_key(|s| (s.grescore,s.name.clone()));
    println!("{:#?}", students);
    
    
        
    }


fn capture_ownership_modify() {
    let nums = vec![1,2,3,4,5].into_iter();
    let product_through_iterator = move|| nums.product();
    let result:i32 = product_through_iterator();
    println!("{}", result);


    // same but with annotation
    // technically FnOnce takes ownership
    // and we can call it only once

    let nums = vec![1,2,3,4,5].into_iter();
    let product_through_iterator: Box<dyn FnOnce() -> i32> = Box::new( move || nums.product());
    let result:i32 = product_through_iterator();
    println!("{}", result);
    
}


fn level_up(){
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct Movie {
        name: String,
        views: i64,
    }
    
    struct MovieDatabase {
        movies: Vec<Movie>,
        filter_by: Box<dyn Fn(&Movie) -> bool>,
    }
    
    impl MovieDatabase {
        fn get_movies(&self) -> Vec<Movie> {
            let mut list_to_return = self.movies.clone();
            list_to_return.retain(|movie| (self.filter_by)(movie));
            list_to_return
        }
    
        fn update_filter(&mut self, new_filter: Box<dyn Fn(&Movie) -> bool>) {
            self.filter_by = new_filter;
        }
    }


    let lst = vec![
        Movie {
            name: String::from("GodFather"),
            views: 1000,
        },
        Movie {
            name: String::from("Once Upon a Time in America"),
            views: 1100,
        },
        Movie {
            name: String::from("Matrix"),
            views: 800,
        },
        Movie {
            name: String::from("HarryPotter"),
            views: 900,
        },
        Movie {
            name: String::from("Troy"),
            views: 1200,
        },
    ];

    let mut movie_db = MovieDatabase {
        movies: lst,
        filter_by: Box::new(|m| m.views > 900),
    };

    let list_popular = movie_db.get_movies();
    println!("Popular movies: {:?}", list_popular);

    movie_db.update_filter(Box::new(|m| m.name.len() > 10));

    let list_long_titles = movie_db.get_movies();
    println!("Movies with long titles: {:?}", list_long_titles);

}

    
fn main() {
    essence_example_closure();
    using_function_as_variable();
    using_function_as_parameter();
    box_pointer_intro();
    box_polymorphism();
    using_closure_as_parameter_and_capture_environment();
    capture_modify_environment();
    trait_fnmut_again();
    capture_ownership_modify();
    level_up();

}
    