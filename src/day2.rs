use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn part1() -> i32 {
    assert_eq!(
        run_program(&mut vec!(1, 0, 0, 0, 99), 0).to_vec(),
        vec!(2, 0, 0, 0, 99)
    );
    assert_eq!(
        run_program(&mut vec!(2, 3, 0, 3, 99), 0).to_vec(),
        vec!(2, 3, 0, 6, 99)
    );
    assert_eq!(
        run_program(&mut vec!(2, 4, 4, 5, 99, 0), 0).to_vec(),
        vec!(2, 4, 4, 5, 99, 9801)
    );
    assert_eq!(
        run_program(&mut vec!(1, 1, 1, 4, 99, 5, 6, 0, 99), 0).to_vec(),
        vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
    );
    assert_eq!(161, comma_separated_to_vec("data/d2.txt").len());

    let mut alarm_state = comma_separated_to_vec("data/d2.txt");

    alarm_state[1] = 12;
    alarm_state[2] = 2;

    run_program(&mut alarm_state, 0)[0] as i32
}

pub fn part2() -> i32 {
    let initial_state = comma_separated_to_vec("data/d2.txt");

    let mut working_state = initial_state.to_vec();
    let mut correct_noun = 0;
    let mut correct_verb = 0;

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            working_state[1] = noun;
            working_state[2] = verb;
            if run_program(&mut working_state, 0)[0] == 19690720 {
                correct_noun = noun;
                correct_verb = verb;
                break 'outer;
            } else {
                working_state = initial_state.to_vec();
            }
        }
    }

    (100 * correct_noun + correct_verb) as i32
}

fn run_program(program: &mut Vec<usize>, pointer: usize) -> &Vec<usize> {
    if program.is_empty() {
        return program;
    }

    match program[pointer] {
        99 => program,
        1 => run_add_instruction(program, pointer),
        2 => run_mult_instruction(program, pointer),
        _ => program,
    }
}

fn run_add_instruction(program: &mut Vec<usize>, pointer: usize) -> &Vec<usize> {
    let operand_1 = program[program[pointer + 1]];
    let operand_2 = program[program[pointer + 2]];
    let destination = program[pointer + 3];

    program[destination] = operand_1 + operand_2;

    run_program(program, pointer + 4)
}

fn run_mult_instruction(program: &mut Vec<usize>, pointer: usize) -> &Vec<usize> {
    let operand_1 = program[program[pointer + 1]];
    let operand_2 = program[program[pointer + 2]];
    let destination = program[pointer + 3];

    program[destination] = operand_1 * operand_2;

    run_program(program, pointer + 4)
}

fn comma_separated_to_vec(filepath: &str) -> Vec<usize> {
    let mut file = File::open(Path::new(filepath)).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split(',')
        .map(|x| match x.trim().parse::<usize>() {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("Error parsing string: {} with err: {}", x, err);
                0
            }
        })
        .collect()
}
