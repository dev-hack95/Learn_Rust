fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = first_word(&s1);
    println!("{:?}", s2);
}

/// Create a function which will return the first word of gibven sentece
fn first_word(s: &String) -> &[u8] {
    let bytes = s.as_bytes();
    bytes
}
