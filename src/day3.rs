use std::collections::HashSet;
use std::hash::{Hash, Hasher};

const ORIGIN: Point = Point {
    x: 0,
    y: 0,
    traveled: 0,
};

pub fn part1() -> i64 {
    let paths = load_paths("data/d3.txt");

    let first_points = points_from_path(ORIGIN, paths[0].to_owned());
    let second_points = points_from_path(ORIGIN, paths[1].to_owned());

    let closest = closest_intersection(ORIGIN, first_points, second_points);
    distance_between(&ORIGIN, &closest)
}

pub fn part2() -> i64 {
    let paths = load_paths("data/d3.txt");

    assert_eq!(2, paths.len());

    let first_points = points_from_path(ORIGIN, paths[0].to_owned());
    let second_points = points_from_path(ORIGIN, paths[1].to_owned());

    shortest_taxi_cab_path_to_intersection(first_points, second_points)
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    traveled: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vector {
    direction: Direction,
    distance: i64,
}

fn closest_intersection(
    start: Point,
    mut first_path: HashSet<Point>,
    second_path: HashSet<Point>,
) -> Point {
    first_path.remove(&start);
    *first_path
        .intersection(&second_path)
        .min_by(|x, y| distance_between(&start, x).cmp(&distance_between(&start, y)))
        .unwrap()
}

// sort by x and y, and then walk both sets, keeping
// intersections. Then, find the minimum in both sets by traveled
fn shortest_taxi_cab_path_to_intersection(
    first_path: HashSet<Point>,
    second_path: HashSet<Point>,
) -> i64 {
    let mut intersections = Vec::new();

    for point in first_path.into_iter() {
        if point == ORIGIN {
            continue;
        }
        match second_path.get(&point) {
            None => continue,
            Some(other_point) => intersections.push(point.traveled + other_point.traveled),
        }
    }

    intersections.sort();

    intersections[0]
}

fn distance_between(first: &Point, second: &Point) -> i64 {
    (first.x - second.x).abs() + (first.y - second.y).abs()
}

fn points_from_path(start: Point, path: Vec<Vector>) -> HashSet<Point> {
    let mut points = HashSet::new();

    points.insert(start.to_owned());

    let mut current_x = start.x;
    let mut current_y = start.y;
    let mut traveled = 1;

    for vector in path {
        match vector.direction {
            Direction::Up => {
                for i in 1..=vector.distance {
                    points.insert(Point {
                        x: current_x,
                        y: current_y + i,
                        traveled,
                    });
                    traveled += 1;
                }
                current_y += vector.distance;
            }
            Direction::Down => {
                for i in 1..=vector.distance {
                    points.insert(Point {
                        x: current_x,
                        y: current_y - i,
                        traveled,
                    });
                    traveled += 1;
                }
                current_y -= vector.distance;
            }
            Direction::Right => {
                for i in 1..=vector.distance {
                    points.insert(Point {
                        x: current_x + i,
                        y: current_y,
                        traveled,
                    });
                    traveled += 1;
                }
                current_x += vector.distance;
            }
            Direction::Left => {
                for i in 1..=vector.distance {
                    points.insert(Point {
                        x: current_x - i,
                        y: current_y,
                        traveled,
                    });
                    traveled += 1;
                }
                current_x -= vector.distance;
            }
        }
    }

    points
}

fn parse_path(path: &str) -> Vector {
    let mut chars = path.chars();

    let direction = match chars.next().unwrap() {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'R' => Direction::Right,
        'L' => Direction::Left,
        x => panic!("unknown direction in data: {}", x),
    };

    let distance = chars.collect::<String>().parse::<i64>().unwrap();

    Vector {
        direction,
        distance,
    }
}

fn load_paths(filepath: &str) -> Vec<Vec<Vector>> {
    super::util::lines_from_path(filepath)
        .filter_map(|x| match x {
            Ok(line) => Some(line.split(',').map(|x| parse_path(x)).collect()),
            _ => None,
        })
        .collect()
}

#[test]
fn parsing_test() {
    assert_eq!(
        Vector {
            direction: Direction::Up,
            distance: 87
        },
        parse_path("U87")
    );
}

#[test]
fn points_equality() {
    assert_eq!(
        Point {
            x: 1,
            y: 1,
            traveled: 2
        },
        Point {
            x: 1,
            y: 1,
            traveled: 3
        }
    );
}

#[test]
fn points_to_path_test() {
    let mut points = HashSet::new();

    points.insert(Point {
        x: 0,
        y: 0,
        traveled: 0,
    });
    points.insert(Point {
        x: 1,
        y: 0,
        traveled: 1,
    });
    points.insert(Point {
        x: 2,
        y: 0,
        traveled: 2,
    });
    points.insert(Point {
        x: 2,
        y: 1,
        traveled: 3,
    });
    points.insert(Point {
        x: 2,
        y: 2,
        traveled: 4,
    });

    assert_eq!(
        points,
        points_from_path(
            ORIGIN,
            vec!(
                Vector {
                    direction: Direction::Right,
                    distance: 2
                },
                Vector {
                    direction: Direction::Up,
                    distance: 2
                }
            )
        )
    );
}

#[test]
fn part1_test() {
    assert_eq!(
        159,
        distance_between(
            &ORIGIN,
            &closest_intersection(
                ORIGIN,
                points_from_path(
                    ORIGIN,
                    vec!("R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72")
                        .iter()
                        .map(|x| parse_path(x))
                        .collect::<Vec<Vector>>()
                ),
                points_from_path(
                    ORIGIN,
                    vec!("U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83")
                        .iter()
                        .map(|x| parse_path(x))
                        .collect::<Vec<Vector>>()
                )
            )
        )
    );

    assert_eq!(
        135,
        distance_between(
            &ORIGIN,
            &closest_intersection(
                ORIGIN,
                points_from_path(
                    ORIGIN,
                    vec!(
                        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"
                    )
                    .iter()
                    .map(|x| parse_path(x))
                    .collect::<Vec<Vector>>()
                ),
                points_from_path(
                    ORIGIN,
                    vec!("U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7")
                        .iter()
                        .map(|x| parse_path(x))
                        .collect::<Vec<Vector>>()
                )
            )
        )
    );

    assert_eq!(
        4,
        distance_between(
            &ORIGIN,
            &Point {
                x: 2,
                y: 2,
                traveled: 4
            }
        )
    );
}

#[test]
fn part2_test() {
    assert_eq!(
        610,
        shortest_taxi_cab_path_to_intersection(
            points_from_path(
                ORIGIN,
                vec!("R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72")
                    .iter()
                    .map(|x| parse_path(x))
                    .collect::<Vec<Vector>>()
            ),
            points_from_path(
                ORIGIN,
                vec!("U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83")
                    .iter()
                    .map(|x| parse_path(x))
                    .collect::<Vec<Vector>>()
            )
        )
    );

    assert_eq!(
        410,
        shortest_taxi_cab_path_to_intersection(
            points_from_path(
                ORIGIN,
                vec!("R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51")
                    .iter()
                    .map(|x| parse_path(x))
                    .collect::<Vec<Vector>>()
            ),
            points_from_path(
                ORIGIN,
                vec!("U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7")
                    .iter()
                    .map(|x| parse_path(x))
                    .collect::<Vec<Vector>>()
            )
        )
    );
}

#[test]
fn string_to_points() {
    let points = points_from_path(
        ORIGIN,
        vec!["U12", "D23", "R13", "L41", "U11"]
            .iter()
            .map(|x| parse_path(x))
            .collect::<Vec<Vector>>(),
    );
    // Duplicates....
    assert_eq!(76, points.len());

    // the first time past the point is retained in the set
    assert_eq!(
        1,
        points
            .get(&Point {
                x: 0,
                y: 1,
                traveled: 1
            })
            .unwrap()
            .traveled
    );
}

#[test]
fn distance_traveled_test() {
    // Travel a total distance of 100, should be...
    assert_eq!(12 + 23 + 13 + 41 + 11, 100);

    let test_points = points_from_path(
        ORIGIN,
        vec!["U12", "D23", "R13", "L41", "U11"]
            .iter()
            .map(|x| parse_path(x))
            .collect::<Vec<Vector>>(),
    );

    assert_eq!(
        12,
        test_points
            .get(&Point {
                x: 0,
                y: 12,
                traveled: 12
            })
            .unwrap()
            .traveled
    );

    let mut test_points = test_points.into_iter().collect::<Vec<Point>>();
    test_points.sort_by(|x, y| x.traveled.cmp(&y.traveled));

    assert!(test_points.len() < 100);

    assert_eq!(0, test_points[0].traveled);

    assert_eq!(100, test_points.last().unwrap().traveled);
}

#[test]
fn hash_equality_test() {
    let mut hash_set = HashSet::new();

    hash_set.insert(Point {
        x: 0,
        y: 0,
        traveled: 0,
    });
    hash_set.insert(Point {
        x: 0,
        y: 0,
        traveled: 1,
    });

    assert_eq!(
        0,
        hash_set
            .get(&Point {
                x: 0,
                y: 0,
                traveled: 0
            })
            .unwrap()
            .traveled
    );
}
