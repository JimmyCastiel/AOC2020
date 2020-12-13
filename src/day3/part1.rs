use crate::day3::Map;

pub fn exo(map: Map) -> u16 {

    let (trees, _) = map.grow_part1();

    trees
}