# Rust Programming Assignments

## Assignment 1: Temperature Converter

Create a Rust program that converts temperatures between Fahrenheit and Celsius. The program should:

1. Declare a constant for the freezing point of water in Fahrenheit (32°F).
2. Implement two functions:
   - `fahrenheit_to_celsius(f: f64) -> f64`: Converts Fahrenheit to Celsius
   - `celsius_to_fahrenheit(c: f64) -> f64`: Converts Celsius to Fahrenheit
3. In the main function:
   - Declare a mutable variable with a temperature in Fahrenheit
   - Convert it to Celsius using your function and print the result
   - Use a loop to convert and print the next 5 integer temperatures (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)


## Assignment 2: Number Analyzer

Create a Rust program that analyzes a series of numbers. The program should:

1. Create an array of 10 integer numbers of your choice.
2. Implement a function `is_even(n: i32) -> bool` that returns true if a number is even, false otherwise.
3. Use a for loop to iterate through the array and for each number:
   - Print whether it's even or odd using your `is_even` function
   - If the number is divisible by 3, print "Fizz" instead
   - If the number is divisible by 5, print "Buzz" instead
   - If it's divisible by both 3 and 5, print "FizzBuzz"
4. Use a while loop to find and print the sum of all numbers in the array.
5. Use a loop to find and print the largest number in the array.

## Assignment 3: Guessing Game

Create a simple number guessing game in Rust. The program should:

1. Use a mutable variable to store a "secret" number (you can hard-code this).
2. Implement a function `check_guess(guess: i32, secret: i32) -> i32` that returns:
   - 0 if the guess is correct
   - 1 if the guess is too high
   - -1 if the guess is too low
3. In the main function:
   - Use a loop to repeatedly:
     - Set a mutable guess variable to a number of your choice (simulating user input)
     - Call the `check_guess` function
     - Use an if-else expression to print whether the guess was correct, too high, or too low
     - If the guess was correct, break the loop
   - After the loop ends, print how many guesses it took (you'll need to track this in a variable)

These assignments strictly use only the concepts covered in the provided materials: variables, mutability, basic data types (integers, booleans), arrays, functions, if-else expressions, and the three types of loops (loop, while, for). 


