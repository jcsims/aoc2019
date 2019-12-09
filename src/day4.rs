pub fn part1() -> i32 {
    let lower = 124075;
    let upper = 580769;

    let mut passwords = 0;

    for i in lower..=upper {
        let input = digits(i);
        match doubled_digits(&input) {
            false => continue,
            true => match digits_in_order(&input) {
                false => continue,
                true => passwords += 1,
            },
        }
    }

    passwords
}

pub fn part2() -> i32 {
    let lower = 124075;
    let upper = 580769;

    let mut passwords = 0;

    for i in lower..=upper {
        let input = digits(i);
        match strictly_doubled_digits(&input) {
            false => continue,
            true => match digits_in_order(&input) {
                false => continue,
                true => passwords += 1,
            },
        }
    }

    passwords
}

fn doubled_digits(input: &Vec<i32>) -> bool {
    let length = input.len();

    let mut deduped = input.clone();

    deduped.dedup();

    length != deduped.len()
}

fn strictly_doubled_digits(input: &Vec<i32>) -> bool {
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

fn digits_in_order(input: &Vec<i32>) -> bool {
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

fn digits(input: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();

    let mut temp = input;

    while temp > 0 {
        digits.insert(0, temp % 10);
        temp /= 10;
    }

    digits
}

#[test]
fn part1_test() {
    assert_eq!(vec!(1, 2, 3, 4), digits(1234));
}
