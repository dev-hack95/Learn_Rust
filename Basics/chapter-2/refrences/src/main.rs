fn main() {
    let x: i32 = 11;
    let x_ref: &i32 = &x;

    println!("Address of x: {}", &x as *const i32 as usize);
    println!("Address of x_ref: {}", x_ref as *const i32 as usize);

    assert_eq!(x_ref as *const i32 as usize, &x as *const i32 as usize);

    let mut s1 = String::from("Hello ");
    let s2 = mutable_refrences(&mut s1);
    println!("String2: {}", s2);
}

fn mutable_refrences(s: &mut String) -> &mut String {
    s.push_str("World");
    println!("InputString: {}", s);
    s
}
