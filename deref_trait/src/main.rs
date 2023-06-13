use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T> {
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let a = 5;
    let b = MyBox::new(a);

    assert_eq!(a, *b);

    let str_box = MyBox::new(String::from("ChaconneLuo"));
    hello(&str_box);
}
