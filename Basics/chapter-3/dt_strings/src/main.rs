fn main() {
    let error_message = "too many arguments".to_string();
    println!("{}", error_message);

    println!("{}", error_message.contains("too"));
    println!("{}", error_message.replace("too", "Too"));
    println!("{}", error_message.starts_with("t"));
    println!("{}", error_message.starts_with("m"));
    let string = "one, two, three";
    let mut list: Vec<String> = vec![];
    for i in string.split(",") {
        list.push(i.to_string());
    }

    println!("{:?}", list);
}
