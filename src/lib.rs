use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = String::from(&args[1]);
        let filename = String::from(&args[2]);
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
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
        let _ = run(Config {
            query: String::from("some query"),
            filename: String::from("somefile.txt"),
        })
        .unwrap_err();
    }

    // #[test]
    // fn one_result() {
    //     let query = "duct";
    //     let contents = "\
    //     Rust
    //     safe, fast, productive.
    //     Pick three.";
    //     assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    // }
}
