use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("Works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("data1".to_string(), vec!["data1_1".to_string(), "data1_2".to_string()]);
    table.insert("data2".to_string(), vec!["data2_1".to_string(), "data2_2".to_string()]);

    show(table);
}
