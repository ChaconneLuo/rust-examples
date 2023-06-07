use std::vec::Vec;

fn main() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("{:?}", vector);

    let second: Option<&i32> = vector.get(1);
    let third: &i32 = &vector[2];

    match second {
        Some(second) => println!("{}", second),
        None => println!("None"),
    }
    println!("{}", third);

    for i in &mut vector {
        *i += 1;
    }
}
