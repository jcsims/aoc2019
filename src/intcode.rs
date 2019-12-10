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

enum OpCode {
    Add(ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode),
    Write,
    Output(ParameterMode),
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
