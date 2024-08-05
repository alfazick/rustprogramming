# Integrated Guide to Rust Traits and Generics


## 1. Introduction to the Idea

```rust
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
```

Explanation:
This function introduces the basic problem that traits and generics solve in Rust:

1. It defines two structs, `Rectangle` and `Circle`, each with their own `get_area` method.
   - `Rectangle` has `width` and `height` fields, and its area is calculated as `width * height`.
   - `Circle` has a `radius` field, and its area is calculated as `π * radius^2` (using 3.14 as an approximation for π).

2. The function creates instances of both structs and calls their respective `get_area` methods.

3. The commented-out line `let shapes: Vec<????> = vec![rec, circle];` highlights a key limitation:
   - We can't easily group these different types together in a collection, even though they both have a `get_area` method.
   - This is because `Rectangle` and `Circle` are different types, and Rust requires vectors to contain elements of the same type.

Key takeaway: Without traits, it's difficult to work with different types that share similar behavior, even if that behavior (like calculating area) is conceptually the same. This sets up the motivation for using traits and generics in Rust.

## 2. Same Method, Same Logical Entity

```rust
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
```

Explanation:
This function introduces traits as a solution to the problem presented in the previous function:

1. It defines an `AreaInfo` trait with a single method `get_area`. This trait represents the common behavior of shapes that can calculate their area.

2. The `Rectangle` and `Circle` structs are defined as before, but now they both implement the `AreaInfo` trait:
   - Each struct provides its own implementation of the `get_area` method, specific to how that shape's area is calculated.

3. The function creates instances of `Rectangle` and `Circle` and calls their `get_area` methods, demonstrating that the original functionality is preserved.

4. The key improvement is demonstrated in the line:
   ```rust
   let shapes: Vec<&dyn AreaInfo> = vec![&rec, &circle];
   ```
   - This creates a vector of trait objects. `&dyn AreaInfo` is a reference to any type that implements the `AreaInfo` trait.
   - This allows us to store references to different types in the same collection, as long as they all implement the `AreaInfo` trait.

5. The `for` loop at the end shows how we can iterate over this collection and call the `get_area` method on each shape, regardless of its concrete type.

Key takeaways:
- Traits allow us to define common behavior for different types, enabling polymorphism.
- We can use trait objects (`&dyn Trait`) to work with different types through a common interface.
- This approach solves the problem of grouping different types with similar behavior, which we couldn't do in the previous example.

## 3. Benefits of Logical Entity

```rust
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
```

Explanation:
This function expands on the uses of traits and introduces trait bounds:

1. It defines a `Summary` trait with a `summarize` method. This trait is marked as `pub`, allowing other modules to implement it.

2. Two structs are defined: `NewsArticle` and `Tweet`, each with different fields relevant to their type.

3. Both `NewsArticle` and `Tweet` implement the `Summary` trait, providing their own specific implementations of the `summarize` method.

4. Instances of `Tweet` and `NewsArticle` are created, and their `summarize` methods are called, demonstrating how different types can implement the same trait with different behavior.

5. Two functions are defined to show different ways of using traits as parameters:
   - `notify_sugar_syntax(item: impl Summary)`: This syntax is more concise and uses the `impl Trait` syntax.
   - `notify_real_syntax<T: Summary>(item: T)`: This syntax uses generics with a trait bound.

   Both functions can accept any type that implements the `Summary` trait.

6. The functions are called with `tw` (Tweet) and `article` (NewsArticle) respectively, demonstrating how they can work with different types as long as they implement the `Summary` trait.

Key takeaways:
- Traits enable writing functions that can work with any type implementing a specific behavior, promoting code reuse and abstraction.
- The `impl Trait` syntax provides a concise way to specify trait bounds on function parameters.
- Generic functions with trait bounds offer more flexibility, especially when dealing with multiple or complex trait bounds.
- This approach encourages "programming to behavior" rather than specific types, enhancing code flexibility and reusability.

## 4. Implementing Traits on Native Data Types

```rust
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
```

Explanation:
This function demonstrates that traits can be implemented for existing types, including built-in types:

1. It defines a `Double` trait with a single method `double` that returns `Self` (the type implementing the trait).

2. The `Double` trait is implemented for two different built-in types:
   - For `i32` (32-bit integer): The `double` method multiplies the number by 2.
   - For `String`: The `double` method concatenates the string with itself, separated by a colon.

3. The function then demonstrates the use of these trait implementations:
   - It calls `double()` on an `i32` value (5), which returns 10.
   - It calls `double()` on a `String` value ("hello"), which returns "hello:hello".

Key takeaways:
- Traits can be implemented for types that you didn't define, including standard library types.
- This feature allows you to extend the functionality of existing types without modifying their original definitions.
- It demonstrates Rust's ability to add behavior to types after they've been defined, which is a form of the "extension methods" pattern seen in some other languages.
- This capability is particularly useful when you want to make existing types conform to a new trait that you've defined for use in your own generic functions or structs.

## 5. Solving the Last Lecture Problem

```rust
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
```

Explanation:
This function demonstrates more advanced use of generics and trait bounds:

1. It defines two generic functions, `largest` and `largest_2`, that find the largest element in a slice of any type `T` that implements both `PartialOrd` and `Copy` traits.

2. The `largest` function uses the syntax `fn largest<T: PartialOrd + Copy>(list: &[T]) -> T`:
   - `T: PartialOrd` ensures that the elements can be compared with each other.
   - `T: Copy` allows the function to copy elements (necessary for the assignment `largest = item`).

3. The `largest_2` function uses a `where` clause to specify the same trait bounds:
   ```rust
   fn largest_2<T>(list: &[T]) -> T 
       where T: PartialOrd + Copy
   ```
   This syntax is equivalent to the first function but can be cleaner when dealing with multiple or complex trait bounds.

4. Both functions work the same way:
   - They initialize `largest` with the first element of the list.
   - They iterate through the list, updating `largest` if a larger element is found.

5. The function demonstrates the use of these generic functions with both a list of integers and a list of characters, showing their flexibility.

Key takeaways:
- Generic functions with trait bounds allow us to write flexible, reusable code while still maintaining type safety.
- The `PartialOrd` trait is used to ensure the elements can be compared.
- The `Copy` trait is used to allow simple copying of values, which is necessary for the assignment in this implementation.
- The `where` clause offers a cleaner syntax for specifying multiple or complex trait bounds, improving readability.
- These functions work with any type that implements `PartialOrd` and `Copy`, demonstrating the power of generics and trait bounds in creating highly reusable code.

## Main Function

```rust
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
```

Explanation:
The `main` function serves as the entry point of the program and orchestrates the execution of all the example functions we've discussed:

1. It calls `intro_to_idea()` to demonstrate the initial problem that traits and generics solve.

2. It then calls `same_method_same_logical_entity()` to show how traits can be used to solve this problem.

3. `benefits_of_logical_entity()` is called to further explore the advantages of using traits, particularly in function parameters.

4. `implemeting_traits_on_native_data_type()` is executed to show how traits can be implemented for existing types.

5. Finally, `last_lecture_problem_fixing()` is called to demonstrate more advanced uses of generics and trait bounds.

This progression allows for a step-by-step exploration of traits and generics in Rust, building from basic