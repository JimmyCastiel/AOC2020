use crate::day5::common::BoadingPass;

pub fn exo(boarding_passes: Vec<BoadingPass>) -> u32 {
    if !boarding_passes.is_empty() {
        let mut max = boarding_passes.first().unwrap().to_owned();
        for boardingpass in boarding_passes {
            if boardingpass > max {
                max = boardingpass;
            }
        }
        return max.get_id()
    }

    0
}