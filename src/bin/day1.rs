extern crate adventOfCode2020;

use adventOfCode2020::day1::{parse_final, part1::exo as part1, part2::exo as part2};

fn main () {
    println!("Part 1: {}", part1(parse_final()));
    println!("Part 2: {}", part2(parse_final()));
}