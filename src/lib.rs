use std::error::Error;
use std::fs;


pub fn run (config: Config)-> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents){
        println!("{}", &line);
    }

    Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {

pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
        return Err("Not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

   Ok(Config { query, filename })
}
    
}

pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();

    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive (){
        let query = "duct";
        let contents = "\
Rust:
safe, productive, fast.
pick three.
Duct tape";

            assert_eq!(vec!["safe, productive, fast."], search(query, contents));
    }
#[test]
    fn case_in_sensitive() {
        let query = "rUST";
        let contents = "\
Rust:
safe, productive, fast.
pick three.
Trust me.";

assert_eq!( vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));

    }
}
