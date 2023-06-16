use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(0 as usize));
    let (tx, rx) = mpsc::channel::<usize>();
    let v: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1];
    let counter = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut mut_counter = counter.lock().unwrap();
        for i in 0..v.len() {
            let now_element = *v.get(i).unwrap();
            *mut_counter = now_element;
            tx.send(now_element).unwrap();
            println!("Sent: {}", now_element);
            thread::sleep(Duration::from_millis(500));
        }
    });
    thread::spawn(move || loop {
        let msg = match rx.recv() {
            Ok(msg) => msg,
            Err(_) => break,
        };
        println!("Got: {}", msg);
    });
    handle.join().unwrap();
    println!("{}", *data.lock().unwrap());
}
