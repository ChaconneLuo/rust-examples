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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
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
