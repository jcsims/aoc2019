use crate::util;

pub fn part1() -> i64 {
    let input = digits(&util::file_as_string("data/d16.txt"));

    let output = fft(input, 100);

    vec_to_int(&output[0..8])
}

pub fn part2() -> i64 {
    let mut input = digits(&util::file_as_string("data/d16.txt")).repeat(10000);

    let offset = vec_to_int(&input[0..7]) as usize;

    // The offset happens to be 5,978,261 (out of 6,500,000 digits in
    // the repeated sequence). By the time we're calculating the FFT
    // all way out here (the remaining 521,739 digits), it's all 1's.
    input = input[offset..].to_vec();

    let output = simple_fft(input, 100);

    vec_to_int(&output[0..8])
}

fn digits(input: &str) -> Vec<i64> {
    input
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| i64::from(x))
        .collect::<Vec<i64>>()
}

fn vec_to_int(input: &[i64]) -> i64 {
    let mut output = 0;
    for i in 0..input.len() as u32 {
        output += input[input.len() - (i + 1) as usize] * 10_i64.pow(i);
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

// Here, we take advantage of the fact that our inputs are all well
// into the range of being multiplied by 1, so we cheat a bit...
fn simple_fft(mut input: Vec<i64>, count: i64) -> Vec<i64> {
    let mut output;

    for _ in 0..count {
        output = Vec::new();
        output.push(input.pop().unwrap());

        for i in 0..input.len() {
            output.push((output[i] + input.pop().unwrap()) % 10);
        }

        output.reverse();

        input = output;
    }

    input
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
    assert_eq!(123, vec_to_int(&[1, 2, 3]));
}

#[test]
fn test_offset_1() {
    let mut input = digits("03036732577212944063491565474664").repeat(10000);

    let offset = vec_to_int(&input[0..7]) as usize;

    input = input[offset..].to_vec();

    let output = simple_fft(input, 100);

    assert_eq!(84462026, vec_to_int(&output[0..8]));
}

#[test]
fn test_offset_2() {
    let mut input = digits("02935109699940807407585447034323").repeat(10000);

    let offset = vec_to_int(&input[0..7]) as usize;

    input = input[offset..].to_vec();

    let output = simple_fft(input, 100);

    assert_eq!(78725270, vec_to_int(&output[0..8]));
}

#[test]
fn test_offset_3() {
    let mut input = digits("03081770884921959731165446850517").repeat(10000);

    let offset = vec_to_int(&input[0..7]) as usize;

    input = input[offset..].to_vec();

    let output = simple_fft(input, 100);

    assert_eq!(53553731, vec_to_int(&output[0..8]));
}
