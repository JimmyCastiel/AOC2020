use crate::day5::common::{parse_test, parse_final};

pub fn exo() -> u32 {
    let mut boardingPasses = parse_final();
    if !boardingPasses.is_empty() {
        boardingPasses.sort();
        let mut current = boardingPasses.first().unwrap().get_id() - 1;
        for boardingpass in boardingPasses {
            let id = boardingpass.get_id();
            if id == current + 1 {
                current = id;
            }
        }
        current + 1
    } else {
        0
    }
}