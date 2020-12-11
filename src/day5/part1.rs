use crate::day5::common::{parse_test, parse_final};

pub fn exo() -> u32 {
    let boardingPasses = parse_final();
    let mut max = 0;
    for boardingpass in boardingPasses {
        let id = boardingpass.get_id();
        if id > max {
            max = id;
        }
    }
    max
}