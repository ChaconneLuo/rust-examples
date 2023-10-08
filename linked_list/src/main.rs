use linked_list::{LinkedList, List};

fn main() {
    let mut link_list = LinkedList::<i32>::new();
    link_list.push_back(66);
    link_list.push_back(66);
    link_list.unshift(99);
    println!("link_list length: {}", link_list.len());
    // let mut iter = link_list.into_iter();
    // iter.next();
    // println!("link_list has 99: {}", link_list.find(99, |a, b| a == b));
    // println!("link_list shift: {}", link_list.shift().unwrap());
    // println!("link_list shift: {}", link_list.shift().unwrap());
    println!("link_list pop: {}", link_list.pop().unwrap());
    println!("link_list pop: {}", link_list.pop().unwrap());
    // println!("link_list ele:{}", iter.next().unwrap());
    // println!("link_list ele:{}", iter.next().unwrap());
    // println!("link_list ele:{}", iter.next().unwrap());
}
