fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    return v;
}

fn main() {
    let result: Vec<i16> = build_vector();
    println!("{:?}", result);
}
