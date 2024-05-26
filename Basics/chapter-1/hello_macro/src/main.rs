// Creating Hello macro

macro_rules! say_hello {
    () => {
        println!("Hello WOrld");
    };
}

fn main() {
    say_hello!();
    println!("Hello, world!");
}
