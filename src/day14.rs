use crate::util;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
//use log::trace;

pub fn part1() -> i64 {
    let mut reactions = HashMap::new();

    for line in util::lines_from_path("data/d14.txt") {
        match line {
            Ok(line) => {
                let reaction = parse_reaction(&line);

                reactions.insert(reaction.output, reaction.input);
            }
            Err(err) => {
                println!("got an error parsing a line: {:?}", err);
                continue;
            }
        }
    }

    let mut working_set = VecDeque::new();
    let mut stock_on_hand = HashMap::new();
    let mut required_ore = 0;

    working_set.push_back(Ingredient::new(1, "FUEL".to_owned()));

    while let Some(mut ingredient) = working_set.pop_front() {
        if let Some(on_hand) = stock_on_hand.remove(&ingredient.name) {
            if on_hand >= ingredient.quantity {
                // covered it completely, don't need to chase down
                // the reaction
                stock_on_hand.insert(ingredient.name, on_hand - ingredient.quantity);
                continue;
            } else {
                // didn't quite cover it, need to cover the rest
                ingredient.quantity -= on_hand;
            }
        }

        match reactions.get_key_value(&ingredient) {
            None => panic!("Unable to find reaction for: {:?}", ingredient),
            Some((target, inputs)) => {
                let mut multiple = 1;
                loop {
                    if target.quantity * multiple >= ingredient.quantity {
                        break;
                    } else {
                        multiple += 1;
                    }
                }

                for input in inputs.clone().iter_mut() {
                    input.quantity *= multiple;
                    if input.name == "ORE" {
                        required_ore += input.quantity;
                    } else {
                        working_set.push_back(input.clone());
                    }
                }

                // Reaction input has been added to the queue,
                // so add any extra output to our stock on hand

                let leftover = (target.quantity * multiple) - ingredient.quantity;

                match stock_on_hand.get_mut(&target.name) {
                    Some(on_hand) => *on_hand += leftover,
                    None => {
                        stock_on_hand.insert(target.name.clone(), leftover);
                    }
                }
            }
        }
    }

    required_ore
}

pub fn part2() -> i64 {
    let available_ore: i64 = 1_000_000_000_000;

    available_ore
}

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    quantity: i64,
}

// Going to cheat here, so we can lookup by name in a hashmap. Can do
// this other ways, as well
impl Hash for Ingredient {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Ingredient {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Ingredient {}

impl Ingredient {
    fn new(quantity: i64, name: String) -> Ingredient {
        Ingredient { quantity, name }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Reaction {
    input: Vec<Ingredient>,
    output: Ingredient,
}

impl Reaction {
    fn new(input: Vec<Ingredient>, output: Ingredient) -> Reaction {
        Reaction { input, output }
    }
}

fn parse_ingredient(raw: &str) -> Ingredient {
    let mut splits = raw.split(' ');

    Ingredient {
        quantity: splits.next().unwrap().parse().unwrap(),
        name: splits.next().unwrap().to_owned(),
    }
}

fn parse_reaction(reaction: &str) -> Reaction {
    // String like: "7 A, 1 B => 1 C"
    let re = Regex::new(r"(\d+ [A-Z]+)").unwrap();

    let mut input = Vec::new();

    let captures = re.captures_iter(reaction).peekable();

    for cap in captures {
        input.push(parse_ingredient(&cap[0]));
    }

    let output = input.pop().unwrap();

    Reaction { input, output }
}

#[test]
fn parsing_test() {
    assert_eq!(
        Reaction::new(
            vec![
                Ingredient::new(7, "A".to_owned()),
                Ingredient::new(1, "B".to_owned()),
                Ingredient::new(3, "D".to_owned())
            ],
            Ingredient::new(1, "C".to_owned())
        ),
        parse_reaction("7 A, 1 B, 3 D => 1 C")
    );
}
