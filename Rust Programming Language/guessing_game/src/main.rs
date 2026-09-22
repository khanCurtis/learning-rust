use std::io::{self, Write};

fn main() {
    let answer = 5;

    println!("Guess the number!");
    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input an integer!");
                continue;
            }
        };

        if guess == answer {
            println!("Correct!");
            break;
        } else {
            println!("Incorrect!")
        }
    }
}
