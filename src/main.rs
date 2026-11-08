use std::io;

fn main() {
    println!("######## Temp Converter #########");

    println!(" Please enter your current Unit (F/C)");
    let unit: String = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("couldnt read unit input");
        let input: String = input.trim().parse().expect("couldnt read unit input");

        println!("input, {}", input);
    };
}
