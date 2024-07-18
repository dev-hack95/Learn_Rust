/// Working on structs and their acccess level

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn execute_fn() -> Vec<Person> {
    let mut consumers: Vec<Person> = Vec::new();
    consumers.push(Person {
        name: "consumer1".to_string(),
        age: 10,
    });
    consumers.push(Person {
        name: "consumer2".to_string(),
        age: 11,
    });
    consumers
}

/// Lets study about rust copy types
#[derive(Debug)]
struct Label1 {
    name: String,
}

/// Write some code on function
fn func_label1() -> Vec<Label1> {
    let mut users = Vec::new();
    users.push(Label1 {
        name: "user1".to_string(),
    });
    users
}

/// Copy of Label
// #[derive(Copy, Clone)] -> Copy can't implement on string datatype
// struct Label2 {
//    name: String,
// }
//

#[derive(Clone)] // Clone -> But we can implement clone trait
struct Label2 {
    name: String,
}

/// In case of number we can implement copy and clone
#[derive(Copy, Clone)]
struct NumLabel {
    number: u32,
}

fn main() {
    let consumers = execute_fn();
    // let first_consumer_name = consumers[0].name; -> Can't use
    // can't assign a new varible beacuse it index content can't be assigned to varibles

    for consumer in consumers {
        println!("Name: {}, Age: {}", consumer.name, consumer.age);
    }

    let user1 = func_label1();
    for user in user1 {
        println!("Name {}", user.name);
    }
}
