use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

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
            None => {
                return Err("Didn't get a query string");
            }
        };
        println!("query {:?}", query);

        let filename = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Didn't get a filename");
            }
        };

        let case_sensitive;

        match env::var("CASE_SENSITIVE") {
            Ok(value) => {
                if value == "1" {
                    case_sensitive = true;
                } else if value == "0" {
                    case_sensitive = false;
                } else {
                    case_sensitive = true;
                }
            }
            Err(_) => {
                case_sensitive = true;
            }
        }

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents =
            "\
                        Rust:
                        safe, fast, productive.
                        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";

        let contents =
            "\
                        Rust:
                        safe, fast, productive.
                        Pick three.
                        Trust me.
                        ";

        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents))
    }
}
