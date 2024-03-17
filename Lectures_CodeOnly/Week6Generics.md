fn motivation_example() {
    // Problem find the largest element
    // Just to realize, a problem

    
    // For integers 32, 
    fn largest_int(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    //Most straighforward function, so far so good.
    // But. it will not work for f32 or char.

    // Well, first solution which cames to mind of course is copy and paste

    // For floats 32, 
    fn largest_float(list: &[f32]) -> f32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // For char, 
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    println!("{}",largest_int(&[1,2,5]));
    println!("{}",largest_float(&[1.5,2.6,5.9]));
    println!("{}",largest_char(&['A','B','C']));
}


fn broken_example(){
    // Generic in functions
    // Often used case scenario, example not working
    // but just to give you general idea how it may look
    // with next week knowledge we will make it work
    // For curious students: there are two problems
    // 1) When you are using generic data type, question is how long that data will live
    // 2) What kind of operation this data type is suppose to support.
    


    //fn largest<T>(list: &[T]) -> T { 
    //     // after name of code entity in our case function, in <your name>, you are announcing, your universal data type
    //     // and use it everwhere, where you would normally specify concrete data type
        
    //     let mut largest = list[0];
    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
     //}
}

#[allow(dead_code)]
fn generics_in_struct() {
    // struct is responsible for keeping data,so second problem, can be potentially be skipped

    #[derive(Debug)]
    struct Point<T> { // in <T> I am declaring that I am gonna use type T in my newly defined struct Point, compiler please compile for me all necessary boilerplate code, for whatever data type I will happen to be use in my programm development.
        
        x: T, // Notice, if I use T type for both fields, both of them will be the same type
        y: T,
    }

    let integer = Point{x:5,y:10};
    let float = Point{x:1.0,y:4.0};

    println!("int Point: {:?} float Point: {:?}",integer, float);


    // And of course there is no limitation for announcing generic types, but more that 2, it's weird : )
    
    #[derive(Debug)]
    struct User<T, U> {
        name: T,
        y: U,
    }

    
    let user1 = User{name: "Vandam",y:35};
    let user2 = User{name:"James Bond".to_string(),y:"===> 007"};

    // I know it's just a horrible example, just wanted to show you flexibility of generics, 
    // there is no limit, basically especially for data types that just holds data
    println!("User1: {:?} User2: {:?}",user1, user2);
}

fn generics_method_definitions(){
    // Again absurd example,but absurd examples, usually helps us to get the idea faster
    struct File<T> {
        name: String,
        data: T,
    }

    impl<T> File<T> { // after keword impl again I need to specify, that I am gonna use T as a placeholder for any datatype I am happen to use
        
        fn new(name: &str, content:T) -> File<T> {
            File { name: String::from(name), data: content }
        }
    }

    let textfile = File::new("lets'go", vec!["K'Maro".to_string()]);
    let imagefile = File::new("MonaLisa",vec![0,123,255]);

    println!("Textfile name {:?}. Textfile content {:?}",textfile.name, textfile.data);
    println!("Imagefile name {:?}. Imagefile content {:?}",imagefile.name, imagefile.data);
}

fn classic_example_stack(){
    // Among all coding examples, this one make a lot of sense
    // so that's why the last

    #[derive(Debug)]
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Stack<T> {
            Stack { items: Vec::new() }
        }
        fn push(&mut self, item: T) {
            self.items.push(item);
        }

        fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }
    }

    let mut stack = Stack::<i32>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("My stack holds{:?}",stack);
    stack.pop();
    println!("My stack holds{:?}",stack);
    

    
}

// Extra: you can came across TurboFish Notation

// Notice usage of generics

#[allow(unused_variables)]
fn turbofish() {
    
    #[derive(Debug)]
    struct Pet<T> {
        cats: T,
        dogs: T,
    }
    
    
    // impl Pet<i32> {
    //     // notice new() is a static method
    //     fn new() -> Self {
    //         Pet {
    //             cats: 5,
    //             dogs: 10,
    //         }
    //     }
    // }

    // so here a couple problems, by naive assumption
    // we could continue like, but it defeats the whole purpose of generics
    // what to do?

    // impl Pet<i64> {
    //     // notice new() is a static method
    //     fn new() -> Self {
    //         Pet {
    //             cats: 5,
    //             dogs: 10,
    //         }
    //     }
    // }

    // let pets = Pet::new();
    // println!("{:?}",pets);

    impl<T> Pet<T> {
        // notice new() is a static method
        fn new(a:T,b:T) -> Self {
            Pet {
                cats: a,
                dogs: b,
            }
        }
    }
    
    let pets = Pet::<i64>::new(5_i64,10_i64);
    println!("Cats: {}, Dogs: {}",pets.cats,pets.dogs);

    let pets = Pet::<String>::new("Million".into(),"Billion".into());
    println!("Cats: {}, Dogs: {}",pets.cats,pets.dogs);
    
}


fn main(){
    motivation_example();

    broken_example();
    generics_in_struct();
    
    generics_method_definitions();

    classic_example_stack();

    turbofish();
}
