use::num::Complex;
//fn square_loop(mut c : f64) {
//    let mut x = 0.0;
//    loop {
       // x = x * x + c;
//        println!("{}", x);
//    }
//}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z: Complex<f64> = Complex{re: 0.2, im: 0.3};
    loop {
        z = z * z + c;
    }
}

fn main() {
    //square_loop(0.5);
    complex_square_add_loop(Complex{re: 1.0, im: 2.0});
    println!("Hello, world!");
}
