use crate::intcode::{self, HaltStatus, Program};
use crate::util::{self, DecreasingRange};
use log::trace;
use std::collections::HashMap;

pub fn part1() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d11.txt"));

    let mut current_direction = Direction::Up;
    let mut current_location = Point { x: 0, y: 0 };

    let mut hull = HashMap::new();

    loop {
        match hull.get(&current_location) {
            None => intcode::push_input(&mut program, unparse_color(&Color::Black)),
            Some(color) => intcode::push_input(&mut program, unparse_color(&color)),
        }

        intcode::run_program(&mut program);

        // get the output, add it to the set
        let new_color = parse_color(intcode::get_next_output(&mut program).unwrap());

        transition_direction(
            &mut current_direction,
            &parse_direction(intcode::get_next_output(&mut program).unwrap()),
        );

        hull.insert(current_location, new_color);

        transition_point(&mut current_location, &current_direction);

        match program.halt_status {
            Some(HaltStatus::Terminated) => break,
            _ => continue,
        }
    }

    hull.len() as i64
}

pub fn part2() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d11.txt"));

    let mut current_direction = Direction::Up;
    let mut current_location = Point { x: 0, y: 0 };

    let mut hull = HashMap::new();

    // starting on white now
    intcode::push_input(&mut program, unparse_color(&Color::White));

    loop {
        match hull.get(&current_location) {
            None => intcode::push_input(&mut program, unparse_color(&Color::Black)),
            Some(color) => intcode::push_input(&mut program, unparse_color(&color)),
        }

        intcode::run_program(&mut program);

        // get the output, add it to the set
        let new_color = parse_color(intcode::get_next_output(&mut program).unwrap());

        transition_direction(
            &mut current_direction,
            &parse_direction(intcode::get_next_output(&mut program).unwrap()),
        );

        hull.insert(current_location, new_color);

        transition_point(&mut current_location, &current_direction);

        match program.halt_status {
            Some(HaltStatus::Terminated) => break,
            _ => continue,
        }
    }

    trace!("hull: {:?}", hull);

    let max_x = hull.keys().map(|x| x.x).max().unwrap();
    let min_x = hull.keys().map(|x| x.x).min().unwrap();
    let max_y = hull.keys().map(|x| x.y).max().unwrap();
    let min_y = hull.keys().map(|x| x.y).min().unwrap();

    let mut output = String::new();

    // We're printing from top to bottom, so start with the max y
    for y in DecreasingRange::new(max_y, min_y - 1, -1) {
        for x in min_x..=max_x {
            match hull.get(&Point { x, y }) {
                None => output.push(' '),
                Some(color) => match color {
                    Color::Black => output.push(' '),
                    Color::White => output.push('#'),
                },
            }
        }
        output.push('\n');
    }

    assert_eq!(
        output.trim(),
        "
 #    ###  #### ####  ##   ##  #### ####   
 #    #  #    # #    #  # #  # #    #      
 #    #  #   #  ###  #    #    ###  ###    
 #    ###   #   #    #    # ## #    #      
 #    # #  #    #    #  # #  # #    #      
 #### #  # #### ####  ##   ### #    ####   
"
        .trim()
    );

    42
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    // 0, every panel starts black
    Black,
    // 1
    White,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_color(color: i64) -> Color {
    match color {
        0 => Color::Black,
        1 => Color::White,
        x => panic!("Unknown paint color {}", x),
    }
}

fn unparse_color(color: &Color) -> i64 {
    match color {
        Color::Black => 0,
        Color::White => 1,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    // 0
    Left,
    // 1
    Right,
    Up,
    Down,
}

fn parse_direction(direction: i64) -> Direction {
    match direction {
        0 => Direction::Left,
        1 => Direction::Right,
        x => panic!("Unknown direction color {}", x),
    }
}

fn transition_direction(current: &mut Direction, turn: &Direction) {
    match turn {
        Direction::Left => match current {
            Direction::Up => *current = Direction::Left,
            Direction::Left => *current = Direction::Down,
            Direction::Down => *current = Direction::Right,
            Direction::Right => *current = Direction::Up,
        },
        Direction::Right => match current {
            Direction::Up => *current = Direction::Right,
            Direction::Right => *current = Direction::Down,
            Direction::Down => *current = Direction::Left,
            Direction::Left => *current = Direction::Up,
        },
        x => panic!("Invalid direction: {:?}", x),
    };
}

fn transition_point(current: &mut Point, new_direction: &Direction) {
    match new_direction {
        Direction::Down => current.y -= 1,
        Direction::Left => current.x -= 1,
        Direction::Right => current.x += 1,
        Direction::Up => current.y += 1,
    }
}

// After the robot turns, it should always move forward exactly one
// panel. The robot starts facing up.

// let _ = env_logger::builder().is_test(true).try_init();
