use std::rc::Rc;

/// Rc -> Refrence counting is memory management technique to use
/// shared ownership of an varible/Object
/// 'Rc' keeps track of how many times the variible is
/// used i.e it track the count of varible use

/// The perfect example we can explain is multiple edges are
/// connected to single node in graph and until and unless each
/// edge is removed from node your can't free the node memory

/// Note: Rc<T> can only be used single threaded application
///

/// RefCell -> Allowing the data to be mutated even when there
/// are immutable refrences to it

fn main() {
    // a, b and c have same meory refrence in heap i.e [3| |string|size|length]
    // 3 is count of Rc pointer i.e refrence counter
    let a = Rc::new("test Rc pointers".to_string());
    let b = a.clone();
    let c = a.clone();

    // b.push_str(" testing is done"); -> Once varible is declared in Rc pointer
    // you can't borrow that memory or a varible
    println!("{}", c);
    println!("{}", b);
}
