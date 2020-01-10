use crate::intcode::{self, Program};
use crate::util::{self, DecreasingRange};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::HashMap;

pub fn part1() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d13.txt"));

    let mut layout = HashMap::new();
    let mut current_position = (0, 0);

    loop {
        let direction: Direction = rand::random();

        intcode::push_input(&mut program, direction_to_input(direction));
        intcode::run_program(&mut program);

        match intcode::get_next_output(&mut program) {
            None => panic!("Got no output from the program..."),
            Some(output) => {
                match parse_status(output) {
                    Status::Oxygen => {
                        layout.insert(current_position, Block::Empty);
                        match direction {
                            Direction::North => {
                                current_position = (current_position.0, current_position.1 + 1)
                            }
                            Direction::South => {
                                current_position = (current_position.0, current_position.1 - 1)
                            }
                            Direction::West => {
                                current_position = (current_position.0 + 1, current_position.1)
                            }
                            Direction::East => {
                                current_position = (current_position.0 - 1, current_position.1)
                            }
                        };
                        layout.insert(current_position, Block::Oxygen);
                        break;
                    }
                    Status::Wall => {
                        match direction {
                            Direction::North => layout
                                .insert((current_position.0, current_position.1 + 1), Block::Wall),
                            Direction::South => layout
                                .insert((current_position.0, current_position.1 - 1), Block::Wall),
                            Direction::West => layout
                                .insert((current_position.0 + 1, current_position.1), Block::Wall),
                            Direction::East => layout
                                .insert((current_position.0 - 1, current_position.1), Block::Wall),
                        };
                    }
                    Status::Moved => {
                        layout.insert(current_position, Block::Empty);
                        match direction {
                            Direction::North => {
                                current_position = (current_position.0, current_position.1 + 1)
                            }
                            Direction::South => {
                                current_position = (current_position.0, current_position.1 - 1)
                            }
                            Direction::West => {
                                current_position = (current_position.0 + 1, current_position.1)
                            }
                            Direction::East => {
                                current_position = (current_position.0 - 1, current_position.1)
                            }
                        };
                        layout.insert(current_position, Block::Droid);
                    }
                };
            }
        }
    }

    draw_space(layout);

    42
}

pub fn part2() -> i64 {
    42
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

// stolen from: https://stackoverflow.com/a/48491021
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            _ => Direction::East,
        }
    }
}

fn direction_to_input(dir: Direction) -> i64 {
    match dir {
        Direction::North => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::East => 4,
    }
}

enum Status {
    Wall,
    Moved,
    Oxygen,
}

#[derive(Debug)]
enum Block {
    Droid,
    Wall,
    Empty,
    Oxygen,
}

fn parse_status(input: i64) -> Status {
    match input {
        0 => Status::Wall,
        1 => Status::Moved,
        2 => Status::Oxygen,
        x => panic!("Unknown status: {}", x),
    }
}

fn draw_space(space: HashMap<(i64, i64), Block>) {
    let max_x = space.keys().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let min_x = space.keys().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let max_y = space.keys().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    let min_y = space.keys().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    let mut output = String::new();

    // We're printing from top to bottom, so start with the max y
    for y in DecreasingRange::new(max_y, min_y - 1, -1) {
        for x in min_x..=max_x {
            match space.get(&(x, y)) {
                None => output.push(' '),
                Some(tile) => match tile {
                    Block::Droid => output.push('D'),
                    Block::Wall => output.push('#'),
                    Block::Empty => output.push('.'),
                    Block::Oxygen => output.push('O'),
                },
            }
        }
        output.push('\n');
    }

    println!("{}", output);
}
