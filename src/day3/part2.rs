use crate::day3::common::parse_final;

pub fn exo() -> u32 {
    let map = parse_final();

    let growths = map.grow_part2();

    let mut res: u32 = 1;
    for (t, _) in growths {
        res *= t as u32;
    }

    res
}