use std::str::FromStr;
use std::env;

/* 
FromStr is a trait
trait is kind of interface
i.e train FromStr {
    fn from_str(&self) {
      // Conversion code
    }
   }
*/

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Error occured while parsing!"));
    }

    if numbers.len() == 0 {
        eprintln!("Zero arguments parsed!");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The gcd of {:?} is {}", numbers, d);
}
