use crate::util;
use log::trace;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Program {
    pub output: VecDeque<i32>,
    pub state: HashMap<i32, i32>,
    pub pointer: i32,
    pub relative_base: i32,
    pub input: VecDeque<i32>,
    pub halt_status: Option<HaltStatus>,
}

enum ParameterMode {
    Immediate,
    Position,
    Relative,
}

#[derive(Debug, Clone, Copy)]
pub enum HaltStatus {
    Terminated, // Got an opcode 99, all done
    WaitingInput,
}

// Parameters that an instruction writes to will never be in immediate mode.
enum OpCode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    Write(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    Stop,
}

impl Program {
    pub fn new(state: Vec<i32>) -> Program {
        Program {
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: vec_to_map(state),
            pointer: 0,
            relative_base: 0,
            halt_status: None,
        }
    }

    pub fn push_input(program: &mut Program, input: i32) {
        program.input.push_back(input);
    }

    pub fn has_input(program: &Program) -> bool {
        program.input.front().is_some()
    }

    pub fn get_next_input(program: &mut Program) -> Option<i32> {
        program.input.pop_front()
    }

    pub fn push_output(program: &mut Program, output: i32) {
        program.output.push_back(output);
    }

    pub fn get_next_output(program: &mut Program) -> Option<i32> {
        program.output.pop_front()
    }

    pub fn get_last_output(program: &mut Program) -> Option<i32> {
        program.output.pop_back()
    }

    pub fn get_state(program: &Program, pointer: i32) -> i32 {
        if pointer < 0 {
            panic!("Invalid pointer (less than zero): {}", pointer);
        }

        match program.state.get(&pointer) {
            Some(x) => {
                trace!("getting state, key: {}, value: {}", pointer, x);
                *x
            }
            None => 0,
        }
    }

    pub fn get_relative_state(program: &Program, pointer: i32) -> i32 {
        let relative_pointer = program.relative_base + pointer;
        if relative_pointer < 0 {
            panic!(
                "Invalid (relative) pointer (less than zero)! pointer: {}, relative_base: {}",
                pointer, program.relative_base
            );
        }
        match program.state.get(&relative_pointer) {
            Some(x) => *x,
            None => 0,
        }
    }

    pub fn set_state(program: &mut Program, key: i32, value: i32) {
        program.state.insert(key, value);
    }
}

pub fn run_program(program: &mut Program) -> &mut Program {
    if program.state.is_empty() {
        return program;
    }

    match parse_opcode(Program::get_state(program, program.pointer)) {
        OpCode::Stop => {
            program.halt_status = Some(HaltStatus::Terminated);
            program
        }
        OpCode::Add(x, y, z) => run_add_instruction(program, x, y, z),
        OpCode::Multiply(x, y, z) => run_mult_instruction(program, x, y, z),
        OpCode::Write(x) => {
            if Program::has_input(&program) {
                run_save(program, x)
            } else {
                program.halt_status = Some(HaltStatus::WaitingInput);
                program
            }
        }
        OpCode::Output(x) => run_output(program, x),
        OpCode::JumpIfTrue(x, y) => run_jump(program, true, x, y),
        OpCode::JumpIfFalse(x, y) => run_jump(program, false, x, y),
        OpCode::LessThan(x, y, z) => run_less_than(program, x, y, z),
        OpCode::Equals(x, y, z) => run_equals(program, x, y, z),
    }
}

fn run_add_instruction(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
    op3_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 1))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 1),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 1),
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 2))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 2),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 2),
    };

    let destination = match op3_mode {
        ParameterMode::Position => Program::get_state(program, program.pointer + 3),
        ParameterMode::Immediate => {
            panic!("Tried to get a write destination using immediate mode!")
        }
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 3),
    };

    trace!(
        "Adding! operand_1: {}, operand_2: {}, destination: {}",
        operand_1,
        operand_2,
        destination
    );

    program.state.insert(destination, operand_1 + operand_2);

    program.pointer += 4;

    run_program(program)
}

fn run_mult_instruction(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
    op3_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 1))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 1),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 1),
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 2))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 2),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 2),
    };

    let destination = match op3_mode {
        ParameterMode::Position => Program::get_state(program, program.pointer + 3),
        ParameterMode::Immediate => {
            panic!("Tried to get a write destination using immediate mode!")
        }
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 3),
    };

    trace!(
        "Multiplying! operand_1: {}, operand_2: {}, destination: {}",
        operand_1,
        operand_2,
        destination
    );

    program.state.insert(destination, operand_1 * operand_2);
    program.pointer += 4;

    run_program(program)
}

fn run_save(program: &mut Program, op1_mode: ParameterMode) -> &mut Program {
    let destination = match op1_mode {
        ParameterMode::Position => Program::get_state(program, program.pointer + 1),
        ParameterMode::Immediate => {
            panic!("Tried to get a write destination using immediate mode!")
        }
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 1),
    };

    let input = Program::get_next_input(program).unwrap();

    trace!(
        "Taking input! destination: {}, input: {}",
        destination,
        input
    );

    program.state.insert(destination, input);

    program.pointer += 2;

    run_program(program)
}

fn run_output(program: &mut Program, op_mode: ParameterMode) -> &mut Program {
    let output = match op_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 1))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 1),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 1),
    };

    trace!("Pushing output: {}", output);

    Program::push_output(program, output);

    program.pointer += 2;

    run_program(program)
}

fn run_jump(
    program: &mut Program,
    jump_if: bool,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 1))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 1),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 1),
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 2))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 2),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 2),
    };

    match operand_1 {
        0 if !jump_if => program.pointer = operand_2,
        x if jump_if && x != 0 => program.pointer = operand_2,
        _ => program.pointer += 3,
    }

    run_program(program)
}

fn run_less_than(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
    op3_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 1))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 1),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 1),
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 2))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 2),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 2),
    };

    let destination = match op3_mode {
        ParameterMode::Position => Program::get_state(program, program.pointer + 3),
        ParameterMode::Immediate => {
            panic!("Tried to get a write destination using immediate mode!")
        }
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 3),
    };

    if operand_1 < operand_2 {
        program.state.insert(destination, 1);
    } else {
        program.state.insert(destination, 0);
    }

    program.pointer += 4;

    run_program(program)
}
fn run_equals(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
    op3_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 1))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 1),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 1),
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => {
            Program::get_state(program, Program::get_state(program, program.pointer + 2))
        }
        ParameterMode::Immediate => Program::get_state(program, program.pointer + 2),
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 2),
    };

    let destination = match op3_mode {
        ParameterMode::Position => Program::get_state(program, program.pointer + 3),
        ParameterMode::Immediate => {
            panic!("Tried to get a write destination using immediate mode!")
        }
        ParameterMode::Relative => Program::get_relative_state(program, program.pointer + 3),
    };

    if operand_1 == operand_2 {
        program.state.insert(destination, 1);
    } else {
        program.state.insert(destination, 0);
    }

    program.pointer += 4;

    run_program(program)
}

////////////////////////////////////////////////////////////////
// Parsing

fn parse_opcode(opcode: i32) -> OpCode {
    let mut digits = util::digits(opcode);
    digits.reverse();

    match digits.first() {
        None => panic!("Unable to parse opcode!"),
        Some(1) => match digits.len() {
            5 => OpCode::Add(
                parse_mode(digits[2]),
                parse_mode(digits[3]),
                parse_mode(digits[4]),
            ),

            4 => OpCode::Add(
                parse_mode(digits[2]),
                parse_mode(digits[3]),
                ParameterMode::Position,
            ),
            3 => OpCode::Add(
                parse_mode(digits[2]),
                ParameterMode::Position,
                ParameterMode::Position,
            ),
            2 | 1 => OpCode::Add(
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position,
            ),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(2) => match digits.len() {
            5 => OpCode::Multiply(
                parse_mode(digits[2]),
                parse_mode(digits[3]),
                parse_mode(digits[4]),
            ),
            4 => OpCode::Multiply(
                parse_mode(digits[2]),
                parse_mode(digits[3]),
                ParameterMode::Position,
            ),
            3 => OpCode::Multiply(
                parse_mode(digits[2]),
                ParameterMode::Position,
                ParameterMode::Position,
            ),
            2 | 1 => OpCode::Multiply(
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position,
            ),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(3) => match digits.len() {
            3 => OpCode::Write(parse_mode(digits[2])),
            2 | 1 => OpCode::Write(ParameterMode::Position),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(4) => match digits.len() {
            3 => OpCode::Output(parse_mode(digits[2])),
            2 | 1 => OpCode::Output(ParameterMode::Position),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(5) => match digits.len() {
            4 => OpCode::JumpIfTrue(parse_mode(digits[2]), parse_mode(digits[3])),
            3 => OpCode::JumpIfTrue(parse_mode(digits[2]), ParameterMode::Position),
            2 | 1 => OpCode::JumpIfTrue(ParameterMode::Position, ParameterMode::Position),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(6) => match digits.len() {
            4 => OpCode::JumpIfFalse(parse_mode(digits[2]), parse_mode(digits[3])),
            3 => OpCode::JumpIfFalse(parse_mode(digits[2]), ParameterMode::Position),
            2 | 1 => OpCode::JumpIfFalse(ParameterMode::Position, ParameterMode::Position),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(7) => match digits.len() {
            5 => OpCode::LessThan(
                parse_mode(digits[2]),
                parse_mode(digits[3]),
                parse_mode(digits[4]),
            ),
            4 => OpCode::LessThan(
                parse_mode(digits[2]),
                parse_mode(digits[3]),
                ParameterMode::Position,
            ),
            3 => OpCode::LessThan(
                parse_mode(digits[2]),
                ParameterMode::Position,
                ParameterMode::Position,
            ),
            2 | 1 => OpCode::LessThan(
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position,
            ),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(8) => match digits.len() {
            5 => OpCode::Equals(
                parse_mode(digits[2]),
                parse_mode(digits[3]),
                parse_mode(digits[4]),
            ),
            4 => OpCode::Equals(
                parse_mode(digits[2]),
                parse_mode(digits[3]),
                ParameterMode::Position,
            ),
            3 => OpCode::Equals(
                parse_mode(digits[2]),
                ParameterMode::Position,
                ParameterMode::Position,
            ),
            2 | 1 => OpCode::Equals(
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position,
            ),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(9) => match digits.get(1) {
            Some(9) => OpCode::Stop,
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(_) => panic!("Invalid opcode: {}", opcode),
    }
}

fn parse_mode(mode: i32) -> ParameterMode {
    match mode {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
        _ => panic!("Unknown parameter mode: {}", mode),
    }
}

fn vec_to_map(vec: Vec<i32>) -> HashMap<i32, i32> {
    let mut hash_state = HashMap::new();

    for (i, val) in vec.iter().enumerate() {
        hash_state.insert(i as i32, val.clone());
    }
    hash_state
}

#[test]
fn day2_intcode_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    assert_eq!(
        run_program(&mut Program::new(vec!(1, 0, 0, 0, 99))).state,
        vec_to_map(vec!(2, 0, 0, 0, 99))
    );
    assert_eq!(
        run_program(&mut Program::new(vec!(2, 3, 0, 3, 99))).state,
        vec_to_map(vec!(2, 3, 0, 6, 99))
    );
    assert_eq!(
        run_program(&mut Program::new(vec!(2, 4, 4, 5, 99, 0))).state,
        vec_to_map(vec!(2, 4, 4, 5, 99, 9801))
    );
    assert_eq!(
        run_program(&mut Program::new(vec!(1, 1, 1, 4, 99, 5, 6, 0, 99))).state,
        vec_to_map(vec!(30, 1, 1, 4, 2, 5, 6, 0, 99))
    );
}

#[test]
fn basic_io_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut input = Program::new(vec![3, 0, 4, 0, 99]);
    Program::push_input(&mut input, 1);

    trace!("Program: {:?}", input);

    assert_eq!(
        1,
        Program::get_next_output(run_program(&mut input)).unwrap()
    );

    input = Program::new(vec![3, 0, 4, 0, 99]);
    Program::push_input(&mut input, 42);

    assert_eq!(
        42,
        Program::get_next_output(run_program(&mut input)).unwrap()
    );
}

#[test]
fn opcode_mode_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    assert_eq!(
        1101,
        Program::get_state(run_program(&mut Program::new(vec![1101, 100, -1, 4, 0])), 0)
    );

    assert_eq!(
        99,
        Program::get_state(run_program(&mut Program::new(vec![1002, 4, 3, 4, 33])), 4)
    )
}

#[test]
fn four_more_opcodes() {
    let _ = env_logger::builder().is_test(true).try_init();

    // equals
    let mut input = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    Program::push_input(&mut input, 8);
    assert_eq!(1, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    Program::push_input(&mut input, 42);
    assert_eq!(0, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    Program::push_input(&mut input, 42);
    assert_eq!(0, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    Program::push_input(&mut input, 8);
    assert_eq!(1, run_program(&mut input).output[0]);

    // less than
    input = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    Program::push_input(&mut input, 42);
    assert_eq!(0, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    Program::push_input(&mut input, 2);
    assert_eq!(1, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    Program::push_input(&mut input, 42);
    assert_eq!(0, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    Program::push_input(&mut input, 4);
    assert_eq!(1, run_program(&mut input).output[0]);

    // Jumps
    input = Program::new(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);
    Program::push_input(&mut input, 42);
    assert_eq!(1, run_program(&mut input).output[0]);

    input = Program::new(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);
    Program::push_input(&mut input, 0);

    assert_eq!(0, run_program(&mut input).output[0]);

    // More complex test
    input = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    Program::push_input(&mut input, 4);
    assert_eq!(999, run_program(&mut input).output[0]);

    input = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    Program::push_input(&mut input, 8);
    assert_eq!(1000, run_program(&mut input).output[0]);

    input = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    Program::push_input(&mut input, 42);
    assert_eq!(1001, run_program(&mut input).output[0]);
}
