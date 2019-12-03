mod day1;
mod day2;
mod util;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("day 1, part 1: {}", day1::part1());

    println!("day 1, part 2: {}", day1::part2());

    println!("day 2, part 1: {}", day2::part1());

    println!("Elapsed time: {:?}", Instant::elapsed(&now));
}
