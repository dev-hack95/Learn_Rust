use std::mem;

fn analyze_slices(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
    println!("All elements in array {:?}", slice);
}

fn main() {
    // Array with size 5 and conatins data type as 32-bit integer
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // All elements in array can be initialed as a constant number
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    println!("Number of elemnts in array: {}", xs.len());
    println!("Array occupes {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice");
    analyze_slices(&xs);
    analyze_slices(&ys[1 .. 4]);
    
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
