use crate::intcode::{get_next_output, push_input, run_program, HaltStatus, Program};
use crate::util;
use itertools::Itertools;
use log::trace;

pub fn part1() -> i64 {
    let program = Program::new(util::comma_separated_to_vec("data/d7.txt"));

    (0..5)
        .permutations(5)
        .map(|x| run_phase_sequence(&program, x, 0).1)
        .max()
        .unwrap()
}

pub fn part2() -> i64 {
    let program = Program::new(util::comma_separated_to_vec("data/d7.txt"));

    (5..10)
        .permutations(5)
        .map(|x| run_phase_sequence(&program, x, 0).1)
        .max()
        .unwrap()
}

fn run_phase_sequence(program: &Program, sequence: Vec<i64>, input: i64) -> (Program, i64) {
    let mut amp_a = program.clone();
    let mut amp_b = program.clone();
    let mut amp_c = program.clone();
    let mut amp_d = program.clone();
    let mut amp_e = program.clone();

    // Add the initial sequence as input
    push_input(&mut amp_a, sequence[0]);
    push_input(&mut amp_b, sequence[1]);
    push_input(&mut amp_c, sequence[2]);
    push_input(&mut amp_d, sequence[3]);
    push_input(&mut amp_e, sequence[4]);

    // Insert the initial machine input
    push_input(&mut amp_a, input);

    // We need to grab our final output
    let final_output;

    loop {
        trace!("Using sequence: {:?}", sequence);

        let a_output = get_next_output(run_program(&mut amp_a)).unwrap();

        trace!("amp a output: {}", a_output);
        trace!("amp a halt status: {:?}", amp_a.halt_status.unwrap());

        push_input(&mut amp_b, a_output);

        let b_output = get_next_output(run_program(&mut amp_b)).unwrap();

        trace!("amp b output: {}", b_output);
        trace!("amp b halt status: {:?}", amp_b.halt_status.unwrap());

        push_input(&mut amp_c, b_output);

        let c_output = get_next_output(run_program(&mut amp_c)).unwrap();

        trace!("amp c output: {}", c_output);
        trace!("amp c halt status: {:?}", amp_c.halt_status.unwrap());

        push_input(&mut amp_d, c_output);

        let d_output = get_next_output(run_program(&mut amp_d)).unwrap();

        trace!("amp d output: {}", d_output);
        trace!("amp d halt status: {:?}", amp_d.halt_status.unwrap());

        push_input(&mut amp_e, d_output);

        let e_output = get_next_output(run_program(&mut amp_e)).unwrap();

        trace!("amp e output: {}", e_output);
        trace!("amp e halt status: {:?}", amp_e.halt_status.unwrap());

        if let Some(HaltStatus::Terminated) = amp_e.halt_status {
            final_output = e_output;
            break;
        } else {
            push_input(&mut amp_a, e_output);
        }
    }

    (amp_e, final_output)
}

#[test]
fn phase_sequence() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut input = Program::new(vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ]);

    assert_eq!(43210, run_phase_sequence(&input, vec![4, 3, 2, 1, 0], 0).1);

    input = Program::new(vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ]);

    assert_eq!(54321, run_phase_sequence(&input, vec![0, 1, 2, 3, 4], 0).1);

    input = Program::new(vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ]);

    assert_eq!(65210, run_phase_sequence(&input, vec![1, 0, 4, 3, 2], 0).1);
}

#[test]
fn permutations() {
    let _ = env_logger::builder().is_test(true).try_init();

    itertools::assert_equal((1..3).permutations(2), vec![vec![1, 2], vec![2, 1]]);

    assert_eq!(120, (0..5).permutations(5).count());
}

#[test]
fn feedback_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut input = Program::new(vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ]);

    assert_eq!(
        139629729,
        run_phase_sequence(&input, vec![9, 8, 7, 6, 5], 0).1
    );

    input = Program::new(vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ]);

    assert_eq!(18216, run_phase_sequence(&input, vec![9, 7, 8, 5, 6], 0).1);
}
