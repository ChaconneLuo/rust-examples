use std::{collections::LinkedList, time::Instant};

use linked_list::List;

fn main() {
    let mut own_link_list = linked_list::LinkedList::<i32>::new();
    let mut std_link_list: LinkedList<i32> = LinkedList::<i32>::new();

    let test_cnt = 1000;

    let mut start_time = Instant::now();
    for i in 0..test_cnt {
        own_link_list.push_back(i);
        own_link_list.unshift(i);
    }
    for _i in 0..test_cnt {
        own_link_list.pop();
        own_link_list.shift();
    }
    let mut end_time = Instant::now();

    println!("own:{:?}", end_time - start_time);

    start_time = Instant::now();
    for i in 0..test_cnt {
        std_link_list.push_back(i);
        std_link_list.push_front(i);
    }
    for _i in 0..test_cnt {
        std_link_list.pop_back();
        std_link_list.pop_front();
    }
    end_time = Instant::now();
    println!("std:{:?}", end_time - start_time);
}
