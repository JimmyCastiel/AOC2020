extern crate adventOfCode2020;

use adventOfCode2020::day9::{part1::exo as part1, part2::exo as part2, parse_final};

fn main () {
    println!("Part 1: {}", part1(parse_final(), 25));
    println!("Part 2: {}", part2(parse_final(), 25));
}