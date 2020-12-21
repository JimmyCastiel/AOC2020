use crate::day10::common::{Day10Error, Serie};

pub fn exo(result: Result<Serie, Day10Error>) -> u64 {
    match result {
        Ok(mut serie) =>  {
            return serie.get_combinations();
        },
        _ => {}
    }
    0
}
