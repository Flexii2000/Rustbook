use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {// Guessing Game in Rust
    println!("Welcome to the Guessing Game!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess between 0 and 100");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>
                {
                    println!("You win!");
                    break;
                },
        }
    }

}

