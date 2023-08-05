use std::error::Error;
use std::fs;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum QueryOption {
    #[strum(serialize = "show")]
    Show,
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 2 {
        //     return Err("not enough arguments");
        // }
        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Didn't get a filename"),
        };
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match QueryOption::from_str(&config.query).unwrap() {
        QueryOption::Show => {
            let contents = fs::read_to_string(&config.filename)
                .expect("Something went wrong reading the file");
            println!("With text:\n{}", contents);
        }
    }
    Ok(())
}
