use crate::day10::common::{Day10Error, Serie};

pub fn exo(result: Result<Serie, Day10Error>) -> u64 {
    match result {
        Ok(mut serie) =>  {
            let diffs = serie.get_differences();
            return diffs.get(&1).unwrap_or(&0) * diffs.get(&3).unwrap_or(&0);
        },
        _ => {}
    }
    0
}
