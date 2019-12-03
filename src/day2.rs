use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn part1() -> i32 {
    42
}

fn comma_separated_to_vec(filepath: &str) -> Vec<i32> {
    let mut file = File::open(Path::new(filepath)).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}
