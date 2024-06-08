fn main() {
    println!("Addition {:?}", 100_i8.checked_add(27));
    println!("Subtraction {:?}", 100_i8.checked_sub(27));
    println!("Multiplication {:?}", 10_i8.saturating_mul(3));
    println!("Divison {:?}", 9_i8.wrapping_div(3));
    println!("Check Negative {:?}", (-128_i16).checked_neg());
    println!("Remainder {:?}", (55_i16).wrapping_rem(3));
    println!("Make absoulte {:?}", (-128_i16).wrapping_abs());
    println!("Check power {:?}", (25_i16).checked_pow(2));
    println!("Bit wise left shift {:?}", 10_u32.wrapping_shl(34));
    println!("Bit wise right shift {:?}", 40_u64.wrapping_shr(66));
}
