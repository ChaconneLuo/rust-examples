fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = list.iter();

    for val in iter.clone() {
        println!("{}", val);
    }

    let plus_list: Vec<i32> = iter.map(|x| x + 1).collect();
    println!("{:?}", plus_list);
}
