enum ipAddrKind {
    V4,
    V6,
}

fn main() {
    let four = ipAddrKind::V4;
    let six = ipAddrKind::V6;

    println!("this is four {four}");
}
