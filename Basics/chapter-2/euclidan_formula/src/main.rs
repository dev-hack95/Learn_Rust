fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Assert Function Check that n and m should not euql to zero
    // If they are euqal to zero they might throw error
    assert!(n != 0 && m != 0);
    // Run while loop until m is not equal to zero
    while m != 0 {
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(357, 3), 3);
} 
