//! This is a simple library that provides utilities for command line tools

use std::io::{BufReader, BufRead};

pub struct Cli {
    pub delimiter: char,
    pub field: usize,
    pub debug: bool,
}

/// the read_stdin function reads a line from stdin and returns it as a String, and will panic
/// if it fails to read a line with the message "Unable to read input".
pub fn read_stdin() -> String {

    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();

    reader.read_line(&mut line).expect("Unable to read input.");
    line.trim().to_string()

}
/// Splits a `String` by a delimiter specified in `cli` and returns the part at the specified `field` index,
/// or an empty string if the index is out of bounds.
pub fn split_string(s : String, cli : &Cli) -> String {

    let parts : Vec<&str> = s.split(&cli.delimiter).collect();

    if cli.debug {

        println!("Parts : {:?}", parts);
        println!("Indexes available starting at 0 : {}", parts.len());
    
    }

    parts.get(cli.field).unwrap_or(&"").to_string()

}