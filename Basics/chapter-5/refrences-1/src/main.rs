use std::collections::HashMap;

/// A Table is a type of datatype which contain a hashmap
/// of type string -> List
type Table = HashMap<String, Vec<String>>;

/// show -> function to display dat which is stored in hashmap
fn show(table: &Table) {
    // First for loop takes ownership of table i.e Hashmap
    for (consumer, websites) in table {
        println!("Consumer uses {}", consumer);
        // Second for take ownership of Vec<String>
        for website in websites {
            println!("* {}", website);
        }
    }
}

fn sort_websites(table: Table) {
    for (_conusmer, mut websites) in table {
        websites.sort();
        println!("{:?}", websites);
    }
}

fn main() {
    // Memory fo table is allocated here
    let mut table = Table::new();
    table.insert(
        "consumer2".to_string(),
        vec!["amazon".to_string(), "flipkart".to_string()],
    );
    table.insert(
        "consumer1".to_string(),
        vec!["netflix".to_string(), "hotstar".to_string()],
    );

    show(&table);
    sort_websites(table); // memory for table droppped here
}
