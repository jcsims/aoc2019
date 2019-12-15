use crate::intcode;
use crate::intcode::Program;
use crate::util;
use log::trace;

pub fn part1() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d11.txt"));

    intcode::push_input(&mut program, 0);

    intcode::run_program(&mut program);

    println!("program halt status: {:?}", program.halt_status);

    println!("program output: {:?}", program.output);
    42
}

pub fn part2() -> i64 {
    42
}

#[derive(Debug, PartialEq, Eq)]
enum Paint {
    // 0, every panel starts black
    Black,
    // 1
    White,
}

fn parse_paint(paint: i64) -> Paint {
    match paint {
        0 => Paint::Black,
        1 => Paint::White,
        x => panic!("Unknown paint color {}", x),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    // 0
    Left,
    // 1
    Right,
}

fn parse_direction(direction: i64) -> Direction {
    match direction {
        0 => Direction::Left,
        1 => Direction::Right,
        x => panic!("Unknown direction color {}", x),
    }
}

// After the robot turns, it should always move forward exactly one
// panel. The robot starts facing up.

// let _ = env_logger::builder().is_test(true).try_init();
