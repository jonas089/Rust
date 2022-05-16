use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    println!("[Search Results]");
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    println!("[END]");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // iterator
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line selected by iterator
            results.push(line);
        }
    }
    results // return results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn on_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        //Asserts that two expressions are equal to each other
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
