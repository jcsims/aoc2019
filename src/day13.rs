use crate::intcode::{self, Program};
use crate::util;
//use log::trace;
use std::collections::HashMap;
use std::fmt;
//use std::io::{self, Write};

pub fn part1() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d13.txt"));

    intcode::run_program(&mut program);

    let mut screen = HashMap::new();
    let mut _score = 0;
    let mut _ball_x = 0;
    let mut _paddle_x = 0;

    parse_draw_instructions(
        &mut screen,
        &mut program,
        &mut _score,
        &mut _ball_x,
        &mut _paddle_x,
    );

    screen.values().filter(|x| **x == Tile::Block).count() as i64
}

pub fn part2() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d13.txt"));

    // Free play!
    intcode::set_state(&mut program, 0, 2);
    // Get the initial board state
    intcode::run_program(&mut program);

    let mut score = 0;
    let mut screen = HashMap::new();
    let mut last_x = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    parse_draw_instructions(
        &mut screen,
        &mut program,
        &mut score,
        &mut ball_x,
        &mut paddle_x,
    );

    // let width = (screen
    //     .iter()
    //     .max_by(|x, y| (x.0).0.cmp(&(y.0).0))
    //     .unwrap()
    //     .0)
    //     .0;
    // let height = (screen
    //     .iter()
    //     .max_by(|x, y| (x.0).1.cmp(&(y.0).1))
    //     .unwrap()
    //     .0)
    //     .1;

    loop {
        //print_screen(&screen, height, width, score);

        if intcode::is_terminated(&program) {
            break;
        }

        // Game's not over, determine the next move
        intcode::push_input(&mut program, paddle_direction(last_x, ball_x, paddle_x));

        // track where it was before we step again
        last_x = ball_x;

        intcode::run_program(&mut program);

        parse_draw_instructions(
            &mut screen,
            &mut program,
            &mut score,
            &mut ball_x,
            &mut paddle_x,
        );
    }

    score
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
            Tile::Ball => write!(f, "·"),
        }
    }
}

// fn tile_to_char(tile: &Tile) -> char {
//     match tile {
//         Tile::Empty => ' ',
//         Tile::Wall => '#',
//         Tile::Block => '0',
//         Tile::HorizontalPaddle => '-',
//         Tile::Ball => '.',
//     }
// }

// fn print_screen(screen: &HashMap<(i64, i64), Tile>, height: i64, width: i64, score: i64) {
//     let mut output = String::new();

//     // We're printing from top to bottom, so start with the max y
//     for y in DecreasingRange::new(height, 0, -1) {
//         for x in 0..=width {
//             match screen.get(&(x, y)) {
//                 None => panic!("no point defined for ({}, {})", x, y),
//                 Some(tile) => output.push(tile_to_char(tile)),
//             }
//         }
//         output.push('\n');
//     }

//     println!("{}", output);
//     println!("Score: {}", score);
// }

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

fn parse_draw_instructions(
    screen: &mut HashMap<(i64, i64), Tile>,
    program: &mut Program,
    score: &mut i64,
    ball_x: &mut i64,
    paddle_x: &mut i64,
) {
    loop {
        match intcode::get_next_output(program) {
            Some(-1) => match intcode::get_next_output(program) {
                Some(0) => match intcode::get_next_output(program) {
                    Some(new_score) => *score = new_score,
                    None => panic!("malformed score instruction"),
                },

                _ => panic!("malformed score instruction"),
            },
            Some(x_position) => match intcode::get_next_output(program) {
                Some(y_position) => match intcode::get_next_output(program) {
                    Some(tile_id) => {
                        let tile = parse_tile(tile_id);
                        if tile == Tile::Ball {
                            *ball_x = x_position;
                        } else if tile == Tile::HorizontalPaddle {
                            *paddle_x = x_position;
                        }
                        screen.insert((x_position, y_position), parse_tile(tile_id));
                        ()
                    }
                    None => panic!("Missing tile id for x- and y-coordinate pair"),
                },
                None => panic!("missing y coordinate for given x coordinate"),
            },
            None => break,
        }
    }
}

// fn get_joystick_input() -> i64 {
//     print!("move joystick (a|s|d): ");
//     io::stdout().flush().unwrap();

//     let mut input = String::new();

//     match io::stdin().read_line(&mut input) {
//         Ok(_) => match input.trim() {
//             "a" => -1,
//             "s" => 0,
//             "d" => 1,
//             // default to centered
//             "" => 0,
//             x => {
//                 println!("Invalid input! {}", x);
//                 get_joystick_input()
//             }
//         },
//         Err(_) => {
//             println!("Error on input!");
//             get_joystick_input()
//         }
//     }
// }

fn paddle_direction(last_x: i64, ball_x: i64, paddle_x: i64) -> i64 {
    if (ball_x - paddle_x) < -1 {
        // too far, move closer
        -1
    } else if (ball_x - paddle_x) > 1 {
        // too far, move closer
        1
    } else {
        // we're close - which direction do we move?
        if last_x < ball_x {
            // moving to the right
            if ball_x > paddle_x {
                // keep moving the paddle to the right....
                1
            } else if ball_x == paddle_x {
                // stay here
                0
            } else {
                // move closer to it
                -1
            }
        } else if last_x > ball_x {
            // moving to the left
            if ball_x < paddle_x {
                // keep moving the paddle to the left....
                -1
            } else if ball_x == paddle_x {
                // stay here
                0
            } else {
                // move closer to it
                1
            }
        } else {
            // it's moving straight up...
            if ball_x < paddle_x {
                -1
            } else if ball_x > paddle_x {
                1
            } else {
                // stay right here....
                0
            }
        }
    }
}
