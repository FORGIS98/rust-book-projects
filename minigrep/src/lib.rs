use std::{error::Error, fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("expecting 2 arguments: `String` and `File`");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        // is_err() returns false if the env var exists, no matter the value
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut all_matches = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            all_matches.push(line);
        }
    }

    all_matches
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust is\na programming language\nit's very safe, fast and productive.\nYou should try it!\nDuct tape.";

        assert_eq!(
            vec!["it's very safe, fast and productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "ruST";
        let contents = "Rust is\na programming language\nit's very safe, fast and productive.\nYou should try it!\nTrust me.";

        assert_eq!(
            vec!["Rust is", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
