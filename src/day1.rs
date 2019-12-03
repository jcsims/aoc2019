pub fn part1() -> i32 {
    assert_eq!(2, mass_to_fuel(12));
    assert_eq!(2, mass_to_fuel(14));
    assert_eq!(654, mass_to_fuel(1969));
    assert_eq!(33583, mass_to_fuel(100756));

    super::util::lines_from_path("data/d1.txt")
        .map(|x| match x {
            Ok(line) => mass_to_fuel(line.parse::<i32>().unwrap()),
            _ => 0,
        })
        .sum::<i32>()
}

pub fn part2() -> i32 {
    assert_eq!(2, mass_to_fuel_inclusive(12));
    assert_eq!(966, mass_to_fuel_inclusive(1969));
    assert_eq!(50346, mass_to_fuel_inclusive(100756));

    super::util::lines_from_path("data/d1.txt")
        .map(|x| match x {
            Ok(line) => mass_to_fuel_inclusive(line.parse::<i32>().unwrap()),
            _ => 0,
        })
        .sum::<i32>()
}

pub fn mass_to_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

pub fn mass_to_fuel_inclusive(mass: i32) -> i32 {
    let mut fuel = mass_to_fuel(mass);

    let mut added_fuel = mass_to_fuel(fuel);

    while added_fuel > 0 {
        fuel += added_fuel;
        added_fuel = mass_to_fuel(added_fuel);
    }

    fuel
}
