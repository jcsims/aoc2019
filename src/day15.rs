use crate::intcode::{self, Program};
use crate::util::{
    self,
    //DecreasingRange
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

pub fn part1() -> i64 {
    let mut program = Program::new(util::comma_separated_to_vec("data/d15.txt"));

    let mut layout = HashMap::new();
    let mut current_position = (0, 0);

    let mut branches_to_explore = Vec::new();
    let mut directions = Vec::new();

    directions.push(Direction::North);

    loop {
        let direction = match directions.pop() {
            None => {
                println!("Didn't have a direction to move in, so using a random one");
                rand::random()
            }
            Some(direction) => direction,
        };

        intcode::push_input(&mut program, direction_to_input(direction));
        intcode::run_program(&mut program);

        match intcode::get_next_output(&mut program) {
            None => panic!("Got no output from the program..."),
            Some(output) => {
                let new_position = move_in_direction(current_position, direction);

                match parse_status(output) {
                    Status::Oxygen => {
                        layout.insert(current_position, Block::Empty(true));
                        layout.insert(new_position, Block::Oxygen);
                        break;
                    }
                    Status::Wall => {
                        layout.insert(new_position, Block::Wall);
                    }
                    Status::Moved => {
                        layout.insert(current_position, Block::Empty(true));
                        layout.insert(new_position, Block::Droid);
                        current_position = new_position;
                    }
                };
            }
        }

        look_around(
            &mut program,
            &mut layout,
            &mut directions,
            &mut branches_to_explore,
            current_position,
        );
    }

    //draw_space(&layout);

    // Get the length of the path to the oxygen
    directions.clear();

    push_path(&layout, &mut directions, current_position, (0, 0));

    // The path gets us to the node just before the goal. This could
    // be changed fairly easily, but I didn't yet...
    (directions.len() + 1) as i64
}

pub fn part2() -> i64 {
    42
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

fn opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        Direction::East => Direction::West,
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
    // Have we explored this block yet?
    Empty(bool),
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

// fn draw_space(space: &HashMap<(i64, i64), Block>) {
//     let max_x = space.keys().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
//     let min_x = space.keys().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
//     let max_y = space.keys().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
//     let min_y = space.keys().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

//     let mut output = String::new();

//     // We're printing from top to bottom, so start with the max y
//     for y in DecreasingRange::new(max_y, min_y - 1, -1) {
//         for x in min_x..=max_x {
//             if y == 0 && x == 0 {
//                 output.push('*');
//             } else {
//                 match space.get(&(x, y)) {
//                     None => output.push(' '),
//                     Some(tile) => match tile {
//                         Block::Droid => output.push('D'),
//                         Block::Wall => output.push('#'),
//                         Block::Empty(_) => output.push('.'),
//                         Block::Oxygen => output.push('O'),
//                     },
//                 }
//             }
//         }
//         output.push('\n');
//     }

//     println!("{}", output);
// }

fn look_around(
    program: &mut Program,
    layout: &mut HashMap<(i64, i64), Block>,
    directions: &mut Vec<Direction>,
    branches_to_explore: &mut Vec<((i64, i64), Direction)>,
    current_position: (i64, i64),
) {
    look_in_direction(program, layout, current_position, Direction::North);
    look_in_direction(program, layout, current_position, Direction::South);
    look_in_direction(program, layout, current_position, Direction::East);
    look_in_direction(program, layout, current_position, Direction::West);

    match layout.get(&move_in_direction(current_position, Direction::North)) {
        // Nothing to see here, move along....
        Some(Block::Wall) | Some(Block::Empty(true)) => (),

        _ => {
            branches_to_explore.push((current_position, Direction::North));
        }
    }

    match layout.get(&move_in_direction(current_position, Direction::South)) {
        // Nothing to see here, move along....
        Some(Block::Wall) | Some(Block::Empty(true)) => (),
        _ => {
            branches_to_explore.push((current_position, Direction::South));
        }
    }

    match layout.get(&move_in_direction(current_position, Direction::West)) {
        // Nothing to see here, move along....
        Some(Block::Wall) | Some(Block::Empty(true)) => (),
        _ => {
            branches_to_explore.push((current_position, Direction::West));
        }
    }

    match layout.get(&move_in_direction(current_position, Direction::East)) {
        // Nothing to see here, move along....
        Some(Block::Wall) | Some(Block::Empty(true)) => (),
        _ => {
            branches_to_explore.push((current_position, Direction::East));
        }
    }

    if !directions.is_empty() {
        // We're already on a path somewhere, keep going
        return;
    }

    loop {
        // Find another branch to explore
        match branches_to_explore.pop() {
            None => {
                println!("No branches to explore, moving randomly!");
                directions.push(rand::random());
                break;
            }
            Some((position, direction)) => {
                let target = move_in_direction(position, direction);

                match layout.get(&target) {
                    Some(Block::Empty(false)) | Some(Block::Oxygen) =>
                    // We haven't explored this yet
                    {
                        if position == current_position {
                            directions.push(direction);
                        } else {
                            push_path(&layout, directions, current_position, position);
                        }
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }
}

fn look_in_direction(
    program: &mut Program,
    layout: &mut HashMap<(i64, i64), Block>,
    current_position: (i64, i64),
    direction: Direction,
) {
    if layout
        .get(&move_in_direction(current_position, direction))
        .is_none()
    {
        let moved;
        intcode::push_input(program, direction_to_input(direction));
        intcode::run_program(program);

        match intcode::get_next_output(program) {
            None => panic!("Got no output from the program..."),
            Some(output) => {
                let new_position = move_in_direction(current_position, direction);

                match parse_status(output) {
                    Status::Oxygen => {
                        layout.insert(new_position, Block::Oxygen);
                        moved = true;
                    }
                    Status::Wall => {
                        layout.insert(new_position, Block::Wall);
                        moved = false;
                    }
                    Status::Moved => {
                        layout.insert(new_position, Block::Empty(false));
                        moved = true;
                    }
                };
            }
        };

        // Need to move back
        if moved {
            intcode::push_input(program, direction_to_input(opposite_direction(direction)));
            intcode::run_program(program);
            intcode::get_next_output(program);
        }
    }
}

#[derive(Clone)]
struct Path {
    position: (i64, i64),
    directions: Vec<Direction>,
    goal_estimate: i64,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_cost = self.goal_estimate + self.directions.len() as i64;
        let other_cost = other.goal_estimate + other.directions.len() as i64;
        self_cost.cmp(&other_cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        (self.directions.len() == other.directions.len())
            && (self.goal_estimate == other.goal_estimate)
    }
}

impl Eq for Path {}

fn straight_line_cost(position: (i64, i64), target: (i64, i64)) -> i64 {
    (position.0 - target.0).abs() + (position.1 - target.1).abs()
}

fn move_in_direction(position: (i64, i64), direction: Direction) -> (i64, i64) {
    match direction {
        Direction::North => (position.0, position.1 + 1),
        Direction::South => (position.0, position.1 - 1),
        Direction::East => (position.0 + 1, position.1),
        Direction::West => (position.0 - 1, position.1),
    }
}

fn neighbors(position: (i64, i64)) -> Vec<(Direction, (i64, i64))> {
    vec![
        (
            Direction::North,
            move_in_direction(position, Direction::North),
        ),
        (
            Direction::South,
            move_in_direction(position, Direction::South),
        ),
        (
            Direction::East,
            move_in_direction(position, Direction::East),
        ),
        (
            Direction::West,
            move_in_direction(position, Direction::West),
        ),
    ]
}

// Used to explore intermediate paths, as well as get the final
// distance from the starting point to the oxygen. This is some
// approximation of A*
fn push_path(
    layout: &HashMap<(i64, i64), Block>,
    directions: &mut Vec<Direction>,
    current_position: (i64, i64),
    target_position: (i64, i64),
) {
    let mut to_explore = BinaryHeap::new();
    let mut shortest_paths: HashMap<(i64, i64), Path> = HashMap::new();

    // Using reverse for a min-heap
    to_explore.push(Reverse(Path {
        position: current_position,
        directions: Vec::new(),
        goal_estimate: straight_line_cost(current_position, target_position),
    }));

    loop {
        match to_explore.pop() {
            None => panic!(
                "No path remaining to point: {:?} from point: {:?}",
                target_position, current_position
            ),
            Some(Reverse(mut node)) if node.position == target_position => {
                node.directions.reverse();
                directions.append(&mut node.directions);
                break;
            }
            Some(Reverse(node)) => {
                // check out our neighbors, and see if we've
                // discovered a shorter path to any of them
                // If we haven't seen this node yet, add it to the
                // queue to explore
                let neighbor_path_cost = node.directions.len() + 1;
                for neighbor in neighbors(node.position) {
                    match layout.get(&neighbor.1) {
                        None | Some(Block::Wall) => {
                            // don't know anything about a path, so
                            // can't use it
                            continue;
                        }
                        _ => {
                            match shortest_paths.get(&neighbor.1) {
                                Some(ref path) if path.directions.len() >= neighbor_path_cost => {
                                    // We already have a path that is at least as
                                    // short, move on to the next
                                    continue;
                                }
                                _ => {
                                    // There either is no path to this node yet,
                                    // or we've found a cheaper path!
                                    let mut new_directions = node.directions.clone();
                                    new_directions.push(neighbor.0);

                                    let new_path = Path {
                                        position: neighbor.1,
                                        goal_estimate: straight_line_cost(
                                            neighbor.1,
                                            target_position,
                                        ),
                                        directions: new_directions,
                                    };

                                    shortest_paths.insert(neighbor.1, new_path.clone());
                                    to_explore.push(Reverse(new_path));
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[test]
fn a_star_test() {
    let mut layout = HashMap::new();
    let mut directions = Vec::new();
    let current_position = (0, 0);
    let target_position = (2, 2);

    // Our test map:
    // 3  ...
    // 2 #.#O#
    // 1 #.#.
    // 0 #*..
    //  -1012

    // third row
    layout.insert((0, 3), Block::Empty(true));
    layout.insert((1, 3), Block::Empty(true));
    layout.insert((2, 3), Block::Empty(true));

    // second row
    layout.insert((-1, 2), Block::Wall);
    layout.insert((0, 2), Block::Empty(true));
    layout.insert((1, 2), Block::Wall);
    layout.insert((2, 2), Block::Oxygen);
    layout.insert((3, 2), Block::Wall);

    // first row
    layout.insert((-1, 1), Block::Wall);
    layout.insert((0, 1), Block::Empty(true));
    layout.insert((1, 1), Block::Wall);
    layout.insert((2, 1), Block::Empty(true));

    // zeroth row
    layout.insert((-1, 0), Block::Wall);
    layout.insert((0, 0), Block::Empty(true));
    layout.insert((1, 0), Block::Empty(true));
    layout.insert((2, 0), Block::Empty(true));

    push_path(&layout, &mut directions, current_position, target_position);

    // Vecs as a stack pop off the end
    assert_eq!(
        vec![
            Direction::North,
            Direction::North,
            Direction::East,
            Direction::East,
        ],
        directions
    );
}
