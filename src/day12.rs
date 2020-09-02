use crate::util;
//use log::trace;
use regex::Regex;
use std::cmp::{self, Ordering};
use std::collections::HashMap;

pub fn part1() -> i64 {
    let mut moons = parse("data/d12.txt");

    for _ in 0..1000 {
        update_velocities(&mut moons);
        update_positions(&mut moons);
    }

    let energy: i32 = moons.iter().map(|x| Moon::energy(x)).sum();

    energy as i64
}

pub fn part2() -> i64 {
    let moons = parse("data/d12.txt");

    let initial_x_state: Vec<(i32, i32)> = moons.iter().map(|m| (m.x, m.x_vel)).collect();
    let initial_y_state: Vec<(i32, i32)> = moons.iter().map(|m| (m.y, m.y_vel)).collect();
    let initial_z_state: Vec<(i32, i32)> = moons.iter().map(|m| (m.z, m.z_vel)).collect();

    // get the x period
    let mut x_period = 0;
    let mut mut_moons = moons.clone();

    loop {
        update_velocities(&mut mut_moons);
        update_positions(&mut mut_moons);
        x_period += 1;

        if mut_moons
            .iter()
            .map(|m| (m.x, m.x_vel))
            .collect::<Vec<(i32, i32)>>()
            == initial_x_state
        {
            break;
        }
    }

    // get the y period
    let mut y_period = 0;
    mut_moons = moons.clone();

    loop {
        update_velocities(&mut mut_moons);
        update_positions(&mut mut_moons);
        y_period += 1;

        if mut_moons
            .iter()
            .map(|m| (m.y, m.y_vel))
            .collect::<Vec<(i32, i32)>>()
            == initial_y_state
        {
            break;
        }
    }

    // get the z period
    let mut z_period = 0;
    mut_moons = moons;

    loop {
        update_velocities(&mut mut_moons);
        update_positions(&mut mut_moons);
        z_period += 1;

        if mut_moons
            .iter()
            .map(|m| (m.z, m.z_vel))
            .collect::<Vec<(i32, i32)>>()
            == initial_z_state
        {
            break;
        }
    }

    let x_factors = primes::factors(x_period);
    let y_factors = primes::factors(y_period);
    let z_factors = primes::factors(z_period);

    let factors = consolidate_prime_factors(vec![x_factors, y_factors, z_factors]);

    factors.iter().copied().product::<u64>() as i64
}

//Given a vector of vectors, each containing a list of prime factors,
// consolidate them by removing duplicates across the vectors, but
// retaining the largest number of duplicates for each factor. E.g.:
// [[2, 2, 3, 7], [2, 11]] -> [2, 2, 3, 7, 11]
fn consolidate_prime_factors(factors: Vec<Vec<u64>>) -> Vec<u64> {
    let mut all = HashMap::new();

    for factorv in factors {
        let mut seen = HashMap::new();
        for factor in factorv {
            let entry = seen.entry(factor).or_insert(0);
            *entry += 1;
        }

        for (factor, count) in seen {
            let entry = all.entry(factor).or_insert(0);
            *entry = cmp::max(count, *entry);
        }
    }

    let mut factors = Vec::new();

    for (k, v) in all {
        for _ in 0..v {
            factors.push(k)
        }
    }
    factors
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    x_vel: i32,
    y_vel: i32,
    z_vel: i32,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            x,
            y,
            z,
            x_vel: 0,
            y_vel: 0,
            z_vel: 0,
        }
    }

    fn energy(&self) -> i32 {
        let potential = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic = self.x_vel.abs() + self.y_vel.abs() + self.z_vel.abs();

        potential * kinetic
    }
}

fn update_velocities(moons: &mut Vec<Moon>) {
    let da_moons = moons.clone();

    let mut delta_vx = 0;
    let mut delta_vy = 0;
    let mut delta_vz = 0;

    for moon in moons {
        for other_moon in &da_moons {
            match other_moon.x.cmp(&moon.x) {
                Ordering::Equal => (),
                Ordering::Less => delta_vx -= 1,
                Ordering::Greater => delta_vx += 1,
            }

            match other_moon.y.cmp(&moon.y) {
                Ordering::Equal => (),
                Ordering::Less => delta_vy -= 1,
                Ordering::Greater => delta_vy += 1,
            }

            match other_moon.z.cmp(&moon.z) {
                Ordering::Equal => (),
                Ordering::Less => delta_vz -= 1,
                Ordering::Greater => delta_vz += 1,
            }

            moon.x_vel += delta_vx;
            moon.y_vel += delta_vy;
            moon.z_vel += delta_vz;
            delta_vx = 0;
            delta_vy = 0;
            delta_vz = 0;
        }
    }
}

fn update_positions(moons: &mut Vec<Moon>) {
    for moon in moons {
        moon.x += moon.x_vel;
        moon.y += moon.y_vel;
        moon.z += moon.z_vel;
    }
}

fn parse(filepath: &str) -> Vec<Moon> {
    util::lines_from_path(filepath)
        .filter_map(|x| match x {
            Ok(x) => Some(parse_line(&x)),
            _ => None,
        })
        .collect::<Vec<Moon>>()
}

fn parse_line(line: &str) -> Moon {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let captures = re.captures(line).unwrap();

    let x = captures[1].parse::<i32>().unwrap();
    let y = captures[2].parse::<i32>().unwrap();
    let z = captures[3].parse::<i32>().unwrap();

    Moon::new(x, y, z)
}

#[test]
fn parsing_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    assert_eq!(Moon::new(1, 2, 3), parse_line("<x=1, y=2, z=3>"));

    assert_eq!(
        vec![
            Moon::new(14, 2, 8),
            Moon::new(7, 4, 10),
            Moon::new(1, 17, 16),
            Moon::new(-4, -1, 1)
        ],
        parse("data/d12.txt")
    );
}

#[test]
fn update_velocities_test() {
    let mut moons = vec![Moon::new(1, -2, 3), Moon::new(4, 0, 4), Moon::new(2, -5, 0)];

    update_velocities(&mut moons);

    assert_eq!(
        moons,
        vec![
            Moon {
                x: 1,
                y: -2,
                z: 3,
                x_vel: 2,
                y_vel: 0,
                z_vel: 0
            },
            Moon {
                x: 4,
                y: 0,
                z: 4,
                x_vel: -2,
                y_vel: -2,
                z_vel: -2
            },
            Moon {
                x: 2,
                y: -5,
                z: 0,
                x_vel: 0,
                y_vel: 2,
                z_vel: 2
            }
        ]
    );

    update_velocities(&mut moons);

    assert_eq!(
        moons,
        vec![
            Moon {
                x: 1,
                y: -2,
                z: 3,
                x_vel: 4,
                y_vel: 0,
                z_vel: 0
            },
            Moon {
                x: 4,
                y: 0,
                z: 4,
                x_vel: -4,
                y_vel: -4,
                z_vel: -4
            },
            Moon {
                x: 2,
                y: -5,
                z: 0,
                x_vel: 0,
                y_vel: 4,
                z_vel: 4
            }
        ]
    );
}

#[test]
fn update_position_test() {
    let mut moons = vec![Moon::new(1, -2, 3), Moon::new(4, 0, 4), Moon::new(2, -5, 0)];

    update_velocities(&mut moons);
    update_positions(&mut moons);

    assert_eq!(
        moons,
        vec![
            Moon {
                x: 3,
                y: -2,
                z: 3,
                x_vel: 2,
                y_vel: 0,
                z_vel: 0
            },
            Moon {
                x: 2,
                y: -2,
                z: 2,
                x_vel: -2,
                y_vel: -2,
                z_vel: -2
            },
            Moon {
                x: 2,
                y: -3,
                z: 2,
                x_vel: 0,
                y_vel: 2,
                z_vel: 2
            }
        ]
    );
}
