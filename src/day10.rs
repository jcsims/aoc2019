use crate::util;
use log::trace;
use std::f32::{self, consts};

pub fn part1() -> i64 {
    let input = parse(&util::file_as_string("data/d10.txt"));

    // Iterate over each point, and see how many other points are
    // visible to it. Don't count the node itself.

    let target = input
        .iter()
        .map(|x| (num_points_visible_to_asteroid(*x, &input), x))
        .max_by(|x, y| x.0.cmp(&y.0))
        .unwrap();

    //println!("station location: {:?}", target);

    target.0 as i64
}

pub fn part2() -> i64 {
    let input = parse(&util::file_as_string("data/d10.txt"));

    // Get this from part 1
    let station = Point { x: 14, y: 17 };

    let mut others = points_visible_to_asteroid(station, &input);

    // Will find it on the first pass!
    assert!(others.len() > 200);

    others.sort_by(|a, b| {
        circular_distance_from_y_axis(station, *a)
            .partial_cmp(&circular_distance_from_y_axis(station, *b))
            .unwrap()
    });

    let target = others[199];

    (target.x * 100 + target.y) as i64
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

fn circular_distance_from_y_axis(origin: Point, dest: Point) -> f32 {
    let delta_x = (dest.x - origin.x) as f32;
    let delta_y = -1f32 * ((dest.y - origin.y) as f32);

    // atan2 is a fun function:
    //    pi/2 - pi | 0 - pi/2
    //              |
    // pi ---> ----------- <--- 0
    // [-pi/2 - pi) | 0 - -pi/2
    //              |

    match delta_y.atan2(delta_x) {
        x if x < 0f32 => (x - consts::FRAC_PI_2).abs(),
        x if x >= 0f32 && x <= consts::FRAC_PI_2 => consts::FRAC_PI_2 - x,
        x if x > consts::FRAC_PI_2 => (2f32 * consts::PI) - (x - consts::FRAC_PI_2),
        _ => panic!("atan2 returned a value I wasn't expecting...."),
    }
}

fn distance(a: Point, b: Point) -> f32 {
    let delta_x = a.x - b.x;
    let delta_y = a.y - b.y;

    ((delta_x * delta_x + delta_y * delta_y) as f32).sqrt()
}

/// From `origin`, is `p` occluding `destination`?
fn occluding(origin: Point, p: Point, destination: Point) -> bool {
    let point_dist = distance(origin, p) + distance(p, destination);
    let dest_dist = distance(origin, destination);

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

fn num_points_visible_to_asteroid(asteroid: Point, others: &[Point]) -> i32 {
    let visible = points_visible_to_asteroid(asteroid, others);

    visible.len() as i32
}

fn points_visible_to_asteroid(asteroid: Point, others: &[Point]) -> Vec<Point> {
    let mut seen = Vec::new();
    for destination in others.iter() {
        if &asteroid == destination {
            continue;
        } else {
            let mut can_see = true;
            for other_point in others.iter() {
                if destination == other_point || other_point == &asteroid {
                    continue;
                } else if occluding(asteroid, *other_point, *destination) {
                    trace!(
                        "origin: {:?}. {:?} is occluding {:?}",
                        asteroid,
                        other_point,
                        destination
                    );
                    can_see = false;
                }
            }
            if can_see {
                seen.push(destination.clone());
            }
        }
    }

    seen
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
        .map(|x| (num_points_visible_to_asteroid(*x, &input), x))
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

#[test]
fn test_circular_distance() {
    let origin = Point { x: 0, y: 0 };

    assert_eq!(0f32, 0f32.atan2(1f32));

    assert_eq!(consts::FRAC_PI_2, 1f32.atan2(0f32));

    assert_eq!(-consts::FRAC_PI_2, -(1f32.atan2(0f32)));

    assert_eq!(-consts::PI + consts::FRAC_PI_4, -(1f32.atan2(-1f32)));

    assert!((consts::PI - 0f32.atan2(-1f32)).abs() < 0.000_001);

    // Note that in our system,y increases as you go "down" in a grid.
    assert_eq!(
        0f32,
        circular_distance_from_y_axis(origin, Point { x: 0, y: -1 })
    );

    assert_eq!(
        consts::FRAC_PI_2,
        circular_distance_from_y_axis(origin, Point { x: 1, y: 0 })
    );

    assert_eq!(
        consts::PI,
        circular_distance_from_y_axis(origin, Point { x: 0, y: 1 })
    );

    assert_eq!(
        consts::PI + consts::FRAC_PI_2,
        circular_distance_from_y_axis(origin, Point { x: -1, y: 0 })
    );
}
