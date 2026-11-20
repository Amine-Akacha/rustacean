fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "the are of a rectangle is {} pixels",
        get_area(width1, height1)
    );
}

fn get_area(width: u32, height: u32) -> u32 {
    width * height
}
