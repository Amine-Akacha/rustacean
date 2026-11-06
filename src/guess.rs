use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::rng().random_range(1..=5);

    println!("secret is {}", secret);

    println!("### Guessing Game ###");

    loop {
        let mut guess = String::new();

        println!("Guess the number!");

        io::stdin()
            .read_line(&mut guess)
            .expect("unexpected happened");

        let guess: u8 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you got it secret was {}!", secret);
                break;
            }
        }
    }
}
