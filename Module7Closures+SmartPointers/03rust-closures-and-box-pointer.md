# Rust Closures, Function Pointers, and Box Smart Pointer

This document explores the concepts of closures, function pointers, and the `Box` smart pointer in Rust, blending theory with practical code examples.

## Table of Contents
1. [Closures](#closures)
2. [Function Pointers](#function-pointers)
3. [Box Smart Pointer](#box-smart-pointer)
4. [Closure Traits](#closure-traits)

## Closures

Closures in Rust are anonymous functions that can capture their environment. They are also known as lambda functions in other programming languages.

### Basic Closure Example

```rust
fn essence_example_closure() {
    let x = 21;
    let get_answer = |y: i32| x + y;
    println!("{:?}", get_answer(21));  // Output: 42
}
```

In this example, `get_answer` is a closure that captures the variable `x` from its environment and adds it to the input parameter `y`.

### Closure Syntax Variations

```rust
fn using_function_as_variable() {
    // Regular function
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Function pointer
    let f = add;

    // Closure with explicit types
    let f = |x: i32, y: i32| { x + y };

    // Simplified closure
    let f = |x: i32, y: i32| x + y;

    // Closure with inferred types
    let f = |x, y| x + y;
    
    let result = f(1, 2);
    println!("{}", result);  // Output: 3
}
```

This example demonstrates various ways to define and use closures, showcasing their flexibility and syntax options.

## Function Pointers

Function pointers allow you to pass functions as arguments to other functions, enabling functional programming paradigms in Rust.

```rust
fn using_function_as_parameter() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn calculator(x: i32, y: i32, operation: fn(i32, i32) -> i32) {
        let result = operation(x, y);
        println!("Result of operation: {}", result);    
    }

    calculator(1, 2, add);
    calculator(1, 2, |x, y| x + y);
    calculator(1, 2, |x, y| x - y);
    calculator(1, 2, |x, y| x * y);
}
```

This example shows how to use function pointers and closures as parameters, allowing for flexible and reusable code.

## Box Smart Pointer

The `Box<T>` type is a smart pointer used for heap allocation in Rust. It's particularly useful for creating recursive data structures, implementing trait objects, and managing large data on the heap.

### Basic Box Usage

```rust
fn box_pointer_intro() {
    let box_default = Box::new(100);
    println!("{}", box_default);  // Output: 100
}
```

### Box for Polymorphism

```rust
fn box_polymorphism() {
    use core::fmt::Debug;
    
    trait Animal: Debug {
        fn sound(&self) -> String;
    }
    
    #[derive(Debug)]
    struct Dog;
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            "Woof woof".to_string()
        }
    }
    
    #[derive(Debug)]
    struct Cat;
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            "Meow meow".to_string()
        }
    }
    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    
    zoo.push(Box::new(Dog{}));
    zoo.push(Box::new(Cat{}));
    
    for animal in zoo {
        println!("{:?} says {}", animal, animal.sound());
    }
}
```

This example demonstrates how `Box<dyn Trait>` can be used to achieve runtime polymorphism in Rust.

## Closure Traits

### Closure Traits: Fn, FnMut, and FnOnce

In order to avoid writing full signature when passing a function as a parameter, convenient way is to use a closures.
But, you still need to be more specific, because closures came in different flavors
and Box pointer will help us

Rust has three closure traits: `Fn`, `FnMut`, and `FnOnce`. These traits define how the closure captures and interacts with its environment.

1. `Fn`: The closure captures by reference (`&T`)
2. `FnMut`: The closure captures by mutable reference (`&mut T`)
3. `FnOnce`: The closure captures by value (`T`)

### Closure that Borrows to Read (Fn)
```rust
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
```
### Closure that Borrows to Mutate (FnMut)

```rust
fn capture_modify_environment() {
    let mut result = 0;

    let mut calculator = |x, y| { result = x + y };
    calculator(1, 2);
    println!("{}", result);  // Output: 3
    
    // Using FnMut trait
    let mut calculator: Box<dyn FnMut(i32, i32)> = Box::new(|x, y| { result = x + y });
    calculator(1, 2);
    drop(calculator);
    println!("{}", result);  // Output: 3
}
```

### Closure that Takes Ownership (FnOnce)

```rust
fn capture_ownership_modify() {
    let nums = vec![1, 2, 3, 4, 5].into_iter();
    let product_through_iterator: Box<dyn FnOnce() -> i32> = Box::new(move || nums.product());
    let result: i32 = product_through_iterator();
    println!("{}", result);  // Output: 120
}
```

### Complex Example: Movie Database with Closures

```rust
fn level_up() {
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
        Movie { name: String::from("GodFather"), views: 1000 },
        Movie { name: String::from("Once Upon a Time in America"), views: 1100 },
        Movie { name: String::from("Matrix"), views: 800 },
        Movie { name: String::from("HarryPotter"), views: 900 },
        Movie { name: String::from("Troy"), views: 1200 },
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
```

This advanced example demonstrates how closures and `Box<dyn Trait>` can be used together to create a flexible and powerful movie database with dynamic filtering capabilities.

## Conclusion

Closures, function pointers, and the `Box` smart pointer are powerful features in Rust that enable functional programming paradigms, dynamic dispatch, and heap allocation. Understanding these concepts allows for more flexible and expressive code in Rust applications.
