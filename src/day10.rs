use crate::util;
use log::trace;

pub fn part1() -> i64 {
    let input = parse(&util::file_as_string("data/d10.txt"));

    // Iterate over each point, and see how many other points are
    // visible to it. Don't count the node itself.

    let foo = input
        .iter()
        .map(|x| (points_visible_to_asteroid(x, &input), x))
        .max_by(|x, y| x.0.cmp(&y.0))
        .unwrap()
        .clone();

    foo.0 as i64
}

pub fn part2() -> i64 {
    42
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn parse(space: &str) -> Vec<Point> {
    let mut parsed = Vec::new();
    for (y, line) in space.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                '#' => parsed.push(Point {
                    x: x as i32,
                    y: y as i32,
                }),
                '.' => continue,
                x => panic!("Unknown character: {}", x),
            };
        }
    }
    parsed
}

fn distance(a: &Point, b: &Point) -> f32 {
    let delta_x = a.x - b.x;
    let delta_y = a.y - b.y;

    ((delta_x * delta_x + delta_y * delta_y) as f32).sqrt()
}

/// From `origin`, is `p` occluding `destination`?
fn occluding(origin: &Point, p: &Point, destination: &Point) -> bool {
    let point_dist = distance(&origin, &p) + distance(&p, &destination);
    let dest_dist = distance(&origin, &destination);

    let diff = (point_dist - dest_dist).abs();

    trace!(
        "diff between origin: {:?}, point: {:?} and dest: {:?} is: {}",
        origin,
        p,
        destination,
        diff
    );

    diff < 0.00001
}

fn points_visible_to_asteroid(asteroid: &Point, others: &Vec<Point>) -> i32 {
    let mut num_seen = 0;
    for destination in others.iter() {
        if asteroid == destination {
            continue;
        } else {
            let mut can_see = true;
            for other_point in others.iter() {
                if destination == other_point || other_point == asteroid {
                    continue;
                } else {
                    if occluding(asteroid, other_point, &destination) {
                        trace!(
                            "origin: {:?}. {:?} is occluding {:?}",
                            asteroid,
                            other_point,
                            destination
                        );
                        can_see = false;
                    }
                }
            }
            if can_see {
                num_seen += 1;
            }
        }
    }

    num_seen
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
            Point { x: 1, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 2 },
            Point { x: 4, y: 2 },
            Point { x: 4, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 4 }
        ],
        parse(field)
    );
}

#[test]
fn can_see_test() {
    let input = parse(
        "
.#..#
.....
#####
....#
...##"
            .trim(),
    );

    let can_see = input
        .iter()
        .map(|x| (points_visible_to_asteroid(x, &input), x))
        .collect::<Vec<(i32, &Point)>>();

    assert_eq!(
        vec![
            (7, &Point { x: 1, y: 0 }),
            (7, &Point { x: 4, y: 0 }),
            (6, &Point { x: 0, y: 2 }),
            (7, &Point { x: 1, y: 2 }),
            (7, &Point { x: 2, y: 2 }),
            (7, &Point { x: 3, y: 2 }),
            (5, &Point { x: 4, y: 2 }),
            (7, &Point { x: 4, y: 3 }),
            (8, &Point { x: 3, y: 4 }),
            (7, &Point { x: 4, y: 4 })
        ],
        can_see
    );

    assert_eq!(
        (8, &Point { x: 3, y: 4 }),
        can_see.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().clone()
    );
}
