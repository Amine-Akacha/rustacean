fn main() {
    let rect1 = (30, 50);
    println!(
        "the are of a rectangle is {} pixels",
        get_area(rect1.0, rect1.1)
    );
}

fn get_area(width: u32, height: u32) -> u32 {
    width * height
}
