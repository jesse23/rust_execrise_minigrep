use std::fs;
use std::env;

struct GrepInput {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let ( query, filename ) = parse_config(&args);
    let input = process_input( &args );

    println!("Searching for {}", input.query);
    println!("In file {}", input.filename);

    let contents = fs::read_to_string(input.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

}

fn parse_config( args: &[String] ) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

fn process_input( args: &[String] ) -> GrepInput {
    let mut input = GrepInput{
        query: String::from(""),
        filename: String::from("")
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
mod tests {
    use super::*;

    #[test]
    fn process_input_filename() {
        let args = vec![ String::from("cmd"), String::from("query_str"), String::from("file.txt") ];
        let input = process_input(&args);
        assert_eq!( input.filename, "file.txt" );
    }

    #[test]
    fn process_input_query() {
        let args = vec![ String::from("cmd"), String::from("query_str"), String::from("file.txt") ];
        let input = process_input(&args);
        assert_eq!( input.query, "query_str" );
    }

    #[test]
    fn process_input_cmd_only() {
        let args = vec![ String::from("cmd") ];
        let input = process_input(&args);
        assert_eq!( input.query, "" );
        assert_eq!( input.filename, "" );
    }

    #[test]
    fn process_input_query_str_only() {
        let args = vec![ String::from("cmd"), String::from("query_str") ];
        let input = process_input(&args);
        assert_eq!( input.query, "query_str" );
        assert_eq!( input.filename, "" );
    }
}