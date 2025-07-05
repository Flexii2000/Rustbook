use std::io;

fn main() {// Guessing Game in Rust
    println!("Welcome to the Guessing Game!");

    println!("Please enter your guess between 0 and 90");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
