fn refrences_to_vector() {
    let test1: Vec<&dyn String> = vec!["One".to_string(), "Two".to_string(), "Three".to_string()];
    println!("{:?}", test1);
}

fn main() {
    refrences_to_vector();
}
