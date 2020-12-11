use crate::day3::common::parse_final;

pub fn exo() -> u16 {
    let map = parse_final();

    let (trees, _) = map.grow_part1();

    trees
}