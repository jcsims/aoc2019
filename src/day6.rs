use crate::util;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn part1() -> i64 {
    let mut orbit_relationships = util::lines_from_path("data/d6.txt")
        .map(|x| match x {
            Ok(orbit_string) => parse_orbit(&orbit_string),
            Err(e) => panic!("Funky line: {:?}", e),
        })
        .collect::<HashSet<OrbitRelationship>>();

    let orbits = build_orbits(&mut orbit_relationships);

    orbit_checksum(orbits)
}

pub fn part2() -> i64 {
    let mut orbit_relationships = util::lines_from_path("data/d6.txt")
        .map(|x| match x {
            Ok(orbit_string) => parse_orbit(&orbit_string),
            Err(e) => panic!("Funky line: {:?}", e),
        })
        .collect::<HashSet<OrbitRelationship>>();

    let orbits = build_orbits(&mut orbit_relationships);

    orbital_transfers(orbits)
}

const COM: &str = "COM";

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct OrbitRelationship {
    orbited: String,
    orbitee: String,
}

#[derive(Debug, Clone)]
struct Orbit {
    depth: i64,
    object: String,
    orbits: String,
}

impl Orbit {
    fn new(object: &str) -> Orbit {
        Orbit {
            object: object.to_owned(),
            orbits: "foo".to_owned(),
            depth: 0,
        }
    }
}

impl PartialEq for Orbit {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object
    }
}

impl Eq for Orbit {}

impl Hash for Orbit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.object.hash(state);
    }
}

fn build_orbits(orbit_relationships: &mut HashSet<OrbitRelationship>) -> HashSet<Orbit> {
    let mut orbits = HashSet::new();
    let com = Orbit {
        depth: 0,
        object: COM.to_owned(),
        orbits: "nothing".to_owned(),
    };
    orbits.insert(com);

    let mut to_remove: HashSet<OrbitRelationship> = HashSet::new();

    while !orbit_relationships.is_empty() {
        for o in orbit_relationships.iter() {
            if let Some(orbited) = orbits.get(&Orbit::new(&o.orbited)) {
                let orbitee = Orbit {
                    depth: orbited.depth + 1,
                    object: o.orbitee.clone(),
                    orbits: orbited.object.clone(),
                };
                orbits.insert(orbitee);
                to_remove.insert(o.clone());
            }
        }

        for o in &to_remove {
            orbit_relationships.remove(&o);
        }
        to_remove.clear();
    }

    orbits
}

fn orbit_checksum(orbits: HashSet<Orbit>) -> i64 {
    orbits.iter().fold(0, |acc, x| acc + x.depth)
}

fn ancestors_for_orbit(orbits: &HashSet<Orbit>, orbit: &Orbit) -> Vec<Orbit> {
    let mut ancestors: Vec<Orbit> = Vec::new();

    let mut current = orbit;

    while let Some(orbited) = orbits.get(&Orbit::new(&current.orbits)) {
        ancestors.push(orbited.to_owned());
        current = orbited;
    }

    ancestors.reverse();

    ancestors
}

fn orbital_transfers(orbits: HashSet<Orbit>) -> i64 {
    let you = orbits.get(&Orbit::new("YOU")).unwrap();
    let santa = orbits.get(&Orbit::new("SAN")).unwrap();

    let your_ancestors = ancestors_for_orbit(&orbits, &you);
    let santa_ancestors = ancestors_for_orbit(&orbits, &santa);

    assert!(
        your_ancestors[0] == santa_ancestors[0],
        "You and Santa don't share a common root!"
    );

    let mut i = 1;

    loop {
        match your_ancestors.get(i) {
            Some(yours) => {
                match santa_ancestors.get(i) {
                    // Still have common ancestors. Move down the tree
                    Some(santas) if yours == santas => {
                        i += 1;
                        continue;
                    }
                    // Need to back up one, that's our common ancestor
                    _ => {
                        let depth = your_ancestors[i - 1].depth;
                        return (santa.depth - depth) + (you.depth - depth) - 2;
                    }
                }
            }
            None => panic!("No common ancestors!"),
        }
    }
}

fn parse_orbit(input: &str) -> OrbitRelationship {
    let mut splits = input.split(")");

    OrbitRelationship {
        orbited: splits.next().unwrap().to_owned(),
        orbitee: splits.next().unwrap().to_owned(),
    }
}

#[test]
fn test_parsing() {
    assert_eq!(
        OrbitRelationship {
            orbited: "foo".to_owned(),
            orbitee: "bar".to_owned()
        },
        parse_orbit("foo)bar")
    );
}

#[test]
fn orbit_count() {
    let mut orbit_relationships = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ]
    .iter()
    .map(|x| parse_orbit(&x))
    .collect();

    assert_eq!(42, orbit_checksum(build_orbits(&mut orbit_relationships)));
}

#[test]
fn hash_empty() {
    let mut hash = HashSet::new();

    assert!(hash.insert(1));

    assert!(hash.remove(&1));

    assert!(hash.is_empty());
}

#[test]
fn ancestors_test() {
    let orbits = build_orbits(
        &mut [
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]
        .iter()
        .map(|x| parse_orbit(&x))
        .collect(),
    );

    assert!(ancestors_for_orbit(&orbits, &Orbit::new("COM")).is_empty());
    let d = Orbit {
        depth: 3,
        object: "D".to_owned(),
        orbits: "C".to_owned(),
    };
    assert_eq!(
        vec![Orbit::new("COM"), Orbit::new("B"), Orbit::new("C")],
        ancestors_for_orbit(&orbits, &d)
    );
}

#[test]
fn orbital_transfers_test() {
    let mut orbit_relationships = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
        "I)SAN",
    ]
    .iter()
    .map(|x| parse_orbit(&x))
    .collect();

    assert_eq!(4, orbital_transfers(build_orbits(&mut orbit_relationships)));
}
