use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let obj_one = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("obj_one initial rc count = {}", Rc::strong_count(&obj_one));
    println!("obj_one next item = {:?}", obj_one.tail());

    let obj_two = Rc::new(Cons(10, RefCell::new(Rc::clone(&obj_one))));
    println!(
        "obj_one rc count after obj_two creation = {}",
        Rc::strong_count(&obj_one)
    );
    println!("obj_two initial rc count = {}", Rc::strong_count(&obj_two));
    println!("obj_two next item = {:?}", obj_two.tail());

    if let Some(link) = obj_one.tail() {
        *link.borrow_mut() = Rc::clone(&obj_two);
    }

    println!(
        "obj_two rc count after changing obj_one = {}",
        Rc::strong_count(&obj_two)
    );
    println!(
        "obj_one rc count after changing obj_one = {}",
        Rc::strong_count(&obj_one)
    );
}
