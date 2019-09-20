use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Invalid Query String"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Invalid File Name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE")
            .is_err();
        return Ok(Config { query, filename, case_sensitive });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let query = "safe";
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
    fn case_sens() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";
        assert_eq!(
            vec!["Rust:"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insens() {
        let query = "rUSt";
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