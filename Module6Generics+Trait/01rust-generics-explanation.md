# Rust Generics: Explanation and Examples

## Introduction

This guide explores the concept of generics in Rust, using a series of examples to illustrate their use and benefits. We'll walk through the provided code, explaining each part and discussing the motivations behind using generics.

## Motivation Example

Let's start by examining a situation where generics can solve a common problem:

```rust
fn motivation_example() {
    // Problem: find the largest element
    // For integers 32
    fn largest_int(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // For floats 32
    fn largest_float(list: &[f32]) -> f32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // For char
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    println!("{}", largest_int(&[1, 2, 5]));
    println!("{}", largest_float(&[1.5, 2.6, 5.9]));
    println!("{}", largest_char(&['A', 'B', 'C']));
}
```

In this example, we have three nearly identical functions that find the largest element in a list of integers, floats, and characters, respectively. This approach leads to code duplication, which is not ideal for maintenance and scalability.

## Broken Example: Introducing Generics

The next example introduces the concept of generics, but intentionally includes some issues to highlight important aspects of using generics in Rust:

```rust
fn broken_example() {
    // Generic in functions
    // Often used case scenario, example not working
    // but just to give you general idea how it may look
    // with next week knowledge we will make it work
    // For curious students: there are two problems
    // 1) When you are using generic data type, question is how long that data will live
    // 2) What kind of operation this data type is supposed to support

    //fn largest<T>(list: &[T]) -> T { 
    //     let mut largest = list[0];
    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    //}
}
```

This example introduces the syntax for generic functions in Rust. The `<T>` after the function name declares a generic type parameter. However, this code doesn't compile due to two main issues:

1. Lifetime specifiers: Rust needs to know how long the generic data will live.
2. Trait bounds: We need to specify what operations the generic type supports (in this case, comparison).

To fix this, we would need to add appropriate trait bounds and possibly lifetime specifiers.

## Generics in Structs

Generics can also be used in struct definitions:

```rust
#[allow(dead_code)]
fn generics_in_struct() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("int Point: {:?} float Point: {:?}", integer, float);

    #[derive(Debug)]
    struct User<T, U> {
        name: T,
        y: U,
    }

    let user1 = User { name: "Vandam", y: 35 };
    let user2 = User { name: "James Bond".to_string(), y: "===> 007" };

    println!("User1: {:?} User2: {:?}", user1, user2);
}
```

This example demonstrates how to use generics in struct definitions. The `Point<T>` struct uses a single generic type for both fields, while the `User<T, U>` struct uses two different generic types.

## Generics in Method Definitions

Generics can also be used in method definitions:

```rust
fn generics_method_definitions() {
    struct File<T> {
        name: String,
        data: T,
    }

    impl<T> File<T> {
        fn new(name: &str, content: T) -> File<T> {
            File { name: String::from(name), data: content }
        }
    }

    let textfile = File::new("lets'go", vec!["K'Maro".to_string()]);
    let imagefile = File::new("MonaLisa", vec![0, 123, 255]);

    println!("Textfile name {:?}. Textfile content {:?}", textfile.name, textfile.data);
    println!("Imagefile name {:?}. Imagefile content {:?}", imagefile.name, imagefile.data);
}
```

This example shows how to implement methods on a generic struct. The `impl<T> File<T>` syntax allows us to define methods that work with any type `T`.

## Classic Example: Stack

A common use case for generics is in implementing data structures. Here's an example of a generic stack:

```rust
fn classic_example_stack() {
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

    println!("My stack holds {:?}", stack);
    stack.pop();
    println!("My stack holds {:?}", stack);
}
```

This example demonstrates a generic stack implementation. The stack can hold elements of any type, showcasing the flexibility of generics.

## Turbofish Notation

The turbofish notation is a unique Rust feature for specifying generic type parameters:

```rust
#[allow(unused_variables)]
fn turbofish() {
    #[derive(Debug)]
    struct Pet<T> {
        cats: T,
        dogs: T,
    }

    impl<T> Pet<T> {
        fn new(a: T, b: T) -> Self {
            Pet {
                cats: a,
                dogs: b,
            }
        }
    }
    
    let pets = Pet::<i64>::new(5_i64, 10_i64);
    println!("Cats: {}, Dogs: {}", pets.cats, pets.dogs);

    let pets = Pet::<String>::new("Million".into(), "Billion".into());
    println!("Cats: {}, Dogs: {}", pets.cats, pets.dogs);
}
```

The `::<>` syntax after `Pet` is called the turbofish. It allows you to explicitly specify the type parameters when creating instances of generic structs or calling generic functions.

## Conclusion

These examples demonstrate various uses of generics in Rust, from simple function definitions to more complex struct and method implementations. Generics provide a powerful way to write flexible, reusable code while maintaining Rust's strong type safety.
