//! Guessing game
//!
//! Generates a terminal number guessing game by first prompting you for a max
//! number, then allowing you to guess between 0 and that number, while
//! providing helpful hints.
//!
//! # Quick Start
//!
//! To get you started quickly, simply run `cargo run` in the terminal to begin
//! a game.

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Initializes the guessing game by first prompting for a max number.
///
/// # Example
///
/// ```
/// cargo run
/// ```
///
/// # Panics
///
/// If fails to read user input into console for max number or play again.
fn main() {
    println!();
    println!("Welcome to the guessing game.");
    println!("Type \"quit\" at any time to quit.");

    loop {
        let mut max_value = String::new();

        println!();
        println!("What is the max value to guess?");

        io::stdin()
            .read_line(&mut max_value)
            .expect("Failed to read line.");

        let max_value = match input_parser(max_value) {
            Ok(num) => num,
            Err(typed_quit) => {
                if typed_quit {
                    println!("Goodbye!");
                    return;
                };
                println!();
                continue;
            }
        };

        if max_value < 1 {
            println!("Value must be greater than zero.");
            continue;
        }

        let can_play_again = guess(max_value);

        if !can_play_again {
            return;
        }

        println!();
        println!("Enter \"y\" to play again.");

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line.");

        if play_again.trim() != "y" {
            println!("At least you're leaving a winner.");
            return;
        }
    }
}

/// Builds a number guessing game between 0 and the provided max number. Returns
/// true if user won, false if user quit before winning.
///
/// # Example
///
/// ```
/// let can_play_again = guess(max_value);
///
/// if can_play_again {
///     println!("You win!");
/// } else {
///     println!("Better luck next time.");
/// }
/// ```
///
/// # Panics
///
/// If fails to read user input into console.
fn guess(max_value: u32) -> bool {
    println!();
    println!("Guess a number from 1 to {}.", max_value);

    let secret_number: u32 = rand::thread_rng().gen_range(1, max_value + 1);

    loop {
        println!();
        println!("Input your guess!");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match input_parser(guess) {
            Ok(num) => num,
            Err(typed_quit) => {
                if typed_quit {
                    println!("Goodbye!");
                    return false;
                };
                continue;
            }
        };

        if guess > max_value {
            println!("Invalid number: {}", guess);
            continue;
        }

        if guess < 1 {
            println!("Invalid number: {}", guess);
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                return true;
            }
        }
    }
}

/// Parses a user input for a number. If it is a number, returns Ok(num). If
/// not, checks if it is the string "quit" and returns Err(true). If some other
/// error, prints that number is invalid and returns Err(false).
///
/// # Example
///
/// ```
/// let guess: u32 = match input_parser(guess) {
///     Ok(num) => num,
///     Err(typed_quit) => {
///         if typed_quit {
///             println!("Goodbye!");
///             return;
///         };
///         continue;
///     }
/// };
/// ```
fn input_parser(input: String) -> Result<u32, bool> {
    return match input.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => {
            if input.trim() == "quit" {
                return Err(true);
            };
            println!("Invalid number: {}", input.trim());
            return Err(false);
        }
    };
}
