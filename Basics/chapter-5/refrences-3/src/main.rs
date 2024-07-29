struct Anime {
    name: &'static str,
    pass: bool,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn refrences_to_refrences() {
    let point = Point { x: 10, y: 10 };
    let r = &point;
    let rr = &r;
    let rrr = &rr;

    println!("{:?}", rrr);
    println!("{:?}", rr);
    println!("{:?}", r);
    println!("{:?}", point);

    assert_eq!(rrr.y, 10);
    assert_eq!(rrr.x, 10);
}

fn main() {
    let aria = Anime {
        name: "Aria: The animation",
        pass: true,
    };

    assert_eq!(aria.name, "Aria: The animation");
    assert!(aria.pass == true);

    refrences_to_refrences();
}
