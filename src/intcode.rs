pub struct Program {
    pub input: usize,
    pub output: Vec<usize>,
    pub state: Vec<usize>,
    pub pointer: usize,
}

pub fn run_program(program: &mut Program) -> &mut Program {
    if program.state.is_empty() {
        return program;
    }

    match program.state[program.pointer] {
        99 => program,
        1 => run_add_instruction(program),
        2 => run_mult_instruction(program),
        // 3 => run_save(program, pointer),
        // 4 => run_output(program, pointer),
        _ => program,
    }
}

pub fn run_program_with_io(
    program: &mut Vec<usize>,
    pointer: usize,
    input: usize,
    output: &mut Vec<usize>,
) -> usize {
    42
}

fn run_add_instruction(program: &mut Program) -> &mut Program {
    let operand_1 = program.state[program.state[program.pointer + 1]];
    let operand_2 = program.state[program.state[program.pointer + 2]];
    let destination = program.state[program.pointer + 3];

    program.state[destination] = operand_1 + operand_2;

    program.pointer += 4;

    run_program(program)
}

fn run_mult_instruction(program: &mut Program) -> &mut Program {
    let operand_1 = program.state[program.state[program.pointer + 1]];
    let operand_2 = program.state[program.state[program.pointer + 2]];
    let destination = program.state[program.pointer + 3];

    program.state[destination] = operand_1 * operand_2;
    program.pointer += 4;

    run_program(program)
}

// fn run_save(program: &mut Vec<usize>, pointer: usize) -> &Vec<usize> {
//     program
// }

// fn run_output(program: &mut Vec<usize>, pointer: usize) -> usize {}

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

    // let identity_program = vec![3, 0, 4, 0, 99];

    // assert_eq!(
    //     1,
    //     run_program_with_input(&mut identity_program.clone(), 0, 1)
    // );
}
