use std::fmt::Display;
struct SmartPointer<T: Display> {
    data: T,
}

impl<T: Display> SmartPointer<T> {
    fn new(val: T) -> SmartPointer<T> {
        SmartPointer { data: val }
    }
}

impl<T: Display> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping SmartPointer with data {}!", self.data);
    }
}

fn main() {
    let _point_one = SmartPointer::<String>::new(String::from("Point One"));
    let _point_two = SmartPointer::<String>::new(String::from("Point Two"));
    drop(_point_one);
    println!("Pointers created.");
}
