// Sharing vs Mutation
fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt)
    }
}
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    {
        let r = &v;
        println!("{}", r[0]);
    }
    let aside = v;
    println!("{:?}", aside);

    let mut wave = Vec::new();
    let head = vec![1.0, 2.0];
    let tail = [3.0, 4.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![1.0, 2.0, 3.0, 4.0]);
    println!("{:?}", wave);
}
