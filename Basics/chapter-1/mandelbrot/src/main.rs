use::num::Complex;
use::std::str::FromStr;
//fn square_loop(mut c : f64) {
//    let mut x = 0.0;
//    loop {
       // x = x * x + c;
//        println!("{}", x);
//    }
//}

#[warn(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z: Complex<f64> = Complex{re: 0.2, im: 0.3};
    loop {
        z = z * z + c;
    }
}

#[warn(dead_code)]
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{re: 0.0, im: 0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c
    }
    None
}

/// Parse the string `s` as a co-ordinate pair, like `"400x6000"` or `"1.0, 0.5"`
/// Specifically `s` should have thr form <left><sep><right> are
/// both strings that can be parsed by `T::from_str` `seperator` must be an ASCII character
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`


/// `<T: FromStr>`: This is a generic type prameter. It indicates that the
/// function works with any type `T` as long as `T` implements the `FromStr`
/// The function return a tuple of two calues of type `T` i.e Option<(T, T)> 
fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair("10x20", 'x'), Some((10, 20)));
}

fn main() {
    //square_loop(0.5);
    //complex_square_add_loop(Complex{re: 1.0, im: 2.0});
    println!("Hello, world!");
}
