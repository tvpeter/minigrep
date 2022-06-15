use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    
    let args : Vec<String> = env::args().collect();

   let config = Config::new(&args).unwrap_or_else(| err| {
    println!("problem parsing arguments: {}", err);
    process::exit(1);
   });

   println!("searching for: {}", config.query);
   println!("filename: {}", config.filename);

  if let Err(e) = run(config){
    println!("Application error: {}", e);
    process::exit(1);
  }
}

fn run (config: Config)-> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;

    println!("with text: \n {}", contents);

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
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