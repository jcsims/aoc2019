use crate::intcode;
use crate::intcode::Program;
use crate::util;

pub fn part1() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d9.txt"));

    intcode::push_input(&mut program, 1);

    intcode::run_program(&mut program);

    let final_output = intcode::get_next_output(&mut program).unwrap();

    assert!(intcode::get_next_output(&mut program).is_none());

    final_output
}

pub fn part2() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d9.txt"));

    intcode::push_input(&mut program, 2);

    intcode::run_program(&mut program);

    let final_output = intcode::get_next_output(&mut program).unwrap();

    assert!(intcode::get_next_output(&mut program).is_none());

    final_output
}
