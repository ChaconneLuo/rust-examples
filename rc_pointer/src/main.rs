use std::rc::Rc;
use List::{Cons, Nil};

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

fn main() {
    let common_list = Rc::new(Cons::<i32>(10, Rc::new(Nil)));
    let _list1 = Cons(5, Rc::clone(&common_list));
    let _list2 = Cons(3, Rc::clone(&common_list));
    println!("strong count: {:}", Rc::strong_count(&common_list));
}
