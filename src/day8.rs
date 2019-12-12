use crate::util;
use log::trace;

pub fn part1() -> i32 {
    let input = util::file_as_string("data/d8.txt");

    let layers = build_layers(&input, 25, 6);

    let target_layer = layers
        .iter()
        .min_by(|x, y| {
            x.iter()
                .filter(|x| **x == '0')
                .count()
                .cmp(&y.iter().filter(|x| **x == '0').count())
        })
        .unwrap();

    let layer_ones = &target_layer.iter().filter(|x| **x == '1').count();
    let layer_twos = &target_layer.iter().filter(|x| **x == '2').count();

    (layer_ones * layer_twos) as i32
}

pub fn part2() -> i32 {
    42
}

fn build_layers(input: &str, width: usize, height: usize) -> Vec<Vec<char>> {
    let mut output = Vec::new();
    let mut iterator = input.chars().filter(|x| x.is_numeric()).peekable();

    let layer_size = width * height;

    while iterator.peek().is_some() {
        output.push((&mut iterator).take(layer_size).collect::<Vec<char>>());
    }

    trace!(
        "Built layers for width: {}, height: {}, and got {} layers for a string with {} length",
        width,
        height,
        output.len(),
        input.len()
    );
    output
}

#[test]
fn building_layers() {
    let _ = env_logger::builder().is_test(true).try_init();

    assert_eq!(
        vec![
            vec!['1', '2', '3', '4', '5', '6'],
            vec!['7', '8', '9', '0', '1', '2']
        ],
        build_layers("123456789012", 3, 2)
    );
}
