use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = String::from(&args[1]);
        let filename = String::from(&args[2]);
        let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_not_enough_args() {
        let args: [String; 1] = [String::from("1")];
        let msg = Config::new(&args).unwrap_err();
        assert_eq!(msg, "Not enough arguments.")
    }

    #[test]
    fn run_file_does_not_exit() {
        assert!(run(Config {
            query: String::from("some query"),
            filename: String::from("somefile.txt"),
            case_sensitive: false,
        })
        .is_err());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
