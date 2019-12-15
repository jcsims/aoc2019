use crate::intcode;
use crate::intcode::Program;
use crate::util;
use log::trace;

pub fn part1() -> i64 {
    42
}

pub fn part2() -> i64 {
    42
}

#[derive(Debug, PartialEq, Eq)]
enum Space {
    //'.'
    Empty,
    // '#'
    Asteroid,
}

fn parse(space: &str) -> Vec<Vec<Space>> {
    let mut parsed = Vec::new();
    for line in space.lines() {
        let mut parsed_line = Vec::new();
        for c in line.trim().chars() {
            let item = match c {
                '.' => Space::Empty,
                '#' => Space::Asteroid,
                x => panic!("Unknown character: {}", x),
            };
            parsed_line.push(item);
        }
        parsed.push(parsed_line);
    }
    parsed
}

#[test]
fn space_parsing_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    let field = "
.#..#
.....
#####
....#
...##"
        .trim();

    assert_eq!(
        vec![
            vec![
                Space::Empty,
                Space::Asteroid,
                Space::Empty,
                Space::Empty,
                Space::Asteroid
            ],
            vec![
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty
            ],
            vec![
                Space::Asteroid,
                Space::Asteroid,
                Space::Asteroid,
                Space::Asteroid,
                Space::Asteroid
            ],
            vec![
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Asteroid
            ],
            vec![
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Asteroid,
                Space::Asteroid
            ],
        ],
        parse(field)
    );
}
