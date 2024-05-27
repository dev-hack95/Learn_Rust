use std::str::FromStr;
use std::env;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }
    return n;
}

fn main() {

    // created an empty array of tyoe u64
    let mut numbers: Vec<u64> = Vec::new();

    // cargo run <args>
    // for next args skip 1 
    // push that args into array i.e -> numbers
    for args in env::args().skip(1) {
        numbers.push(u64::from_str(&args).expect("error parsing arguments!"));
    }

    // if len of args enter is 0 then return error and end the process
    if numbers.len() == 0 {
        eprintln!("Usage: gcd Number ...");
        std::process::exit(1);
    }

    // assign first element of array to d
    let mut d = numbers[0];
    // iterate over array and save data in pointers
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
