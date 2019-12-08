use std::collections::HashSet;

pub fn part1() -> i32 {
    let paths = load_paths("data/d3.txt");

    let origin = Point { x: 0, y: 0 };

    println!(
        "{:?}",
        points_from_path(
            origin,
            vec!("R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72")
                .iter()
                .map(|x| parse_path(x))
                .collect::<Vec<Vector>>()
        )
    );

    assert_eq!(
        159,
        distance_between(
            &origin,
            &closest_intersection(
                origin,
                points_from_path(
                    origin,
                    vec!("R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72")
                        .iter()
                        .map(|x| parse_path(x))
                        .collect::<Vec<Vector>>()
                ),
                points_from_path(
                    origin,
                    vec!("U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83")
                        .iter()
                        .map(|x| parse_path(x))
                        .collect::<Vec<Vector>>()
                )
            )
        )
    );

    assert_eq!(
        Vector {
            direction: Direction::Up,
            distance: 87
        },
        parse_path("U87")
    );

    assert_eq!(Point { x: 1, y: 1 }, Point { x: 1, y: 1 });

    let mut points = HashSet::new();

    points.insert(Point { x: 0, y: 0 });
    points.insert(Point { x: 1, y: 0 });
    points.insert(Point { x: 2, y: 0 });
    points.insert(Point { x: 2, y: 1 });
    points.insert(Point { x: 2, y: 2 });

    assert_eq!(
        points,
        points_from_path(
            origin,
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

    assert_eq!(4, distance_between(&origin, &Point { x: 2, y: 2 }));

    let mut first_points = points_from_path(origin, paths[0].to_owned());

    println!("number of points from first path: {}", first_points.len());

    let second_points = points_from_path(origin, paths[1].to_owned());

    println!("number of points from second path: {}", second_points.len());

    let closest = closest_intersection(origin, first_points.clone(), second_points.clone());
    let distance = distance_between(&origin, &closest);

    println!("Distance: {}", distance);

    first_points.remove(&origin);
    println!(
        "{:?}",
        first_points
            .intersection(&second_points)
            .map(|x| distance_between(&origin, x))
            .collect::<Vec<i32>>()
    );

    //2611 is too high
    distance
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
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
    distance: i32,
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

fn distance_between(first: &Point, second: &Point) -> i32 {
    (first.x - second.x).abs() + (first.y - second.y).abs()
}

fn points_from_path(start: Point, path: Vec<Vector>) -> HashSet<Point> {
    let mut points = HashSet::new();

    points.insert(start.to_owned());

    let mut current_x = start.x;
    let mut current_y = start.y;

    for vector in path {
        match vector.direction {
            Direction::Up => {
                for i in 1..=vector.distance {
                    points.insert(Point {
                        x: current_x,
                        y: current_y + i,
                    });
                }
                current_y -= vector.distance;
            }
            Direction::Down => {
                for i in 1..=vector.distance {
                    points.insert(Point {
                        x: current_x,
                        y: current_y - i,
                    });
                }
                current_y -= vector.distance;
            }
            Direction::Right => {
                for i in 1..=vector.distance {
                    points.insert(Point {
                        x: current_x + i,
                        y: current_y,
                    });
                }
                current_x += vector.distance;
            }
            Direction::Left => {
                for i in 1..=vector.distance {
                    points.insert(Point {
                        x: current_x - i,
                        y: current_y,
                    });
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

    let distance = chars.collect::<String>().parse::<i32>().unwrap();

    Vector {
        direction: direction,
        distance: distance,
    }
}

fn load_paths(filepath: &str) -> Vec<Vec<Vector>> {
    super::util::lines_from_path(filepath)
        .filter_map(|x| match x {
            Ok(line) => Some(line.split(",").map(|x| parse_path(x)).collect()),
            _ => None,
        })
        .collect()
}
