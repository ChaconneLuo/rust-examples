use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
    let handle = thread::spawn(move || {
        for i in 0..10 {
            println!("{}", v.get(i).unwrap());
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
