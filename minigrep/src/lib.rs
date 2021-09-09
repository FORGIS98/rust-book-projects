use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &file_content) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("expecting 2 arguments: `String` and `File`");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut all_matches = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            all_matches.push(line);
        }
    }

    all_matches
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust is a programming language\nit's very safe, fast and productive.\nYou should try it!";

        assert_eq!(
            vec!["it's very safe, fast and productive."],
            search(query, contents)
        );
    }
}
