// Return refrences
fn smallest(v: &[i32]) -> &i32 {
    // s is pointing refrence of 0th index
    let mut s = &v[0];
    // The r will iterate from first index
    for r in &v[1..] {
        // if s > r the replace s with r 
        if *s > *r { 
            s = r;
        }
    }
    return s;
}

fn main() {
    let s;
    let parabola = [9, 4, 1, 0, 1, 4, 9];
    s = smallest(&parabola);
    println!("{}", s);
}