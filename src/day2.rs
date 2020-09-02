use crate::intcode;
use crate::intcode::{ParameterMode, Program};
use crate::util;

pub fn part1() -> i64 {
    let mut alarm_state = util::comma_separated_to_vec("data/d2.txt");

    alarm_state[1] = 12;
    alarm_state[2] = 2;

    let mut program = Program::new(alarm_state);

    intcode::get_state(
        intcode::run_program(&mut program),
        0,
        ParameterMode::Immediate,
    )
}

pub fn part2() -> i64 {
    let initial_state = util::comma_separated_to_vec("data/d2.txt");

    let mut program = Program::new(initial_state.to_vec());

    let mut correct_noun = 0;
    let mut correct_verb = 0;

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            intcode::set_state(&mut program, 1, noun);
            intcode::set_state(&mut program, 2, verb);
            if intcode::get_state(
                intcode::run_program(&mut program),
                0,
                ParameterMode::Immediate,
            ) == 19_690_720
            {
                correct_noun = noun;
                correct_verb = verb;
                break 'outer;
            } else {
                program = Program::new(initial_state.to_vec());
            }
        }
    }

    100 * correct_noun + correct_verb
}

#[test]
fn part1_test() {
    assert_eq!(161, util::comma_separated_to_vec("data/d2.txt").len());
}
