use crate::day5::common::BoadingPass;

pub fn exo(mut boarding_passes: Vec<BoadingPass>) -> u32 {
    if !boarding_passes.is_empty() {
        boarding_passes.sort();
        let mut current = boarding_passes.first().unwrap().get_id() - 1;
        for boardingpass in boarding_passes {
            let id = boardingpass.get_id();
            if id == current + 1 {
                current = id;
            }
        }
        return current + 1;
    }

    0
}