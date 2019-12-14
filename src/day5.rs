use crate::intcode;
use crate::intcode::Program;
use crate::util;

pub fn part1() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d5.txt"));

    intcode::push_input(&mut program, 1);

    let final_state = intcode::run_program(&mut program);

    match intcode::get_last_output(final_state) {
        Some(x) => return x.clone(),
        None => panic!("Bad output :("),
    }
}

pub fn part2() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d5.txt"));

    intcode::push_input(&mut program, 5);

    let final_state = intcode::run_program(&mut program);

    match intcode::get_next_output(final_state) {
        Some(x) => return x.clone(),
        None => panic!("Bad output :("),
    }
}
