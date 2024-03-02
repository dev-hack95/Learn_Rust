fn main() {
    let mut y = 10;
    let m1 = &mut y; 
    //let m2 = &mut y; // error: can't borrow as mutable more than once
    let z = y; // error: can't use `y` beacuse it was mutable borrowed
    //println!("{}, {}, {}", m1, m2, z);
}
