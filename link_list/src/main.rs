use link_list::{LinkedList, List};

fn main() {
    let mut link_list = LinkedList::<i32>::new();
    link_list.unshift(66);
    link_list.push_back(66);
    link_list.push_back(99);
    println!("link_list length: {}", link_list.len());
    println!("link_list has 99: {}", link_list.find(99, |a, b| a == b));
    println!("link_list shift: {}", link_list.shift().unwrap());
    println!("link_list shift: {}", link_list.shift().unwrap());
    println!("link_list pop: {}", link_list.pop().unwrap());
    println!("link_list pop: {}", link_list.len());
}
