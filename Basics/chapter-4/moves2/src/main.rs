fn main() {
    let mut v: Vec<String> = Vec::new();
    for i in 101..106 {
        v.push(i.to_string())
    }
    println!("{:?}", v);

    v.pop().expect("vector empty!");
    println!("{:?}", v);

    let second = v.swap_remove(1);
    println!("{}", second);
    println!("{:?}", v);
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    println!("{}", third);
    println!("{:?}", v);
    for i in v {
        println!("{}", i);
    }
}