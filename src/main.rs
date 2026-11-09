use std::io;

fn main() {
    println!("######## Temp Converter #########");
    const VALID_UNITS: [&str; 2] = ["c", "f"];

    println!(" Please enter your current Unit (F/C)");
    let unit: String = loop {
        let mut input = String::new();
        // read and clean user input
        io::stdin()
            .read_line(&mut input)
            .expect("couldnt read unit input");
        let input: String = input.trim().to_string().to_lowercase();
        if VALID_UNITS.contains(&input.as_str()) {
            break input;
        }
        println!("Please choose one of the vailable temps {:?}", VALID_UNITS);
    };

    println!("Current unit is {}", unit);
    println!("Enter current temp");

    let temp: f32 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Couldnt read temp input");

        let res: f32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        break res;
    };
    println!("Your current temperature {temp}{unit} is getting converted");
    let (res_temp, target_unit) = match unit.to_string().as_str() {
        "c" => (c_to_f(temp), "F"),
        "f" => (f_to_c(temp), "C"),
        &_ => todo!(),
    };

    println!("We Converted your temp from {temp}{unit} to {res_temp}{target_unit}");
}

fn c_to_f(x: f32) -> f32 {
    (x * 1.8) + 32.0
}

fn f_to_c(y: f32) -> f32 {
    (y - 32.0) * 5.0 / 9.0
}
