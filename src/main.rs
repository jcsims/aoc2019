mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
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
    match env::args().nth(1) {
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
            "d14p1" => run_one(day14::part1),
            "d14p2" => run_one(day14::part2),
            _ => panic!("unknown exercise: {}", exercise),
        },
    }
}

fn run_all() {
    let start = Instant::now();

    assert_eq!(
        3_442_987,
        run_one_and_return("day1::part1", day1::part1),
        "day1::part1 failed!"
    );

    assert_eq!(
        5_161_601,
        run_one_and_return("day1::part2", day1::part2),
        "day1::part2 failed!"
    );

    assert_eq!(
        3_306_701,
        run_one_and_return("day2::part1", day2::part1),
        "day2::part1 failed!"
    );

    assert_eq!(
        7_621,
        run_one_and_return("day2::part2", day2::part2),
        "day2::part2 failed!"
    );

    assert_eq!(
        651,
        run_one_and_return("day3::part1", day3::part1),
        "day3::part1 failed!"
    );

    assert_eq!(
        7_534,
        run_one_and_return("day3::part2", day3::part2),
        "day3::part2 failed!"
    );

    assert_eq!(
        2_150,
        run_one_and_return("day4::part1", day4::part1),
        "day4::part1 failed!"
    );

    assert_eq!(
        1_462,
        run_one_and_return("day4::part2", day4::part2),
        "day4::part2 failed!"
    );

    assert_eq!(
        9_938_601,
        run_one_and_return("day5::part1", day5::part1),
        "day5::part1 failed!"
    );

    assert_eq!(
        4_283_952,
        run_one_and_return("day5::part2", day5::part2),
        "day5::part2 failed!"
    );

    assert_eq!(
        621_125,
        run_one_and_return("day6::part1", day6::part1),
        "day6::part1 failed!"
    );

    assert_eq!(
        550,
        run_one_and_return("day6::part2", day6::part2),
        "day6::part2 failed!"
    );

    assert_eq!(
        21_000,
        run_one_and_return("day7::part1", day7::part1),
        "day7::part1 failed!"
    );

    assert_eq!(
        61_379_886,
        run_one_and_return("day7::part2", day7::part2),
        "day7::part2 failed!"
    );

    assert_eq!(
        1_792,
        run_one_and_return("day8::part1", day8::part1),
        "day8::part1 failed!"
    );

    assert_eq!(
        42,
        run_one_and_return("day8::part2", day8::part2),
        "day8::part2 failed!"
    );

    assert_eq!(
        2_752_191_671,
        run_one_and_return("day9::part1", day9::part1),
        "day9::part1 failed!"
    );

    assert_eq!(
        87_571,
        run_one_and_return("day9::part2", day9::part2),
        "day9::part2 failed!"
    );

    assert_eq!(
        260,
        run_one_and_return("day10::part1", day10::part1),
        "day10::part1 failed!"
    );

    assert_eq!(
        608,
        run_one_and_return("day10::part2", day10::part2),
        "day10::part2 failed!"
    );

    assert_eq!(
        2_160,
        run_one_and_return("day11::part1", day11::part1),
        "day11::part1 failed!"
    );

    assert_eq!(
        42,
        run_one_and_return("day11::part2", day11::part2),
        "day11::part2 failed!"
    );

    assert_eq!(
        9_139,
        run_one_and_return("day12::part1", day12::part1),
        "day12::part1 failed!"
    );

    assert_eq!(
        420_788_524_631_496,
        run_one_and_return("day12::part2", day12::part2),
        "day12::part2 failed!"
    );

    assert_eq!(
        200,
        run_one_and_return("day13::part1", day13::part1),
        "day13::part1 failed!"
    );

    assert_eq!(
        9_803,
        run_one_and_return("day13::part2", day13::part2),
        "day13::part2 failed!"
    );

    assert_eq!(
        532_506,
        run_one_and_return("day14::part1", day14::part1),
        "day14::part1 failed!"
    );

    assert_eq!(
        2_595_245,
        run_one_and_return("day14::part2", day14::part2),
        "day14::part2 failed!"
    );

    println!("Total elapsed time: {:?}", Instant::elapsed(&start));
}

fn run_one(exercise: fn() -> i64) {
    let now = Instant::now();

    println!("{}", exercise());

    println!("Elapsed time: {:?}", Instant::elapsed(&now));
}

fn run_one_and_return(name: &str, exercise: fn() -> i64) -> i64 {
    let now = Instant::now();

    let result = exercise();

    println!("Elapsed time for {}: {:?}", name, Instant::elapsed(&now));

    result
}
