fn main() {
    let s = String::from("testworld!");
    let l = first_word(&s);

    println!("first word is {}", l);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..s.len()]
}
