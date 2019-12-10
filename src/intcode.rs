use crate::util;

#[derive(Debug)]
pub struct Program {
    pub input: i32,
    pub output: Vec<i32>,
    pub state: Vec<i32>,
    pub pointer: usize,
}

enum ParameterMode {
    Immediate,
    Position,
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

pub fn empty_program() -> Program {
    Program {
        input: 0,
        output: Vec::new(),
        state: Vec::new(),
        pointer: 0,
    }
}

pub fn run_program(program: &mut Program) -> &mut Program {
    if program.state.is_empty() {
        return program;
    }

    match parse_opcode(program.state[program.pointer]) {
        OpCode::Stop => program,
        OpCode::Add(x, y) => run_add_instruction(program, x, y),
        OpCode::Multiply(x, y) => run_mult_instruction(program, x, y),
        OpCode::Write => run_save(program),
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
    // println!("Adding! state: {:?}", program);
    let operand_1 = match op1_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 2] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 2],
    };
    let destination = program.state[program.pointer + 3] as usize;

    program.state[destination] = operand_1 + operand_2;

    program.pointer += 4;

    run_program(program)
}

fn run_mult_instruction(
    program: &mut Program,
    op1_mode: ParameterMode,
    op2_mode: ParameterMode,
) -> &mut Program {
    // println!("Multiplying! state: {:?}", program);
    let operand_1 = match op1_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
    };
    let operand_2 = match op2_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 2] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 2],
    };
    let destination = program.state[program.pointer + 3] as usize;

    program.state[destination] = operand_1 * operand_2;
    program.pointer += 4;

    run_program(program)
}

fn run_save(program: &mut Program) -> &mut Program {
    // println!("Taking input! state: {:?}", program);
    let input_pointer = program.state[program.pointer + 1] as usize;
    program.state[input_pointer] = program.input;

    program.pointer += 2;

    run_program(program)
}

fn run_output(program: &mut Program, op_mode: ParameterMode) -> &mut Program {
    // println!("Pushing output! state: {:?}", program);
    let output = match op_mode {
        ParameterMode::Position => program.state[program.state[program.pointer + 1] as usize],
        ParameterMode::Immediate => program.state[program.pointer + 1],
    };

    program.output.push(output);

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
        run_program(&mut Program {
            state: vec!(1, 0, 0, 0, 99),
            pointer: 0,
            input: 0,
            output: Vec::new()
        })
        .state,
        vec!(2, 0, 0, 0, 99)
    );
    assert_eq!(
        run_program(&mut Program {
            state: vec!(2, 3, 0, 3, 99),
            pointer: 0,
            input: 0,
            output: Vec::new()
        })
        .state,
        vec!(2, 3, 0, 6, 99)
    );
    assert_eq!(
        run_program(&mut Program {
            state: vec!(2, 4, 4, 5, 99, 0),
            pointer: 0,
            input: 0,
            output: Vec::new()
        })
        .state,
        vec!(2, 4, 4, 5, 99, 9801)
    );
    assert_eq!(
        run_program(&mut Program {
            state: vec!(1, 1, 1, 4, 99, 5, 6, 0, 99),
            pointer: 0,
            input: 0,
            output: Vec::new()
        })
        .state,
        vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
    );
}

#[test]
fn basic_io_test() {
    assert_eq!(
        1,
        run_program(&mut Program {
            state: vec!(3, 0, 4, 0, 99),
            pointer: 0,
            input: 1,
            output: Vec::new()
        })
        .output[0]
    );

    assert_eq!(
        42,
        run_program(&mut Program {
            state: vec!(3, 0, 4, 0, 99),
            pointer: 0,
            input: 42,
            output: Vec::new()
        })
        .output[0]
    );
}

#[test]
fn opcode_mode_test() {
    assert_eq!(
        1101,
        run_program(&mut Program {
            state: vec![1101, 100, -1, 4, 0],
            pointer: 0,
            input: 0,
            output: Vec::new(),
        })
        .state[0]
    );

    assert_eq!(
        99,
        run_program(&mut Program {
            state: vec![1002, 4, 3, 4, 33],
            pointer: 0,
            input: 0,
            output: Vec::new(),
        })
        .state[4]
    )
}

#[test]
fn four_more_opcodes() {
    // equals
    assert_eq!(
        1,
        run_program(&mut Program {
            state: vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            pointer: 0,
            input: 8,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        0,
        run_program(&mut Program {
            state: vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            pointer: 0,
            input: 42,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        0,
        run_program(&mut Program {
            state: vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            pointer: 0,
            input: 42,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        1,
        run_program(&mut Program {
            state: vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            pointer: 0,
            input: 8,
            output: Vec::new(),
        })
        .output[0]
    );

    // less than
    assert_eq!(
        0,
        run_program(&mut Program {
            state: vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            pointer: 0,
            input: 42,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        1,
        run_program(&mut Program {
            state: vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            pointer: 0,
            input: 2,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        0,
        run_program(&mut Program {
            state: vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            pointer: 0,
            input: 42,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        1,
        run_program(&mut Program {
            state: vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
            pointer: 0,
            input: 4,
            output: Vec::new(),
        })
        .output[0]
    );

    // Jumps
    assert_eq!(
        1,
        run_program(&mut Program {
            state: vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            pointer: 0,
            input: 42,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        0,
        run_program(&mut Program {
            state: vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            pointer: 0,
            input: 0,
            output: Vec::new(),
        })
        .output[0]
    );

    // More complex test
    assert_eq!(
        999,
        run_program(&mut Program {
            state: vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            pointer: 0,
            input: 4,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        1000,
        run_program(&mut Program {
            state: vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            pointer: 0,
            input: 8,
            output: Vec::new(),
        })
        .output[0]
    );

    assert_eq!(
        1001,
        run_program(&mut Program {
            state: vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            pointer: 0,
            input: 42,
            output: Vec::new(),
        })
        .output[0]
    );
}
