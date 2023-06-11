use std::env;
use std::fs;
use std::str::FromStr;
use strum_macros::EnumString;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    dbg!(&args);
    let Config { filename, query } = Config::new(&args);

    match QueryOption::from_str(&query).unwrap() {
        QueryOption::Show => {
            let contents =
                fs::read_to_string(filename).expect("Something went wrong reading the file");
            println!("With text:\n{}", contents);
        }
    }
}

#[derive(Debug, EnumString)]
enum QueryOption {
    #[strum(serialize = "show")]
    Show,
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("Please provide a file name")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
