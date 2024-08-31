use std::io::{BufReader, BufRead};

/// the read_stdin function reads a line from stdin and returns it as a String, and will panic
/// if it fails to read a line with the message "Unable to read input".
pub fn read_stdin() -> String {

    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();

    reader.read_line(&mut line).expect("Unable to read input.");
    line.trim().to_string()

}