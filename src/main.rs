fn main() {
    let mut x = String::from("test");
    let r1 = &x;
    println!("this is x {}", x);
    println!("this is r1 {}", r1);
    println!("this is r1 after print{}", r1);
}
