use crate::util;

pub fn part1() -> i64 {
    let lower = 124_075;
    let upper = 580_769;

    let mut passwords = 0;

    for i in lower..=upper {
        let input = util::digits(i);
        if doubled_digits(&input) {
            if digits_in_order(&input) {
                passwords += 1;
            } else {
                continue;
            }
        } else {
            continue;
        }
    }

    passwords
}

pub fn part2() -> i64 {
    let lower = 124_075;
    let upper = 580_769;

    let mut passwords = 0;

    for i in lower..=upper {
        let input = util::digits(i);
        if strictly_doubled_digits(&input) {
            if digits_in_order(&input) {
                passwords += 1;
            } else {
                continue;
            }
        } else {
            continue;
        }
    }

    passwords
}

fn doubled_digits(input: &[i64]) -> bool {
    let length = input.len();

    let mut deduped = input.to_owned();

    deduped.dedup();

    length != deduped.len()
}

fn strictly_doubled_digits(input: &[i64]) -> bool {
    let mut input_iter = input.iter();

    let mut first = input_iter.next().unwrap();
    let mut second = input_iter.next().unwrap();
    let mut third = input_iter.next().unwrap();

    // special case
    if (first == second) && (second != third) {
        return true;
    }

    for i in input_iter {
        match (first, second, third, i) {
            (w, x, y, z) if x == y => {
                if (w != x) && (y != z) {
                    return true;
                } else {
                    first = x;
                    second = y;
                    third = z;
                }
            }
            (_, x, y, z) => {
                first = x;
                second = y;
                third = z;
            }
        }
    }

    if (first != second) && (second == third) {
        return true;
    }

    false
}

fn digits_in_order(input: &[i64]) -> bool {
    input
        .iter()
        .fold(Some(&-1), |acc, x| match acc {
            None => None,
            Some(y) => {
                if y <= x {
                    Some(x)
                } else {
                    None
                }
            }
        })
        .is_some()
}

#[test]
fn part1_test() {
    assert_eq!(vec!(1, 2, 3, 4), util::digits(1234));
}
