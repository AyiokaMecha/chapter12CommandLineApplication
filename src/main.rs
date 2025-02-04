use std::env;
use std::process;
use chapter12_cli_application::{Config, run};


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("{:?}", args);
    if let Err(e) = chapter12_cli_application::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }


}



