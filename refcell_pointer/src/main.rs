use std::cell::RefCell;

use refcell_pointer::{LimitTracker, Message};

struct Messenger {
    message: RefCell<Vec<String>>,
}

impl Messenger {
    fn new() -> Messenger {
        Messenger {
            message: RefCell::new(vec![]),
        }
    }
}

impl Message for Messenger {
    fn send(&self, msg: &str) {
        self.message.borrow_mut().push(String::from(msg));
    }
}

fn main() {
    let messenger = Messenger::new();
    let tracker = LimitTracker::new(&messenger, 100);
    tracker.send("hello");
    tracker.send("world");
    println!("length: {}", messenger.message.borrow().len());
}
