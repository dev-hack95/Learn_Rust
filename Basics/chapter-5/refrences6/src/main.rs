// Structs containing refrences
// 'a are used to express relationships between the lifetimes of refrences in a struct 
struct S<'a>{
    r: &'a i32
}

// By taking a lifetime parameter 'a and using it in s’s type, we’ve allowed Rust to relate D value’s lifetime to that of the reference its S holds.
struct D<'a>{
    s: S<'a>
}


fn main() {
    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10)  
    }
    println!("Success")
}
