mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod intcode;
mod util;

use std::env;
use std::time::Instant;

fn main() {
    env_logger::init();
    match env::args().skip(1).next() {
        None => run_all(),
        Some(exercise) => match exercise.as_ref() {
            "d1p1" => run_one(day1::part1),
            "d1p2" => run_one(day1::part2),
            "d2p1" => run_one(day2::part1),
            "d2p2" => run_one(day2::part2),
            "d3p1" => run_one(day3::part1),
            "d3p2" => run_one(day3::part2),
            "d4p1" => run_one(day4::part1),
            "d4p2" => run_one(day4::part2),
            "d5p1" => run_one(day5::part1),
            "d5p2" => run_one(day5::part2),
            "d6p1" => run_one(day6::part1),
            "d6p2" => run_one(day6::part2),
            "d7p1" => run_one(day7::part1),
            "d7p2" => run_one(day7::part2),
            "d8p1" => run_one(day8::part1),
            "d8p2" => run_one(day8::part2),
            "d9p1" => run_one(day9::part1),
            "d9p2" => run_one(day9::part2),
            _ => panic!("unknown exercise: {}", exercise),
        },
    }
}

fn run_all() {
    let now = Instant::now();

    assert_eq!(3442987, day1::part1(), "day1::part1 failed!");
    assert_eq!(5161601, day1::part2(), "day1::part2 failed!");
    assert_eq!(3306701, day2::part1(), "day2::part1 failed!");
    assert_eq!(7621, day2::part2(), "day2::part2 failed!");
    assert_eq!(651, day3::part1(), "day3::part1 failed!");
    assert_eq!(7534, day3::part2(), "day3::part2 failed!");
    assert_eq!(2150, day4::part1(), "day4::part1 failed!");
    assert_eq!(1462, day4::part2(), "day4::part2 failed!");
    assert_eq!(9938601, day5::part1(), "day5::part1 failed!");
    assert_eq!(4283952, day5::part2(), "day5::part2 failed!");
    assert_eq!(621125, day6::part1(), "day6::part1 failed!");
    assert_eq!(550, day6::part2(), "day6::part2 failed!");
    assert_eq!(21000, day7::part1(), "day7::part1 failed!");
    assert_eq!(61379886, day7::part2(), "day7::part2 failed!");
    assert_eq!(1792, day8::part1(), "day8::part1 failed!");
    assert_eq!(42, day8::part2(), "day8::part2 failed!");
    assert_eq!(2752191671, day9::part1(), "day9::part1 failed!");
    assert_eq!(87571, day9::part2(), "day9::part2 failed!");

    println!("Elapsed time: {:?}", Instant::elapsed(&now));
}

fn run_one(exercise: fn() -> i64) {
    let now = Instant::now();

    println!("{}", exercise());

    println!("Elapsed time: {:?}", Instant::elapsed(&now));
}
