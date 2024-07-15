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

    let str3 = return_ownership(str2);
    take_ownership(str3);
    //println!("{}", str3);

    indexed_ownership_acces();
    refrence_ownership();
}

/// The function uses allocated varible and droopped as soon as the programme is finished
fn take_ownership(input_str: String) {
    println!("{}", input_str); // input_str is used here
                               // deallocated a varibele  e
}

fn return_ownership(input_str: String) -> String {
    println!("{}", input_str);
    input_str
}

fn indexed_ownership_acces() {
    let mut v = Vec::new();
    for i in 1..5 {
        v.push(i.to_string());
    }
    println!("{:?}", v);

    // let third = v[2]; -> You can't assigned v[2] to a varible name third
    // you can access a varible via indexing but can't assign the varible to
    // the other varible
}

fn refrence_ownership() {
    // If you want to use a varible without taking ownership of that
    // we use refrences there to manage these condtions

    let s1 = String::from("Hello WOrld");
    let (s2, length) = calculate_length_of_string(&s1);
    println!("String: {} and Length: {}", s2, length);
    println!("Can i access s1: {}", s1);
    // We can use s1 now throught the programme or function
    println!("Can we use s1 second time: {}", s1);
}

fn calculate_length_of_string(s: &String) -> (&String, usize) {
    let length = s.len();
    (s, length)
}
