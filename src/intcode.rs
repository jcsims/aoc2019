#[derive(Debug)]
pub struct Program {
    pub input: usize,
    pub output: Vec<usize>,
    pub state: Vec<usize>,
    pub pointer: usize,
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

    match program.state[program.pointer] {
        99 => program,
        1 => run_add_instruction(program),
        2 => run_mult_instruction(program),
        3 => run_save(program),
        4 => run_output(program),
        _ => program,
    }
}

fn run_add_instruction(program: &mut Program) -> &mut Program {
    println!("Adding! state: {:?}", program);
    let operand_1 = program.state[program.state[program.pointer + 1]];
    let operand_2 = program.state[program.state[program.pointer + 2]];
    let destination = program.state[program.pointer + 3];

    program.state[destination] = operand_1 + operand_2;

    program.pointer += 4;

    run_program(program)
}

fn run_mult_instruction(program: &mut Program) -> &mut Program {
    println!("Multiplying! state: {:?}", program);
    let operand_1 = program.state[program.state[program.pointer + 1]];
    let operand_2 = program.state[program.state[program.pointer + 2]];
    let destination = program.state[program.pointer + 3];

    program.state[destination] = operand_1 * operand_2;
    program.pointer += 4;

    run_program(program)
}

fn run_save(program: &mut Program) -> &mut Program {
    println!("Taking input! state: {:?}", program);
    let input_pointer = program.state[program.pointer + 1];
    program.state[input_pointer] = program.input;

    program.pointer += 2;

    run_program(program)
}

fn run_output(program: &mut Program) -> &mut Program {
    println!("Pushing output! state: {:?}", program);
    let output_pointer = program.state[program.pointer + 1];
    program.output.push(program.state[output_pointer]);

    program.pointer += 2;

    run_program(program)
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
