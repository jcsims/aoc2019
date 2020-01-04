use crate::util;
use log::trace;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Program {
    pub output: VecDeque<i64>,
    pub state: HashMap<i64, i64>,
    pub pointer: i64,
    pub relative_base: i64,
    pub input: VecDeque<i64>,
    pub halt_status: Option<HaltStatus>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParameterMode {
    // (1) The Parameter is interpreted as a value
    Immediate,
    // (0) The parameter is interpreted as a position (default if not set)
    Position,
    // (2) The address a relative mode parameter refers to is itself plus
    // the current relative base
    Relative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HaltStatus {
    Terminated, // Got an opcode 99, all done
    WaitingInput,
}

// Parameters that an instruction writes to will never be in immediate
// mode.
#[derive(Debug)]
enum OpCode {
    // (01) Add parameter 1 and 2, write to parameter 3
    Add(ParameterMode, ParameterMode, ParameterMode),
    // (02) Multiply parameter 1 and 2, write to parameter 3
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    // (03) Write the next input to parameter 1
    Input(ParameterMode),
    // (04) Write parameter 1 to output
    Output(ParameterMode),
    // (05) if the first parameter is non-zero, it sets the instruction
    // pointer to the value from the second parameter
    JumpIfTrue(ParameterMode, ParameterMode),
    // (06) if the first parameter is zero, it sets the instruction pointer
    // to the value from the second parameter
    JumpIfFalse(ParameterMode, ParameterMode),
    // (07) if the first parameter is less than the second parameter, it
    // stores 1 in the position given by the third
    // parameter. Otherwise, it stores 0
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    // (08) if the first parameter is equal to the second parameter, it
    // stores 1 in the position given by the third
    // parameter. Otherwise, it stores 0
    Equals(ParameterMode, ParameterMode, ParameterMode),
    // (09) Adjust the relative base by parameter 1
    AdjustRelBase(ParameterMode),
    // (99)
    Stop,
}

impl Program {
    pub fn new(state: Vec<i64>) -> Program {
        Program {
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: vec_to_map(state),
            pointer: 0,
            relative_base: 0,
            halt_status: None,
        }
    }
}

pub fn push_input(program: &mut Program, input: i64) {
    program.input.push_back(input);
}

pub fn has_input(program: &Program) -> bool {
    program.input.front().is_some()
}

pub fn get_next_input(program: &mut Program) -> Option<i64> {
    program.input.pop_front()
}

pub fn push_output(program: &mut Program, output: i64) {
    program.output.push_back(output);
}

pub fn get_next_output(program: &mut Program) -> Option<i64> {
    program.output.pop_front()
}

pub fn get_last_output(program: &mut Program) -> Option<i64> {
    program.output.pop_back()
}

pub fn get_state(program: &Program, pointer: i64, pm: ParameterMode) -> i64 {
    match pm {
        ParameterMode::Immediate => {
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
        ParameterMode::Position => {
            trace!("getting position state, pointer: {}", pointer);
            get_state(
                program,
                get_state(program, pointer, ParameterMode::Immediate),
                ParameterMode::Immediate,
            )
        }
        ParameterMode::Relative => {
            trace!("getting relative state, pointer: {}", pointer);
            get_state(
                program,
                get_state(program, pointer, ParameterMode::Immediate) + program.relative_base,
                ParameterMode::Immediate,
            )
        }
    }
}

pub fn is_terminated(program: &Program) -> bool {
    match program.halt_status {
        Some(HaltStatus::Terminated) => true,
        _ => false,
    }
}

pub fn is_waiting_input(program: &Program) -> bool {
    match program.halt_status {
        Some(HaltStatus::WaitingInput) => true,
        _ => false,
    }
}

pub fn get_destination(program: &Program, pointer: i64, pm: ParameterMode) -> i64 {
    match pm {
        ParameterMode::Immediate => {
            panic!("Tried to get a write destination using immediate mode!")
        }
        ParameterMode::Position => get_state(program, pointer, ParameterMode::Immediate),
        ParameterMode::Relative => {
            get_state(program, pointer, ParameterMode::Immediate) + program.relative_base
        }
    }
}

pub fn set_state(program: &mut Program, key: i64, value: i64) {
    program.state.insert(key, value);
}

pub fn run_program(program: &mut Program) -> &mut Program {
    if program.state.is_empty() {
        return program;
    }

    loop {
        match parse_opcode(get_state(
            program,
            program.pointer,
            ParameterMode::Immediate,
        )) {
            OpCode::Stop => {
                program.halt_status = Some(HaltStatus::Terminated);
                break;
            }
            OpCode::Add(x, y, z) => run_add_instruction(program, x, y, z),
            OpCode::Multiply(x, y, z) => run_mult_instruction(program, x, y, z),
            OpCode::Input(x) => {
                if has_input(&program) {
                    run_input(program, x);
                } else {
                    program.halt_status = Some(HaltStatus::WaitingInput);
                    break;
                }
            }
            OpCode::Output(x) => run_output(program, x),
            OpCode::JumpIfTrue(x, y) => run_jump(program, true, x, y),
            OpCode::JumpIfFalse(x, y) => run_jump(program, false, x, y),
            OpCode::LessThan(x, y, z) => run_less_than(program, x, y, z),
            OpCode::Equals(x, y, z) => run_equals(program, x, y, z),
            OpCode::AdjustRelBase(x) => run_adjust_relative_base(program, x),
        }
    }

    program
}

fn run_add_instruction(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
    op3_mode: ParameterMode,
) {
    if op3_mode == ParameterMode::Immediate {
        panic!("Tried to get a write destination using immediate mode!")
    }

    let operand_1 = get_state(program, program.pointer + 1, op1_mode);
    let operand_2 = get_state(program, program.pointer + 2, op2_mode);
    let destination = get_destination(program, program.pointer + 3, op3_mode);

    trace!(
        "Adding! operand_1: {}, operand_2: {}, destination: {}",
        operand_1,
        operand_2,
        destination
    );

    program.state.insert(destination, operand_1 + operand_2);

    program.pointer += 4;
}

fn run_mult_instruction(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
    op3_mode: ParameterMode,
) {
    let operand_1 = get_state(program, program.pointer + 1, op1_mode);
    let operand_2 = get_state(program, program.pointer + 2, op2_mode);
    let destination = get_destination(program, program.pointer + 3, op3_mode);

    trace!(
        "Multiplying! operand_1: {}, operand_2: {}, destination: {}",
        operand_1,
        operand_2,
        destination
    );

    program.state.insert(destination, operand_1 * operand_2);
    program.pointer += 4;
}

fn run_input(program: &mut Program, op_mode: ParameterMode) {
    let destination = get_destination(program, program.pointer + 1, op_mode);

    let input = get_next_input(program).unwrap();

    trace!(
        "Taking input! destination: {}, input: {}",
        destination,
        input
    );

    program.state.insert(destination, input);

    program.pointer += 2;
}

fn run_output(program: &mut Program, op_mode: ParameterMode) {
    let output = get_state(program, program.pointer + 1, op_mode);

    trace!("Pushing output: {}", output);

    push_output(program, output);

    program.pointer += 2;
}

fn run_jump(
    program: &mut Program,
    jump_if: bool,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
) {
    let operand_1 = get_state(program, program.pointer + 1, op1_mode);
    let operand_2 = get_state(program, program.pointer + 2, op2_mode);

    match operand_1 {
        0 if !jump_if => program.pointer = operand_2,
        x if jump_if && x != 0 => program.pointer = operand_2,
        _ => program.pointer += 3,
    }
}

fn run_less_than(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
    op3_mode: ParameterMode,
) {
    let operand_1 = get_state(program, program.pointer + 1, op1_mode);
    let operand_2 = get_state(program, program.pointer + 2, op2_mode);
    let destination = get_destination(program, program.pointer + 3, op3_mode);

    if operand_1 < operand_2 {
        program.state.insert(destination, 1);
    } else {
        program.state.insert(destination, 0);
    }

    program.pointer += 4;
}

fn run_equals(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
    op3_mode: ParameterMode,
) {
    let operand_1 = get_state(program, program.pointer + 1, op1_mode);
    let operand_2 = get_state(program, program.pointer + 2, op2_mode);
    let destination = get_destination(program, program.pointer + 3, op3_mode);

    if operand_1 == operand_2 {
        program.state.insert(destination, 1);
    } else {
        program.state.insert(destination, 0);
    }

    program.pointer += 4;
}

fn run_adjust_relative_base(program: &mut Program, op_mode: ParameterMode) {
    let operand = get_state(program, program.pointer + 1, op_mode);

    program.relative_base += operand;

    trace!(
        "Adjusting relative base by: {}, new relative base: {}",
        operand,
        program.relative_base
    );

    program.pointer += 2;
}

////////////////////////////////////////////////////////////////
// Parsing

// Opcodes are 2-digit values, then parameter modes for any parameters

fn parse_opcode(opcode: i64) -> OpCode {
    let mut digits = util::digits(opcode);
    digits.reverse();

    let parsed = match digits.first() {
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
            3 => OpCode::Input(parse_mode(digits[2])),
            2 | 1 => OpCode::Input(ParameterMode::Position),
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
            _ => match digits.len() {
                3 => OpCode::AdjustRelBase(parse_mode(digits[2])),
                2 | 1 => OpCode::AdjustRelBase(ParameterMode::Position),
                _ => panic!("Invalid opcode: {}", opcode),
            },
        },
        Some(_) => panic!("Invalid opcode: {}", opcode),
    };

    trace!("parsed opcode \"{}\" as: {:?}", opcode, parsed);

    parsed
}

fn parse_mode(mode: i64) -> ParameterMode {
    match mode {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
        _ => panic!("Unknown parameter mode: {}", mode),
    }
}

fn vec_to_map(vec: Vec<i64>) -> HashMap<i64, i64> {
    let mut hash_state = HashMap::new();

    for (i, val) in vec.iter().enumerate() {
        hash_state.insert(i as i64, val.clone());
    }
    hash_state
}

#[test]
fn day2_intcode_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    assert_eq!(
        run_program(&mut Program::new(vec![1, 0, 0, 0, 99])).state,
        vec_to_map(vec!(2, 0, 0, 0, 99))
    );
    assert_eq!(
        run_program(&mut Program::new(vec![2, 3, 0, 3, 99])).state,
        vec_to_map(vec!(2, 3, 0, 6, 99))
    );
    assert_eq!(
        run_program(&mut Program::new(vec![2, 4, 4, 5, 99, 0])).state,
        vec_to_map(vec!(2, 4, 4, 5, 99, 9801))
    );
    assert_eq!(
        run_program(&mut Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])).state,
        vec_to_map(vec!(30, 1, 1, 4, 2, 5, 6, 0, 99))
    );
}

#[test]
fn basic_io_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut input = Program::new(vec![3, 0, 4, 0, 99]);
    push_input(&mut input, 1);

    trace!("Program: {:?}", input);

    assert_eq!(1, get_next_output(run_program(&mut input)).unwrap());

    input = Program::new(vec![3, 0, 4, 0, 99]);
    push_input(&mut input, 42);

    assert_eq!(42, get_next_output(run_program(&mut input)).unwrap());
}

#[test]
fn opcode_mode_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    assert_eq!(
        1101,
        get_state(
            run_program(&mut Program::new(vec![1101, 100, -1, 4, 0])),
            0,
            ParameterMode::Immediate
        )
    );

    assert_eq!(
        99,
        get_state(
            run_program(&mut Program::new(vec![1002, 4, 3, 4, 33])),
            4,
            ParameterMode::Immediate
        )
    )
}

#[test]
fn four_more_opcodes() {
    let _ = env_logger::builder().is_test(true).try_init();

    // equals
    let mut input = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    push_input(&mut input, 8);
    assert_eq!(1, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    push_input(&mut input, 42);
    assert_eq!(0, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    push_input(&mut input, 42);
    assert_eq!(0, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    push_input(&mut input, 8);
    assert_eq!(1, run_program(&mut input).output[0]);

    // less than
    input = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    push_input(&mut input, 42);
    assert_eq!(0, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    push_input(&mut input, 2);
    assert_eq!(1, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    push_input(&mut input, 42);
    assert_eq!(0, run_program(&mut input).output[0]);

    input = Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    push_input(&mut input, 4);
    assert_eq!(1, run_program(&mut input).output[0]);

    // Jumps
    input = Program::new(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);
    push_input(&mut input, 42);
    assert_eq!(1, run_program(&mut input).output[0]);

    input = Program::new(vec![
        3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
    ]);
    push_input(&mut input, 0);

    assert_eq!(0, run_program(&mut input).output[0]);

    // More complex test
    input = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    push_input(&mut input, 4);
    assert_eq!(999, run_program(&mut input).output[0]);

    input = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    push_input(&mut input, 8);
    assert_eq!(1000, run_program(&mut input).output[0]);

    input = Program::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    push_input(&mut input, 42);
    assert_eq!(1001, run_program(&mut input).output[0]);
}

#[test]
fn relative_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut program = Program::new(vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ]);

    run_program(&mut program);

    for (expected, produced) in vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ]
    .iter()
    .zip((&program).output.iter())
    {
        assert_eq!(expected, produced);
    }
}

#[test]
fn large_numbers_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut program = Program::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);

    run_program(&mut program);

    assert_eq!(
        16,
        util::digits(get_next_output(&mut program).unwrap()).len()
    );

    program = Program::new(vec![104, 1125899906842624, 99]);

    run_program(&mut program);

    assert_eq!(1125899906842624, get_next_output(&mut program).unwrap());
}

#[test]
fn output_queue_test() {
    let mut program = Program::new(vec![]);

    push_output(&mut program, 42);

    assert_eq!(42, get_next_output(&mut program).unwrap());

    push_output(&mut program, 42);
    push_output(&mut program, 105);

    assert_eq!(42, get_next_output(&mut program).unwrap());
    assert_eq!(105, get_next_output(&mut program).unwrap());

    assert!(get_next_output(&mut program).is_none());
}
