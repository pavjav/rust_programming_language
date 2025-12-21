use std::error::Error;
use std::fs;
use std::env;

//This function borrows query and contents and returns a vector of &str types in contents' lifetime
// The conntents of each &str is a slice of the original &str, so no data is duplicated.

// We modify search to use iterators instead. This means we skip out on using an intermediate return variable.
// We will use filter() on the contents.lines() iterator
pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


// Here we use filter() in conjunction with to_lowercase on both self/argument
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

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
    // We modify this to take a mut args iterator from env::args()
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next(); // Skip this value because its just the name of the binary
        // Use matching on the Option<String> values to store the query, file_path vars
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't receive a query string!")
        };
        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file path!")
        };
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