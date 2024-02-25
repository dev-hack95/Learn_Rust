#![allow(dead_code)]
#[derive(Debug)]
struct Anime {
    name: &'static str,
    pass: bool,
}

fn main() {
    let mut y = 32;
    let m = &mut y; // &mut y is a multiple refrence to y
    *m += 32; // Explicitly de-refrencem to set y's values
    println!("{}", m); // to see y's new value

    let jjk = Anime { name: "jjk", pass: true};
    let anime_ref = &jjk;
    println!("{:?}", anime_ref);
    println!("{:?}", jjk);
}
