use crate::text::{get_first, get_first_char};

pub mod text;

fn main() {
    println!("{}", get_first("hello world"));
    println!("{}", get_first_char("hello world"));
}
