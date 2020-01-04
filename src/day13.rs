use crate::util::{self, DecreasingRange};
//use log::trace;
use crate::intcode::{self, Program};
use std::collections::HashMap;
use std::fmt;

pub fn part1() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d13.txt"));

    intcode::run_program(&mut program);

    parse_draw_instructions(&mut program)
        .values()
        .filter(|x| **x == Tile::Block)
        .count() as i64
}

pub fn part2() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d13.txt"));

    // Free play!
    intcode::set_state(&mut program, 0, 2);
    // Get the initial board state
    intcode::run_program(&mut program);

    let screen = parse_draw_instructions(&mut program);

    let width = (screen
        .iter()
        .max_by(|x, y| (x.0).0.cmp(&(y.0).0))
        .unwrap()
        .0)
        .0;
    let height = (screen
        .iter()
        .max_by(|x, y| (x.0).1.cmp(&(y.0).1))
        .unwrap()
        .0)
        .1;

    loop {

        print_screen(screen, height, width);

        if intcode::is_terminated(&program) {
            get
        })

    }

    42
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,            // 0
    Wall,             // 1
    Block,            // 2
    HorizontalPaddle, // 3
    Ball,             //4
}

// unused?
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Wall => write!(f, "#"),
            Tile::Block => write!(f, "0"),
            Tile::HorizontalPaddle => write!(f, "-"),
            Tile::Ball => write!(f, "."),
        }
    }
}

fn tile_to_char(tile: &Tile) -> char {
    match tile {
        Tile::Empty => ' ',
        Tile::Wall => '#',
        Tile::Block => '0',
        Tile::HorizontalPaddle => '-',
        Tile::Ball => '.',
    }
}

fn print_screen(screen: HashMap<(i64, i64), Tile>, height: i64, width: i64) {
    let mut output = String::new();

    // We're printing from top to bottom, so start with the max y
    for y in DecreasingRange::new(height, 0, -1) {
        for x in 0..=width {
            match screen.get(&(x, y)) {
                None => panic!("no point defined for ({}, {})", x, y),
                Some(tile) => output.push(tile_to_char(tile)),
            }
        }
        output.push('\n');
    }

    println!("{}", output);
}

fn parse_tile(tile_id: i64) -> Tile {
    match tile_id {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::HorizontalPaddle,
        4 => Tile::Ball,
        x => panic!("Unknown tile_id: {}", x),
    }
}

fn parse_draw_instructions(program: &mut Program) -> HashMap<(i64, i64), Tile> {
    let mut screen = HashMap::new();

    loop {
        if let Some(x_position) = intcode::get_next_output(program) {
            if let Some(y_position) = intcode::get_next_output(program) {
                if let Some(tile_id) = intcode::get_next_output(program) {
                    screen.insert((x_position, y_position), parse_tile(tile_id));
                } else {
                    panic!("Missing tile id for x- and y-coordinate pair");
                }
            } else {
                panic!("missing y coordinate for given x coordinate");
            }
        } else {
            break;
        }
    }

    screen
}

fn get_joystick_input() -> i64 {
     match input {}
 }
