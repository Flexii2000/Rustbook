use std::io;

use rand::Rng;

fn main() {// Guessing Game in Rust
    println!("Welcome to the Guessing Game!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Please enter your guess between 0 and 100");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    
    println!("The secret number is {secret_number} ")
}