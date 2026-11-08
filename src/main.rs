use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::rng().random_range(1..=10);
    println!("guess game");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("couldnt read");

        let guess: u64 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you got it secret was {}", secret);
                break;
            }
        }
    }
}
