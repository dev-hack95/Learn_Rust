/// A Vector is a resizable array of elemets Vec<T>, T -> DataType
/// The simplest way to use vector is vec! macro
/// Data in vector is allocated to heap

#[derive(Debug)]
enum Any {
    Integer(i32),
    Text(String),
}

fn print_varible_type<K>(_: &K) {
    println!("{}", std::any::type_name::<K>())
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn main() {
    let primes = vec![2, 3, 5, 7];
    println!("{:?}", primes.iter().product::<i32>());

    let pixel = new_pixel_buffer(3, 3);
    print_varible_type(&pixel);
    println!("{:?}", pixel);


    // A Vector can also be defined with a capacity
    // But it does't prevent the vector from growing
    let mut v: Vec<i32> = Vec::with_capacity(2);
    println!("{}", v.len());
    v.push(2);
    println!("{}", v.len());
    v.push(4);
    println!("{}", v.len());
    v.push(3);
    println!("{:?}", v);

    for i in v {
        println!("{}", i);
    }

    let random_vec: Vec<Any> = vec![Any::Text(String::from("data")), Any::Integer(1)];
    //println!("{}", random_vec);

    for data in random_vec {
        println!("{:#?}", data);
    }    
}
