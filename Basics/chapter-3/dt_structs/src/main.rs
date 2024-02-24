// An attribute to hide warning for unused code.
#![allow(dead_code)]


#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(f32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Circle {
    r: f32
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let pi = 3.14;
    let peter = Person{name, age};
 
    println!("{:?}", peter);

    let point: Point = Point{x:5.0, y: 4.0};
    println!("{:?}", point);

    let bottom_right = Point{x:2.5, ..point};
    println!("{:?}", bottom_right);

    let circle: Circle = Circle{r: 3.0};
    println!("{}", circle.r);
    println!("Area of circle {}", pi*circle.r*circle.r);

    let _rectangle = Rectangle {
        top_left: point,
        bottom_right: bottom_right,
    };


    let _unit = Unit;

    let pair = Pair(1.0, 0.1);
    println!("{}, {}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("{}, {}", integer, decimal);
}
