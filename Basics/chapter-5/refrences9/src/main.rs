struct StringTable {
    elements: Vec<String>
}

impl StringTable {
    fn find_the_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0 .. self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn main() {
    let table = StringTable {
        elements: vec![
            "apple".to_string(),
            "banana".to_string(),
            "appletini".to_string(),
        ],
    };

    let prefix = "app";
    match table.find_the_prefix(prefix) {
        Some(result) => println!("Found prefix '{}': {}", prefix, result),
        None => println!("No element with prefix '{}'", prefix),
    }

}
