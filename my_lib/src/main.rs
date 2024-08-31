use std::env;
use my_lib;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Basic argument parsing
    let delimiter = args.get(1).map_or(',', |d| d.chars().next().unwrap_or(','));
    let field = args.get(2).and_then(|f| f.parse().ok()).unwrap_or(0);
    let debug = args.get(3).map_or(false, |d| d == "debug");

    let cli = my_lib::Cli {
        delimiter,
        field,
        debug,
    };

    let buffer = my_lib::read_stdin();
    let result = my_lib::split_string(buffer, &cli);
    println!("{}", result);
}

