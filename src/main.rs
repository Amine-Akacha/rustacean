use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::rng().random_range(0..=10);

    println!("#### Guess Game ##### {}", secret);
    println!("Guess a number");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Wrong input");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you guessed the secret number {}", secret);
                break;
            }
        }
    }
}
