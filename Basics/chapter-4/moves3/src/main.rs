#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: i32,
}

fn main() {
    let mut composers: Vec<Person> = Vec::new();
    composers.push(Person{name: Some("person1".to_string()), age: 15});
    composers.push(Person{name: Some("person2".to_string()), age: 16});
    composers.push(Person{name: Some("person3".to_string()), age: 17});

    println!("The array is {:?}", composers);

    let first_name = &composers[0].name.take();
    println!("{:?}", first_name);
}