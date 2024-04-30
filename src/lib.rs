//! # minigrep
//! this is the minimal representation of the grep command in rust
//! it has only one run function which takes a Config struct and 
//! logs the lines containing the query string to stdout

use std::{
    error::Error, 
    fs, 
    io::{stdin, Read as _}
};
use clap::Parser;
use colored::*;

/// The Config struct will store all necessary parameters as properties 
/// to be used by the run function of this library crate!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(name = "QUERY", required = true, help = "The query string to search for", index = 1)]
    pub query: String,
    #[arg(name = "FILENAME", required = false, help = "The file to search in", index = 2)]
    pub filename: Option<String>,
    #[arg(short, long, help = "Case sensitive search" )]
    pub sensitive: bool
}

impl Config {
    /// Will parse the command line arguments and return as a Config struct
    /// If the command line arguments are invalid, it will return an error
    /// It was made specially for the binary crate to use of this package.
    /// For library use, you can create a custom Config struct directly.
    pub fn new() -> Result<Config, &'static str> {
        let config = Config::parse();
        Ok(config)
    }
}

//color the query string
fn color_query(line:&str,query:&str){
    //split the line after trimming based on spaces
    let words = line.split_whitespace();

    //go through each word and if the word is the query, color it
    for word in words {
        if word.contains(query) {
            //as this word contain the query so divide it into 3 parts

            //before the query
            let before = word.find(query).unwrap();
            print!("{}", &word[..before].normal());

            //the query
            print!("{}", &word[before..before+query.len()].bright_red().bold());

            //after the query
            print!("{} ", &word[before+query.len()..].normal());
        } else {
            print!("{} ", word.normal());
        }
    }
    println!();
}

/// runs the program using the Config struct which will search a query in a file 
/// or input passed via stdin
pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match &config.filename {
        Some(filename) => fs::read_to_string(filename)?,
        None => {
            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    if config.sensitive {
        for line in search_sensitive(&config.query, &contents) {
            color_query(line, &config.query);
        }
    } else {
        for line in search_insensitive(&config.query, &contents) {
            color_query(line, &config.query);
        }
    }
    
    Ok(())
}

fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}


#[cfg(test)]
mod tests {
    //access the parent module
    use super::*;
    
    //test for search_sensitive
    #[test]
    fn case_sensitive(){
        let query = "kawasaki";
        let contents = "Car kawasaki\nMotorcycle kawasaki\n";
        assert_eq!(
            vec!["Car kawasaki", "Motorcycle kawasaki"],
            search_sensitive(query, contents)
        );
    }

    //test for case_insensitive
    #[test]
    fn case_insensitive(){
        let query = "kaWasAki";
        let contents = "Car kawasaki\nMotorcycle kawasaki\n";
        assert_eq!(
            vec!["Car kawasaki", "Motorcycle kawasaki"],
            search_insensitive(query, contents)
        );
    }
}