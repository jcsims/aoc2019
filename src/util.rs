use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn lines_from_path(filepath: &str) -> io::Lines<std::io::BufReader<std::fs::File>> {
    let file = File::open(Path::new(filepath)).unwrap();

    BufReader::new(file).lines()
}
