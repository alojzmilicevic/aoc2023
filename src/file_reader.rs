use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    reader.lines().collect()
}