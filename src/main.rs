use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::rng().random_range(0..=10);

    println!("Guess a number");

    loop {}
}
