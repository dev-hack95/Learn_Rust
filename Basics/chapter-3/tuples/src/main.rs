fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    return (bool_param, int_param);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let pair = (1, true);
    println!("The reverse is: {:?}", reverse(pair));

    let tuple = (1, "test", true, 8.9);
    println!("Print Tuple: {:?}", tuple);
    // Lets acces the single carible
    let (a, b, c, d) = tuple;
    println!("I am aceesing single varibles in tuples: {}, {}, {}, {}", a, b, c, d);

    // Working with struct
    // Struct is a collection of data types
    let matrix = Matrix(1.0, 1.1, 1.2, 1.3);
    println!("{:?}", matrix);
}
