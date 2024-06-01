use::num::Complex;
use::std::str::FromStr;
/// The FromStr trait is used for converting a string into another type.

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
/// we can define parse pair as i32 i.e parse_pair::<i32> or parse_pair::<f64>
fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator) {
        None => None, // If seperator not found then return None
        Some(index) => { // If seperator found we proceed with parsing substring
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        // for literal use single quotes
        Some((re, im)) => Some(Complex{re, im}),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(",-123"), None);
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair("10x20", 'x'), Some((10, 20)));
}

/// Given the row and column of a pixel in output image,
/// return the corresponding point on the complex plane.

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width  / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                              Complex { re: -1.0, im:  1.0 },
                              Complex { re:  1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.75 });
} 

fn main() {
    //square_loop(0.5);
    //complex_square_add_loop(Complex{re: 1.0, im: 2.0});
    println!("Hello, world!");
}
