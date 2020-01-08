use crate::util;
use log::trace;

pub fn part1() -> i64 {
    let input = util::file_as_string("data/d8.txt");

    let layers = build_layers(&input, 25, 6);

    let target_layer = layers
        .iter()
        .min_by(|x, y| {
            x.iter()
                .filter(|x| **x == Pixel::Black)
                .count()
                .cmp(&y.iter().filter(|x| **x == Pixel::Black).count())
        })
        .unwrap();

    let layer_ones = &target_layer.iter().filter(|x| **x == Pixel::White).count();
    let layer_twos = &target_layer
        .iter()
        .filter(|x| **x == Pixel::Transparent)
        .count();

    (layer_ones * layer_twos) as i64
}

pub fn part2() -> i64 {
    let input = util::file_as_string("data/d8.txt");

    let width = 25;
    let height = 6;

    let layers = build_layers(&input, width, height);

    let mut image = Vec::with_capacity(width * height);

    for i in 0..(width * height) {
        let pixel = match layers
            .iter()
            .map(|x| x[i])
            .try_fold(Pixel::Transparent, compare_layers)
        {
            // This is gross
            Ok(x) => x,
            Err(x) => x,
        };
        image.push(pixel);
    }

    let mut image_string = String::new();

    for (i, pixel) in image.iter().enumerate() {
        if i % width == 0 {
            image_string.push('\n');
        }
        match pixel {
            Pixel::Black => image_string.push(' '),
            Pixel::White => image_string.push('1'),
            Pixel::Transparent => image_string.push(' '),
        }
    }

    assert_eq!(
        "
1      11 1111  11  1  1 
1       1 1    1  1 1  1 
1       1 111  1    1111 
1       1 1    1    1  1 
1    1  1 1    1  1 1  1 
1111  11  1111  11  1  1 ",
        image_string
    );

    42
}

// When folding over layer values, which is retained?
fn compare_layers(x: Pixel, y: Pixel) -> Result<Pixel, Pixel> {
    match (x, y) {
        (_, Pixel::Black) => Err(Pixel::Black),
        (_, Pixel::White) => Err(Pixel::White),
        (_, Pixel::Transparent) => Ok(Pixel::Transparent),
    }
}

fn build_layers(input: &str, width: usize, height: usize) -> Vec<Vec<Pixel>> {
    let mut output = Vec::new();
    let mut iterator = input.chars().filter_map(parse_pixel).peekable();

    let layer_size = width * height;

    while iterator.peek().is_some() {
        output.push((&mut iterator).take(layer_size).collect::<Vec<Pixel>>());
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pixel {
    Transparent,
    Black,
    White,
}

fn parse_pixel(ch: char) -> Option<Pixel> {
    match ch {
        '0' => Some(Pixel::Black),
        '1' => Some(Pixel::White),
        '2' => Some(Pixel::Transparent),
        _ => None,
    }
}

#[test]
fn building_layers() {
    let _ = env_logger::builder().is_test(true).try_init();

    assert_eq!(
        vec![
            vec![
                Pixel::Black,
                Pixel::Transparent,
                Pixel::White,
                Pixel::Black,
                Pixel::Transparent,
                Pixel::Transparent
            ],
            vec![
                Pixel::White,
                Pixel::Black,
                Pixel::Black,
                Pixel::White,
                Pixel::Transparent,
                Pixel::White
            ]
        ],
        build_layers("021022100121", 3, 2)
    );
}
