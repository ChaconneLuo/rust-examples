use link_list::{LinkedList, List};

fn main() {
    let link_list = LinkedList::<i32>::new();
    link_list.unshift(Option::Some(66));
    link_list.push_back(Option::Some(66));
    link_list.push_back(Option::Some(99));
    println!("link_list length: {}", link_list.len());
    println!(
        "link_list has 99: {}",
        link_list.find(Some(99), |a, b| a == b)
    );
}
