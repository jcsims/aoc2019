mod day1;
mod day2;
mod util;

use std::env;
use std::time::Instant;

// Doesn't work yet....
// macro_rules! run_one {
//     ( $( $x:item )+ ) => {
//         let now = Instant::now();
//         $(
//             println!("{:?}", $x());
//         )
//         println!("Elapsed time: {:?}", Instant::elapsed(&now));
//     };
// }

fn main() {
    match env::args().skip(1).next() {
        None => run_all(),
        Some(exercise) => match exercise.as_ref() {
            "d1p1" => run_one(day1::part1),
            "d1p2" => run_one(day1::part2),
            "d2p1" => run_one(day2::part1),
            "d2p2" => run_one(day2::part2),
            _ => panic!("unknown exercise: {}", exercise),
        },
    }
}

fn run_all() {
    let now = Instant::now();

    assert_eq!(3442987, day1::part1());
    assert_eq!(5161601, day1::part2());
    assert_eq!(3306701, day2::part1());

    println!("Elapsed time: {:?}", Instant::elapsed(&now));
}

fn run_one(exercise: fn() -> i32) {
    let now = Instant::now();

    println!("{}", exercise());

    println!("Elapsed time: {:?}", Instant::elapsed(&now));
}
