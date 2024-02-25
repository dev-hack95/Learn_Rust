fn main() {
    let s: Vec<String> = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();

    println!("{:?}", s);
    println!("{:?}", t);
    println!("{:?}", u);

    let mut s1: String = "udon".to_string();
    s1 = "ramen".to_string(); // value `udon` drop here

    let mut s2: String = "udon".to_string();
    // t1 has taken ownership of s2
    let t1: String = s2;
    s2 = "ramen".to_string(); // nothing dropped here

    println!("{}", s1);
    println!("{}", t1);
    println!("{}", s2);

    let mut v: Vec<i32> = Vec::new();
    for i in 101..106 {
        v.push(i);
    }
    println!("{:?}", v);
 
    let third: i32 = v[2]; // third value
    let fifth: i32 = v[4]; // Fifth

    println!("{}, {}", third, fifth);
    v.pop().expect("Vector Empty");

    println!("{:?}", v);
}
