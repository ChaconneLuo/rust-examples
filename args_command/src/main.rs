use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    dbg!(&args);

    if args.len() < 2 {
        println!("Please provide a file name");
        return;
    } else {
        let filename = &args[1];
        let path = ".\\src\\".to_string() + filename;
        let contents = fs::read_to_string(&path).expect("Something went wrong reading the file");
        println!("With text:\n{}", contents);
    }
}
