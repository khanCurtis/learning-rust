use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret: u32 = rand::rng().random_range(1..=100);

    // println!("DEBUG: Secret: {secret}");

    loop {
        print!("Guess the number: ");
        io::stdout().flush().unwrap();  // flushing buffer before getting user input

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
