enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl std::fmt::Display for List<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cons(head, tail) => write!(f, "{}\n{}", head, tail),
            Nil => write!(f, ""),
        }
    }
}

use std::io;
use List::{Cons, Nil};

fn main() {
    let mut length = 0;
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            length = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
        }
        Err(_) => println!("error"),
    }

    if length == 0 {
        return;
    }

    insert_head(length);
    insert_rear(length);
}

fn insert_head(length: u32) -> List<String> {
    let mut list: List<String> = Cons("".to_string(), Box::new(Nil));
    for _i in 0..length {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                list = Cons(input.trim().to_string(), Box::new(list));
            }
            Err(_) => println!("error"),
        }
    }
    println!("{}", list);
    list
}

fn insert_rear(length: u32) -> List<String> {
    let mut list: List<String> = Cons("".to_string(), Box::new(Nil));
    let mut tail = &mut list;
    for _i in 0..length {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                tail = match tail {
                    Cons(_, tail) => tail,
                    Nil => &mut list,
                };
                *tail = Cons(input.trim().to_string(), Box::new(Nil));
            }
            Err(_) => println!("error"),
        }
    }
    println!("{}", list);
    list
}
