use std::fs;
use std::env;

struct GrepInput {
    query: String,
    filename: String,
}

fn main() {
    let input = process_input( env::args().collect() );

    println!("Searching for {}", input.query);
    println!("In file {}", input.filename);

    let contents = fs::read_to_string(input.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

}

fn process_input( args: Vec<String> ) -> GrepInput {
    let mut input = GrepInput{
        query: String::from(""),
        filename: String::from("")
    };

    if args.len() > 0 {
        input.query = args[1].clone();
    }

    if args.len() > 1 {
        input.query = args[2].clone();
    }

    return input;
}
