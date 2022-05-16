/*
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
*/

use std::env;
use std::fs;
use std::process;

use std::error::Error;

struct Config{
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // handling the dynamic error from run()
    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// !COMPLETED To be done: handle errors in run() and escalate them to main()
fn run(config: Config) -> Result<(), Box<dyn Error>>{ // dynamic Error handling
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
    // ? will return the error value from the current function for the caller to handle.
    // if the error is not handled, the compiler will throw a warning: note: this `Result` may be an `Err` variant, which should be handled
}
