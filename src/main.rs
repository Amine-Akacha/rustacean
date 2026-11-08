use std::io;

fn main() {
    println!("Temp converter");
    println!("choose unit you want to convert from (F or C)");

    let unit = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("couldnt read unit");

        let input = input.trim().to_lowercase();

        if input == "c" || input == "f" {
            break input;
        }

        println!("choose F or C");
    };
    println!("set your current temp in {}", unit);

    let temp = loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("cant read temp");

        match input.trim().parse() {
            Ok(n) => break n,
            Err(_) => continue,
        };
    };
    let result: f32;
    println!("current unit is {}", unit);
    if unit == "f" {
        result = f_to_c(temp);
        println!("temperature from F to C is {}", result);
    } else if unit == "c" {
        result = c_to_f(temp);
        println!("temperature from F to C is {}", result);
    } else {
        println!("couldnt read");
    }
}

fn f_to_c(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

fn c_to_f(x: f32) -> f32 {
    ((x * 9.0) / 5.0) + 32.0
}
