use crate::util;
use log::trace;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Program {
    pub output: VecDeque<i32>,
    pub state: Vec<i32>,
    pub pointer: usize,
    pub input: VecDeque<i32>,
    pub halt_status: Option<HaltStatus>,
}

enum ParameterMode {
    Immediate,
    Position,
}

#[derive(Debug, Clone, Copy)]
pub enum HaltStatus {
    Terminated, // Go to an opcode 99, all done
    WaitingInput,
}

// Parameters that an instruction writes to will never be in immediate mode.
enum OpCode {
    Add(ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode),
    Write,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
    Stop,
}

impl Program {
    pub fn new(state: Vec<i32>) -> Program {
        Program {
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: state,
            pointer: 0,
            halt_status: None,
        }
    }

    pub fn push_input(program: &mut Self, input: i32) {
        program.input.push_back(input);
    }

    pub fn has_input(program: &Self) -> bool {
        program.input.front().is_some()
    }

    pub fn get_next_input(program: &mut Self) -> Option<i32> {
        program.input.pop_front()
    }

    pub fn push_output(program: &mut Self, output: i32) {
        program.output.push_back(output);
    }

    pub fn get_next_output(program: &mut Self) -> Option<i32> {
        program.output.pop_front()
    }

    pub fn get_last_output(program: &mut Self) -> Option<i32> {
        program.output.pop_back()
    }
}

pub fn run_program(program: &mut Program) -> &mut Program {
    if program.state.is_empty() {
        return program;
    }

    match parse_opcode(program.state[program.pointer]) {
        OpCode::Stop => {
            program.halt_status = Some(HaltStatus::Terminated);
            program
        }
        OpCode::Add(x, y) => run_add_instruction(program, x, y),
        OpCode::Multiply(x, y) => run_mult_instruction(program, x, y),
        OpCode::Write => {
            if Program::has_input(&program) {
                run_save(program)
            } else {
                program.halt_status = Some(HaltStatus::WaitingInput);
                program
            }
        }
        OpCode::Output(x) => run_output(program, x),
        OpCode::JumpIfTrue(x, y) => run_jump(program, true, x, y),
        OpCode::JumpIfFalse(x, y) => run_jump(program, false, x, y),
        OpCode::LessThan(x, y) => run_less_than(program, x, y),
        OpCode::Equals(x, y) => run_equals(program, x, y),
    }
}

fn run_add_instruction(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 2] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 2],
    };
    let destination = program.state[program.pointer + 3] as usize;

    trace!(
        "Adding! operand_1: {}, operand_2: {}, destination: {}",
        operand_1,
        operand_2,
        destination
    );

    program.state[destination] = operand_1 + operand_2;

    program.pointer += 4;

    run_program(program)
}

fn run_mult_instruction(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 2] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 2],
    };
    let destination = program.state[program.pointer + 3] as usize;

    trace!(
        "Multiplying! operand_1: {}, operand_2: {}, destination: {}",
        operand_1,
        operand_2,
        destination
    );

    program.state[destination] = operand_1 * operand_2;
    program.pointer += 4;

    run_program(program)
}

fn run_save(program: &mut Program) -> &mut Program {
    let destination = program.state[program.pointer + 1] as usize;
    let input = Program::get_next_input(program).unwrap();

    trace!(
        "Taking input! destination: {}, input: {}",
        destination,
        input
    );

    program.state[destination] = input;

    program.pointer += 2;

    run_program(program)
}

fn run_output(program: &mut Program, op_mode: ParameterMode) -> &mut Program {
    let output = match op_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
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
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 2] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 2],
    };

    match operand_1 {
        0 if !jump_if => program.pointer = operand_2 as usize,
        x if jump_if && x != 0 => program.pointer = operand_2 as usize,
        _ => program.pointer += 3,
    }

    run_program(program)
}

fn run_less_than(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 2] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 2],
    };
    let destination = program.state[program.pointer + 3] as usize;

    if operand_1 < operand_2 {
        program.state[destination] = 1;
    } else {
        program.state[destination] = 0;
    }

    program.pointer += 4;

    run_program(program)
}
fn run_equals(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
) -> &mut Program {
    let operand_1 = match op1_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 2] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 2],
    };
    let destination = program.state[program.pointer + 3] as usize;

    if operand_1 == operand_2 {
        program.state[destination] = 1;
    } else {
        program.state[destination] = 0;
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
            4 => OpCode::Add(parse_mode(digits[2]), parse_mode(digits[3])),
            3 => OpCode::Add(parse_mode(digits[2]), ParameterMode::Position),
            2 | 1 => OpCode::Add(ParameterMode::Position, ParameterMode::Position),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(2) => match digits.len() {
            4 => OpCode::Multiply(parse_mode(digits[2]), parse_mode(digits[3])),
            3 => OpCode::Multiply(parse_mode(digits[2]), ParameterMode::Position),
            2 | 1 => OpCode::Multiply(ParameterMode::Position, ParameterMode::Position),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(3) => match digits.len() {
            2 | 1 => OpCode::Write,
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
            4 => OpCode::LessThan(parse_mode(digits[2]), parse_mode(digits[3])),
            3 => OpCode::LessThan(parse_mode(digits[2]), ParameterMode::Position),
            2 | 1 => OpCode::LessThan(ParameterMode::Position, ParameterMode::Position),
            _ => panic!("Invalid opcode: {}", opcode),
        },
        Some(8) => match digits.len() {
            4 => OpCode::Equals(parse_mode(digits[2]), parse_mode(digits[3])),
            3 => OpCode::Equals(parse_mode(digits[2]), ParameterMode::Position),
            2 | 1 => OpCode::Equals(ParameterMode::Position, ParameterMode::Position),
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
        _ => panic!("Unknown parameter mode: {}", mode),
    }
}

#[test]
fn day2_intcode_test() {
    assert_eq!(
        run_program(&mut Program::new(vec!(1, 0, 0, 0, 99))).state,
        vec!(2, 0, 0, 0, 99)
    );
    assert_eq!(
        run_program(&mut Program::new(vec!(2, 3, 0, 3, 99))).state,
        vec!(2, 3, 0, 6, 99)
    );
    assert_eq!(
        run_program(&mut Program::new(vec!(2, 4, 4, 5, 99, 0))).state,
        vec!(2, 4, 4, 5, 99, 9801)
    );
    assert_eq!(
        run_program(&mut Program::new(vec!(1, 1, 1, 4, 99, 5, 6, 0, 99))).state,
        vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
    );
}

#[test]
fn basic_io_test() {
    let mut input = Program::new(vec![3, 0, 4, 0, 99]);
    Program::push_input(&mut input, 1);
    assert_eq!(1, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 0, 4, 0, 99]);
    Program::push_input(&mut input, 42);

    assert_eq!(42, run_program(&mut input).output[0]);
}

#[test]
fn opcode_mode_test() {
    assert_eq!(
        1101,
        run_program(&mut Program::new(vec![1101, 100, -1, 4, 0])).state[0]
    );

    assert_eq!(
        99,
        run_program(&mut Program::new(vec![1002, 4, 3, 4, 33])).state[4]
    )
}

#[test]
fn four_more_opcodes() {
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
