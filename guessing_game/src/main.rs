use rand::Rng; //importing random library
use std::cmp::Ordering;
use std::io; //importing input/output library

fn main() {
    let num = rand::rng().gen_range(1..=100);

    println!("{num}");

    let mut i = 0;
    while i < 10 {
        println!("Guess a number between 1 and 100");
        let mut input = String::new();
        //getting user input
        io::stdin() //if io wasn't imported, this couldve been written as `std::io::Stdin`
            .read_line(&mut input) //gets imput from user
            .expect("Failed to read line");

        //turns the user input into a integer
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", input);
        match input.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if input != num {
            println!("{} attempts remaining.", 9 - i);
            i += 1;
        }
        if i == 10 {
            println!("Too many attempts. YOU LOSE!")
        }
    }
}
