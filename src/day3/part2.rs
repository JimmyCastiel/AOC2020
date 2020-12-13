use crate::day3::Map;

pub fn exo(map: Map) -> u32 {
    let growths = map.grow_part2();

    let mut res: u32 = 1;
    for (t, _) in growths {
        res *= t as u32;
    }

    res
}