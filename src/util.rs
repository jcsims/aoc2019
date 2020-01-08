use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn lines_from_path(filepath: &str) -> io::Lines<std::io::BufReader<std::fs::File>> {
    let file = File::open(Path::new(filepath)).unwrap();

    BufReader::new(file).lines()
}

pub fn comma_separated_to_vec(filepath: &str) -> Vec<i64> {
    let mut file = File::open(Path::new(filepath)).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split(',')
        .map(|x| match x.trim().parse::<i64>() {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("Error parsing string: {} with err: {}", x, err);
                0
            }
        })
        .collect()
}

pub fn file_as_string(filepath: &str) -> String {
    let file = File::open(Path::new(filepath)).unwrap();

    let mut out_string = String::new();

    match BufReader::new(file).read_to_string(&mut out_string) {
        Err(e) => panic!("Error reading string from file: {:?}", e),
        _ => out_string,
    }
}

pub fn digits(input: i64) -> Vec<i64> {
    let mut digits: Vec<i64> = Vec::new();

    let mut temp = input;

    while temp > 0 {
        digits.insert(0, temp % 10);
        temp /= 10;
    }

    digits
}

// Modified from https://stackoverflow.com/a/40168843
pub struct DecreasingRange {
    pub start: i64,
    pub end: i64,
    pub step: i64,
}

impl DecreasingRange {
    pub fn new(start: i64, end: i64, step: i64) -> DecreasingRange {
        DecreasingRange { start, end, step }
    }
}

impl Iterator for DecreasingRange {
    type Item = i64;

    #[inline]
    fn next(&mut self) -> Option<i64> {
        if self.start > self.end {
            let v = self.start;
            self.start = v + self.step;
            Some(v)
        } else {
            None
        }
    }
}
