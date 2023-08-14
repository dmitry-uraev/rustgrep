use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments given!");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok (Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file: File = File::open(config.filename).expect("File not found!");

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            lines.push(line);
        }
    }

    lines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "some";
        let content = "Some string,\ngood string\nanother some string";

        assert_eq!(
            vec!["another some string"],
            search(query, content)
        );
    }
}