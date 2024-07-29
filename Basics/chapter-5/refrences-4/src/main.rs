fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

fn main() {
    let f5 = factorial(5);
    let t = 1000;
    println!("{}", f5 + &t);

    // In this case rust create an anonymous varible
    // for refrence value &1000 and add the value.

    // The lifetime of this anonymous varible depends
    // on ehat you do with the refence.
    assert_eq!(f5 + &1000, 1120);
}
