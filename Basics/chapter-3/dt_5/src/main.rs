fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.insert(5, 7);
    v.remove(1);
    println!("{:?}", v);
}
