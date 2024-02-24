struct Person {
    firstname: String,
    lastname: String,
    age: i32,
}


fn main() {
    let mut composers: Vec<Person> = Vec::new();
    composers.push(Person{firstname: "person1".to_string(), lastname: "lastname1".to_string(), age: 15});
    composers.push(Person{firstname: "person2".to_string(), lastname: "lastname2".to_string(), age: 16});
    composers.push(Person{firstname: "person3".to_string(), lastname: "lastname3".to_string(), age: 17});
    composers.push(Person{firstname: "person4".to_string(), lastname: "lastname4".to_string(), age: 18});

    for composer in &composers {
        println!("firstname: {}, lastname: {}, age: {}", composer.firstname, composer.lastname, composer.age);
    }

}