use crate::intcode;
use crate::util;

pub fn part1() -> i32 {
    let mut program = intcode::empty_program();

    program.state = util::comma_separated_to_vec("data/d5.txt");
    program.input = 1;

    let final_state = intcode::run_program(&mut program);

    match final_state.output.last() {
        Some(x) => return x.clone(),
        None => panic!("Bad output :("),
    }
}

pub fn part2() -> i32 {
    42
}
