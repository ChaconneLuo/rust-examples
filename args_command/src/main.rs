use std::env;

use args_command::{run, Config};

fn main() {
    // let args: Vec<String> = env::args().collect::<Vec<String>>();
    // dbg!(&args);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
