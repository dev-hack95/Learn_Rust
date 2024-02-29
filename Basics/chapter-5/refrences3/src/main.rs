#![allow(dead_code)]
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}  

fn main() {
    let x: u32 = 10;
    let _y: u32 = 20;

    let r: &u32  = &x;

    println!("{}", r);
    let s: String = "Hello world".to_string();
    println!("{}", s);

    let point = Point {x: 10, y: 20};
    let r: &Point = &point;
    let rr: &&Point = &r;
    println!("{:?}", rr);
}
