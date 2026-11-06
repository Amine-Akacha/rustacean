use std::io;

fn main() {
    let arr: [i32; 4] = [1, 2, 3, 4];

    let mut idx: String = String::new();
    println!("pick a number from 1 to 3");
    io::stdin().read_line(&mut idx).expect("wrong input");

    let idx: usize = idx.trim().parse().expect("you did it right");

    println!("your number is {}", arr[idx]);
}
