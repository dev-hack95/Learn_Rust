/// The most programming language allow users to create
/// arbitrary graphs of objects that points to each other
/// in whatever you see fit. But rust worked on tree based
/// model in which the values are worked as children and
/// the varible worked as root in tree

fn main() {
    let str1: String = String::from("Hello");
    let str2: String = str1; // Move (not a shallow copy)

    println!("{:?}", str2);
    // println!("{:?}", str1); -> this line of code will not work
    // beacuse str1 have lost the ownership of str1

    take_ownership(str2);
    //println!("{}", str2);
}

fn take_ownership(input_str: String) {
    println!("{}", input_str);
}

fn returnownership(input_str: String) {
    return input_str;
}
