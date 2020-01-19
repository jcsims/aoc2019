use crate::util;

pub fn part1() -> i64 {
    let input = digits(&util::file_as_string("data/d16.txt"));

    vec_to_int(
        fft(input, 100)
            .iter()
            .take(8)
            .copied()
            .collect::<Vec<i64>>(),
    )
}

pub fn part2() -> i64 {
    42
}

fn digits(input: &str) -> Vec<i64> {
    input
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| i64::from(x))
        .collect::<Vec<i64>>()
}

fn vec_to_int(mut input: Vec<i64>) -> i64 {
    let mut output = 0;
    for i in 0..input.len() as u32 {
        output += input.pop().unwrap() * 10_i64.pow(i);
    }

    output
}

// Generate the base pattern, which can just be cycled
fn pattern_iterator<'a>(
    base_pattern: &'a Vec<i64>,
    repeats: &'a usize,
) -> impl Iterator<Item = i64> + 'a {
    base_pattern
        .iter()
        .flat_map(move |x| vec![*x].repeat(*repeats))
        .cycle()
        .skip(1)
}

fn normalize(num: i64) -> i64 {
    (num % 10).abs()
}

fn ith_output(pattern: &Vec<i64>, input: &Vec<i64>, i: &usize) -> i64 {
    let mut pattern_iter = pattern_iterator(pattern, i);

    normalize(input.iter().map(|x| x * pattern_iter.next().unwrap()).sum())
}

// - Output length is the same as input length
// - each output element is the sum of:
//   - each input element multiplied by the corresponding number in a
//     sequence from the patter_iterator, with `repeats` equal to the
//     count of the element being generated (starting at 1)
fn fft_step(input: &Vec<i64>) -> Vec<i64> {
    let pattern = vec![0, 1, 0, -1];

    (1..input.len() + 1)
        .map(|x| ith_output(&pattern, &input, &x))
        .collect()
}

fn fft(input: Vec<i64>, count: i64) -> Vec<i64> {
    let mut current = input.clone();

    for _ in 0..count {
        current = fft_step(&current);
    }

    current
}

#[test]
fn fft_test() {
    // single step
    assert_eq!(vec![4, 8, 2, 2, 6, 1, 5, 8], fft_step(&digits("12345678")));

    assert_eq!(
        vec![2, 4, 1, 7, 6, 1, 7, 6],
        fft(digits("80871224585914546619083218645595"), 100)
            .iter()
            .take(8)
            .copied()
            .collect::<Vec<i64>>()
    );
}

#[test]
fn normalize_test() {
    assert_eq!(5, normalize(-45));

    assert_eq!(4, normalize(4));

    assert_eq!(0, normalize(0));
}

#[test]
fn pattern_test() {
    assert_eq!(
        vec![2, 3, 4, 1],
        pattern_iterator(&vec![1, 2, 3, 4], &1)
            .take(4)
            .collect::<Vec<i64>>()
    );

    assert_eq!(
        vec![1, 2, 2, 3, 3, 4],
        pattern_iterator(&vec![1, 2, 3, 4], &2)
            .take(6)
            .collect::<Vec<i64>>()
    );
}

#[test]
fn vec_to_int_test() {
    assert_eq!(123, vec_to_int(vec![1, 2, 3]));
}
