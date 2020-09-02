use crate::util;

pub fn part1() -> i64 {
    util::lines_from_path("data/d1.txt")
        .map(|x| match x {
            Ok(line) => mass_to_fuel(line.parse::<i64>().unwrap()),
            _ => 0,
        })
        .sum::<i64>()
}

pub fn part2() -> i64 {
    util::lines_from_path("data/d1.txt")
        .map(|x| match x {
            Ok(line) => mass_to_fuel_inclusive(line.parse::<i64>().unwrap()),
            _ => 0,
        })
        .sum::<i64>()
}

pub fn mass_to_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

pub fn mass_to_fuel_inclusive(mass: i64) -> i64 {
    let mut fuel = mass_to_fuel(mass);

    let mut added_fuel = mass_to_fuel(fuel);

    while added_fuel > 0 {
        fuel += added_fuel;
        added_fuel = mass_to_fuel(added_fuel);
    }

    fuel
}

#[test]
fn part1_test() {
    assert_eq!(2, mass_to_fuel(12));
    assert_eq!(2, mass_to_fuel(14));
    assert_eq!(654, mass_to_fuel(1969));
    assert_eq!(33583, mass_to_fuel(100_756));
}

#[test]
fn part2_test() {
    assert_eq!(2, mass_to_fuel_inclusive(12));
    assert_eq!(966, mass_to_fuel_inclusive(1969));
    assert_eq!(50346, mass_to_fuel_inclusive(100_756));
}
