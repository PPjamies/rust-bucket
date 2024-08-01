use std::env;
use std::fs::File;
use std::io::{Error, Read};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, is_case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let results = if config.is_case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}
