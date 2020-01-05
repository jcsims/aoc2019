mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
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
            "d10p1" => run_one(day10::part1),
            "d10p2" => run_one(day10::part2),
            "d11p1" => run_one(day11::part1),
            "d11p2" => run_one(day11::part2),
            "d12p1" => run_one(day12::part1),
            "d12p2" => run_one(day12::part2),
            "d13p1" => run_one(day13::part1),
            "d13p2" => run_one(day13::part2),
            _ => panic!("unknown exercise: {}", exercise),
        },
    }
}

fn run_all() {
    let start = Instant::now();
    let mut ex_start;

    assert_eq!(3442987, day1::part1(), "day1::part1 failed!");
    println!("day::part1 elapsed time: {:?}", Instant::elapsed(&start));

    ex_start = Instant::now();
    assert_eq!(5161601, day1::part2(), "day1::part2 failed!");
    println!("day::part2 elapsed time: {:?}", Instant::elapsed(&ex_start));

    ex_start = Instant::now();
    assert_eq!(3306701, day2::part1(), "day2::part1 failed!");
    println!(
        "day2::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(7621, day2::part2(), "day2::part2 failed!");
    println!(
        "day2::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(651, day3::part1(), "day3::part1 failed!");
    println!(
        "day3::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(7534, day3::part2(), "day3::part2 failed!");
    println!(
        "day3::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(2150, day4::part1(), "day4::part1 failed!");
    println!(
        "day4::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(1462, day4::part2(), "day4::part2 failed!");
    println!(
        "day4::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(9938601, day5::part1(), "day5::part1 failed!");
    println!(
        "day5::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(4283952, day5::part2(), "day5::part2 failed!");
    println!(
        "day5::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(621125, day6::part1(), "day6::part1 failed!");
    println!(
        "day6::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(550, day6::part2(), "day6::part2 failed!");
    println!(
        "day6::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(21000, day7::part1(), "day7::part1 failed!");
    println!(
        "day7::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(61379886, day7::part2(), "day7::part2 failed!");
    println!(
        "day7::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(1792, day8::part1(), "day8::part1 failed!");
    println!(
        "day8::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(42, day8::part2(), "day8::part2 failed!");
    println!(
        "day8::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(2752191671, day9::part1(), "day9::part1 failed!");
    println!(
        "day9::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(87571, day9::part2(), "day9::part2 failed!");
    println!(
        "day9::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(260, day10::part1(), "day10::part1 failed!");
    println!(
        "day10::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(608, day10::part2(), "day10::part2 failed!");
    println!(
        "day10::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(2160, day11::part1(), "day11::part1 failed!");
    println!(
        "day11::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(42, day11::part2(), "day11::part2 failed!");
    println!(
        "day11::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(9139, day12::part1(), "day12::part1 failed!");
    println!(
        "day12::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(420788524631496, day12::part2(), "day12::part2 failed!");
    println!(
        "day12::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(200, day13::part1(), "day13::part1 failed!");
    println!(
        "day13::part1 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    ex_start = Instant::now();
    assert_eq!(9803, day13::part2(), "day13::part2 failed!");
    println!(
        "day13::part2 elapsed time: {:?}",
        Instant::elapsed(&ex_start)
    );

    println!("Total elapsed time: {:?}", Instant::elapsed(&start));
}

fn run_one(exercise: fn() -> i64) {
    let now = Instant::now();

    println!("{}", exercise());

    println!("Elapsed time: {:?}", Instant::elapsed(&now));
}
