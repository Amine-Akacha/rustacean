fn main() {
    let rect1 = (30, 50);
    println!("the are of a rectangle is {} pixels", get_area(rect1));
}

fn get_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
