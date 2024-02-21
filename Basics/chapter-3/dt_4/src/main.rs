fn main() {
    let mut chaos = [3, 4, 5, 2, 1];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
    println!("{:?}", chaos);
    println!("Success");

    let mut primes = vec![2, 3, 4, 5];
    primes.push(11);
    primes.push(13);
    println!("{:?}", primes);

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let mut v1 = Vec::with_capacity(2);
    v1.push(1);
    v1.push(2);
    println!("{:?}", v1);
}
