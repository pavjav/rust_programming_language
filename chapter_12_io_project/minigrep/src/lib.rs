use std::error::Error;
use std::fs;
use std::env;

//This function borrows query and contents and returns a vector of &str types in contents' lifetime
// The conntents of each &str is a slice of the original &str, so no data is duplicated.
pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase(); // Shadowed query with let, did not overwrite it. Once returned will be old value
    let mut results: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // Because query is a String type, need to lend its contents to contains()
            results.push(line);
        }
    }

    results
}

// run takes config, takes ownership of it, and returns a Result
// By setting E=Box<dyn Error> we are allowing the error to take on any object
// that implements the std::error::Error trait
// This makes it easier to handle any error that might arise.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    //println!("With text:\n{contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query,&contents)
    }
    else{
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
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
Duct tape.
Trust me.";

        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}