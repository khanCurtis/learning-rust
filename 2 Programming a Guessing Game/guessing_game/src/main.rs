use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret: i32 = rand::rng().random_range(1..=100);

    println!("DEBUG: Secret: {secret}");

    print!("Guess the number: ");
    io::stdout().flush().unwrap();  // flushing buffer before getting user input
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    if secret == guess {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
