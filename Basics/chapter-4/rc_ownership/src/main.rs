#![allow(dead_code)]
use std::rc::Rc;

fn main() {
    let s: Rc<String> = Rc::new("udon".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    println!("{}", s);
    println!("{}", t);
    println!("{}", u);`
    //s.push_str(" noodles");
    println!("{}", s);
}