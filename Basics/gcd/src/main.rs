// Here we are writing alogorithum to find greatest common divisor
// Using Euclidean algorithum 
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // The function accepts two varibles which are n, m of unsigned integer 64 
    // conditon-1: The both n, m are not equal to zero
    assert!(n != 0 && m != 0);

    // Euclidean algorithum works on chaining principle
    // chain until m == 0 
    while m != 0 {
        // if n is greater than 1 replace the m with n
        // if not then directly run the code 
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
    println!("{:?}", gcd(14, 15));
    println!("{:?}", gcd(270, 192));
}
