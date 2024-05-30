use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    println!("Search for {}", config.query);
    println!("In file {}", config.query);
    let contents = fs::read_to_string("./src/poem.txt")?;
    println!("{contents}");
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str>{
        // let query = args[1].clone();
        // let filename = args[2].clone();

        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = &args[1];
        let filename = &args[2];

        Ok(Config {
            query,
            filename
        })
    }
}