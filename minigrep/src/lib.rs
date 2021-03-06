use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}
impl Config {
    /*pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { 
            query, 
            filename,
            case_sensitive
        })
    }*/
    // Args impl Iterator, iterator holds fn next() => next() can be called on Args instance
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        args.next(); // skip index 0, as there is a target stored in it.
        println!("{:?}", args);
                                                                                                                                                                                                                                                                                                                                                                                                                                                      
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a valid filename")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else{
        search_case_insensitive(&config.query, &contents)
    };
    
    
    println!("With text:\n{}", contents);
    println!("[Search Results]");
    for line in results {
        println!("{}", line);
    }
    println!("[END]");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // iterator
    /*let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line selected by iterator
            results.push(line);
        }*/
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
/* Test the search function
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
*/
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],
        search(query, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three. 
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}


/* an additional test
#[cfg(test)]
mod test2{
    use super::*;

    #[test]
    fn on_result(){
        let query = "duct";
        println!("{}", query.to_string());
    }
}

When Running cargo test:
output:
running 2 tests
test test2::on_result ... ok
test tests::on_result ... ok
---

*/
