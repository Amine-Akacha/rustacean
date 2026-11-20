struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };
    println!("the are of a rectangle is {} pixels", get_area(&rect1));
}

fn get_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
