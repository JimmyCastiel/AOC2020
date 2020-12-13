extern crate regex;
extern crate adventOfCode2020;

use adventOfCode2020::day4::{part1::exo as part1, part2::exo as part2, parse_final_p1, parse_final_p2};

fn main () {
    println!("Part 1: {}", part1(parse_final_p1()));
    println!("Part 2: {}", part2(parse_final_p2()));
}