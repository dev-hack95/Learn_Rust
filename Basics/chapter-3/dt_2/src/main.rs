fn main() {
    println!("{}", 2_u16.pow(4));
    println!("{}", 0b101101_u8.count_ones());
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    let i: i32 = 1;
    println!("{:?}", i.checked_add(27));
    println!("{:?}", i.checked_neg());
    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));
}
