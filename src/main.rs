use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod test_config_object {
    use super::*;

    #[test]
    fn config_param_exceed() {
        let args = vec![String::from("cmd")];
        assert_eq!(Config::new(&args).err(), Some("not enough arguments"));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", input.query);
    println!("In file {}", input.filename);

    let contents =
        fs::read_to_string(input.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> Config {
    let mut input = Config {
        query: String::from(""),
        filename: String::from(""),
    };

    if args.len() > 1 {
        input.query = args[1].clone();
    }

    if args.len() > 2 {
        input.filename = args[2].clone();
    }

    return input;
}

#[cfg(test)]
mod test_parse_config {
    use super::*;

    #[test]
    fn parse_config_filename() {
        let args = vec![
            String::from("cmd"),
            String::from("query_str"),
            String::from("file.txt"),
        ];
        let input = parse_config(&args);
        assert_eq!(input.filename, "file.txt");
    }

    #[test]
    fn parse_config_query() {
        let args = vec![
            String::from("cmd"),
            String::from("query_str"),
            String::from("file.txt"),
        ];
        let input = parse_config(&args);
        assert_eq!(input.query, "query_str");
    }

    #[test]
    fn parse_config_cmd_only() {
        let args = vec![String::from("cmd")];
        let input = parse_config(&args);
        assert_eq!(input.query, "");
        assert_eq!(input.filename, "");
    }

    #[test]
    fn parse_config_query_str_only() {
        let args = vec![String::from("cmd"), String::from("query_str")];
        let input = parse_config(&args);
        assert_eq!(input.query, "query_str");
        assert_eq!(input.filename, "");
    }
}
