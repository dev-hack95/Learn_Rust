fn print_varible_type<K>(_: &K) {
    println!("{}", std::any::type_name::<K>())
}

fn main() {
    // Define array of a particular size 
    // arr1 accepts the u32 datatype of size 6 
    let mut arr1: [u32; 6] = [1, 2, 3, 4, 5, 5];
    println!("arr1: {:?}", arr1);
    arr1[5] = 6;
    println!("arr1: {:?}", arr1);

    let arr2 = ["a", "b", "c"];
    print_varible_type(&arr2);
    println!("arr2: {:?}", arr2);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    println!("{:?}", chaos);
}
