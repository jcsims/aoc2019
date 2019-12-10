use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn lines_from_path(filepath: &str) -> io::Lines<std::io::BufReader<std::fs::File>> {
    let file = File::open(Path::new(filepath)).unwrap();

    BufReader::new(file).lines()
}

pub fn comma_separated_to_vec(filepath: &str) -> Vec<i32> {
    let mut file = File::open(Path::new(filepath)).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split(',')
        .map(|x| match x.trim().parse::<i32>() {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("Error parsing string: {} with err: {}", x, err);
                0
            }
        })
        .collect()
}

pub fn digits(input: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();

    let mut temp = input;

    while temp > 0 {
        digits.insert(0, temp % 10);
        temp /= 10;
    }

    digits
}
