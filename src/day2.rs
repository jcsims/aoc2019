use crate::intcode;
use crate::util;

pub fn part1() -> i32 {
    let mut alarm_state = util::comma_separated_to_vec("data/d2.txt");

    alarm_state[1] = 12;
    alarm_state[2] = 2;

    let mut program = intcode::empty_program();

    program.state = alarm_state;

    intcode::run_program(&mut program).state[0] as i32
}

pub fn part2() -> i32 {
    let initial_state = util::comma_separated_to_vec("data/d2.txt");

    let mut program = intcode::empty_program();
    program.state = initial_state.to_vec();

    let mut correct_noun = 0;
    let mut correct_verb = 0;

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            program.state[1] = noun;
            program.state[2] = verb;
            if intcode::run_program(&mut program).state[0] == 19690720 {
                correct_noun = noun;
                correct_verb = verb;
                break 'outer;
            } else {
                program.state = initial_state.to_vec();
                program.pointer = 0;
            }
        }
    }

    (100 * correct_noun + correct_verb) as i32
}

#[test]
fn part1_test() {
    assert_eq!(161, util::comma_separated_to_vec("data/d2.txt").len());
}
