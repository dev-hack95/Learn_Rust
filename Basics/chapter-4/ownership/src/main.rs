fn print_padovan() {
    let mut padovan: Vec<i32> = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next)
    }
    println!("{:?}", padovan); // dropped here
}

fn main() {
    //print_padovan();
    // The data between brackets are in same scope
    {
        let point = Box::new((0.625, 0.5)); // Point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(point, label);
        println!("Success");
    }// both dropped here
}