use colored::*;
use std::process;
use minigrep::{
    Config,
    run
};

fn main() {
    //parse the command line arguments 
    let config = Config::new().unwrap();

    //print friendly message
    println!("{}{}", "Searching for: ".green(), format!("{}", config.query).blue());
    match &config.filename {
        Some(filename) => println!("{}{}", "In file: ".green(), format!("{}", filename).blue()),
        None => println!("{}{}", "In file: ".green(), "stdin".blue())
        
    }
    
    //try to read the file
    if let Err(e) = run(config) {
        eprintln!("{}", format!("Application error: {}", e).red());
        process::exit(1);
    }
}