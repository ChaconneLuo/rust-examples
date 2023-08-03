use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn give_away(&self, user_perference: Option<ShirtColor>) -> ShirtColor {
        user_perference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut red_num = 0;
        let mut blue_num = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_num += 1,
                ShirtColor::Blue => blue_num += 1,
            }
        }
        if red_num > blue_num {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let mut list = vec![1, 2, 3];

    let user_perfer = Some(ShirtColor::Blue);
    let user_give = store.give_away(user_perfer);
    println!(
        "The user with preference {:?} gets {:?}",
        user_perfer, user_give
    );
    let return_value = |x| x;
    let closure_test = return_value(String::from("Test"));
    println!("{}", closure_test);

    let mut only_borrows = || list.push(4);
    only_borrows();
    println!("After calling closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
