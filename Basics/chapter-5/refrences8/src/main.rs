struct S<'a, 'b>{
    x: &'a i32,
    y: &'b i32,
}

fn sum_r_xy(r: &i32, s: S) -> i32 {
    return r + s.x + s.y;
}

fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
    return (&point[0], &point[1]);
} 

fn main() {
    let x: i32 = 10;
    let y: i32= 20;
    let r = 30;
    let v: Vec<i32> = vec![2, 1, 2];
    let arr: [i32; 3] = [v[0], v[1], v[2]];
    let output: (&i32, &i32) = first_third(&arr);
    println!("({}, {})", *output.0, *output.1);

    let result: i32 = sum_r_xy(&r, S{x: &x, y: &y});
    println!("{}", result);
}
