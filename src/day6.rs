use crate::util;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn part1() -> i32 {
    let mut orbit_relationships = util::lines_from_path("data/d6.txt")
        .map(|x| match x {
            Ok(orbit_string) => parse_orbit(&orbit_string),
            Err(e) => panic!("Funky line: {:?}", e),
        })
        .collect::<HashSet<OrbitRelationship>>();

    let orbits = build_orbits(&mut orbit_relationships);

    orbits.iter().fold(0, |acc, x| acc + x.depth)
}

pub fn part2() -> i32 {
    42
}

const COM: &str = "COM";

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct OrbitRelationship {
    orbited: String,
    orbitee: String,
}

#[derive(Debug)]
struct Orbit {
    depth: i32,
    object: String,
}

impl Orbit {
    fn new(object: String) -> Orbit {
        Orbit {
            object: object,
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
    };
    orbits.insert(com);

    let mut to_remove: HashSet<OrbitRelationship> = HashSet::new();

    while !orbit_relationships.is_empty() {
        for o in orbit_relationships.iter() {
            let orbited = Orbit::new(o.orbited.clone());
            if orbits.contains(&orbited) {
                let orbited = orbits.get(&orbited).unwrap();
                let orbitee = Orbit {
                    depth: orbited.depth + 1,
                    object: o.orbitee.clone(),
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

    assert_eq!(
        42,
        build_orbits(&mut orbit_relationships)
            .iter()
            .fold(0, |acc, x| acc + x.depth)
    );
}

#[test]
fn hash_empty() {
    let mut hash = HashSet::new();

    assert!(hash.insert(1));

    assert!(hash.remove(&1));

    assert!(hash.is_empty());
}
