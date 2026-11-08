use std::io;

fn main() {
    println!("Temp converter");
    println!("choose unit you want to covert from");
    let mut unit = String::new();

    io::stdin().read_line(&mut unit).expect("couldnt read unit");
    if unit.to_lowercase() != "c" || unit.to_lowercase() != "f" {
        println!("pick F or C");
    }
    let unit = unit.trim().to_lowercase();
    println!("enter temperature in {}", unit);
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("error reading temp");

    let temp: f32 = match temp.trim().parse() {
        Ok(n) => n,
        Err(_) => todo!(),
    };

    let mut result: f32 = 0.0;
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
