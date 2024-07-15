/// Every value have its single owner when owner dropeed the memory assigned
/// to that owner is freed up as we used terms allocated and drooped in rust
fn print_test() {
    // test_arr is a owner with a memeory holding the vec<i32> values
    let mut test_arr = vec![1, 2, 3, 4, 5];
    // lets peforms some action
    for i in 6..100 {
        test_arr.push(i);
    }

    println!("{:?}", test_arr);
    // ownership assigned to test_arr dropped here
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Hello, world!");
    print_test();
    {
        // point has allocated memory here
        let point = Box::new((1, 2));
        let (x, y) = *point;
        println!("{} {}", x, y);
        // point memeory dropped here
    }

    let mut consumers: Vec<Person> = Vec::new();
    consumers.push(Person {
        name: "hello".to_string(),
        age: 8,
    });
    println!("{:?}", consumers);
}
