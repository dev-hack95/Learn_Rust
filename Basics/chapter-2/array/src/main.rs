/// A vector is nothing but an array of size n 
/// A Vector can of many size it can be of 1X1 or 1X2 ... etc

fn build_vector() -> Vec<i32> {
    // Declare a mutable vector(array) which will only accept of i32 type in data
    let mut v: Vec<i32> = Vec::<i32>::new();
    v.push(1); // Push data to an vector i.e array
    v.push(2);
    v.push(3);
    return v;
}

fn main() {
    println!("{:?}", build_vector());
}
