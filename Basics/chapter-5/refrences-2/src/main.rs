/// Rust vs C++ Refrences

/// C++ refrences
/// int x = 10;
/// int &r = x;
/// assert(r == 10); -> Implictily derefrence r to see the x's value
/// r = 20 -> stores the value of x = 20 which is itsel points to r;

fn main() {
    let mut x = 10;
    let r = &mut x;

    assert!(*r == 10); // Explictly derefrence the value of r;
                       // Testing function doesn't  move the value of varible
    *r = 20;
    println!("{}", *r);
}
