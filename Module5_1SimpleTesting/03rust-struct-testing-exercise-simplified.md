# Simplified Rust Struct Testing Exercise: BankAccount

## Objective
Create a `BankAccount` struct with methods to deposit, withdraw, and check the balance. Write tests for these methods.

## Instructions

1. Create a new file named `bank_account.rs` in your `src` directory.

2. In `bank_account.rs`, define a `BankAccount` struct with the following:
   - A private field `balance` of type `f64`
   - A public method `new(initial_balance: f64) -> BankAccount` to create a new account
   - Public methods:
     - `deposit(&mut self, amount: f64)`
     - `withdraw(&mut self, amount: f64)`
     - `balance(&self) -> f64`

3. Implement the methods with the following rules:
   - `deposit`: Should increase the balance. Ignore the operation if the amount is negative.
   - `withdraw`: Should decrease the balance. If the amount is greater than the balance or negative, the balance should remain unchanged.
   - `balance`: Should return the current balance.

4. Write tests for your `BankAccount` struct. Include tests for:
   - Creating a new account
   - Depositing money
   - Withdrawing money
   - Checking the balance
   - Edge cases (e.g., depositing/withdrawing negative amounts, withdrawing more than the balance)

5. Update `main.rs` to demonstrate the use of your `BankAccount` struct.

## Starting Template

Here's a template to get you started. Add this to `bank_account.rs`:

```rust
#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
    }

    // Add more tests here
}
```

## Hints
- Use `assert_eq!` to check if values are equal.
- Remember to test both normal operations and edge cases.
- For floating-point comparisons, you might want to use `assert!((a - b).abs() < epsilon)` where `epsilon` is a small number like `1e-10`, to account for potential floating-point inaccuracies.

## Bonus Challenge
Implement and test an `apply_interest` method that increases the balance by a given interest rate.

When you're done, run your tests with `cargo test` and make sure they all pass!
