// My name is Jesus Martinez July 17, 2024
fn check_guess(guess: i32, secret: i32) -> i32 {
    // check guess function
    if guess == secret {
        return 0;
    } else if guess < secret {
        return -1;
    } else {
        return 1;
    }
}

fn main() {
    let mut num = 0;
    loop {
        if num > 10 {
            /// To stop the infinite loop, needs user input
            break;
        }
        let mut guess = 90; // user's guess
        if check_guess(guess, 45) == 0 {
            // calling function to see if its the same
            println!("Your guess was Correct!");
            break;
        } else if check_guess(guess, 45) == -1 {
            // calling function to see if its the smaller
            println!("Your guess was Wrong, Too Low!");
        } else {
            println!("Your guess was Wrong, Too High!");
        }
        num += 1; // keeps amount of guesses done
    }
    println!("It took you {} guesses to guess the secret number", num);
}
