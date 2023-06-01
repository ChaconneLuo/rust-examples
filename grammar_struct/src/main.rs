#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(width: {}, height: {})", self.width, self.height)
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn idbg<T: std::fmt::Debug + std::fmt::Display>(val: T) -> T {
    println!("{}", val);
    println!("{:?}", val);
    val
}

fn main() {
    let mut rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    rec1 = idbg(rec1);
    println!("rec1 is {:?}, area is {}", rec1, area(&rec1));
}
