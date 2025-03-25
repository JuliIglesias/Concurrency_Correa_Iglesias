use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::{io, thread};

pub fn read_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.trim().is_empty())
        .collect();
    Ok(lines)
}
