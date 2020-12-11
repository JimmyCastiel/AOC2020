use crate::day5::common::{parse_test, parse_final};

pub fn exo() -> u32 {
    let boardingPasses = parse_final();
    if !boardingPasses.is_empty() {
        let mut max = boardingPasses.first().unwrap().to_owned();
        for boardingpass in boardingPasses {
            if boardingpass > max {
                max = boardingpass;
            }
        }
        return max.get_id()
    }
    0
}