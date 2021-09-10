use rand::Rng;
use std::{cmp::Ordering, io, io::Write};

fn main() {
    println!("Let's play a little game! Try to guess a random number between 1 and 100! You only have 7 attempts.");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut counter = 7;

    loop {
        print!("\nPlease input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if counter == 0 {
            println!("You loose!");
            break;
        } else {
            counter -= 1;
            println!("Number of attempts: {}", counter);
        }
    }
}
