use crate::day9::common::{Day9Error, Serie};

pub fn exo(result: Result<Serie, Day9Error>, depth: usize) -> u64 {
    match result {
        Ok(serie) =>  {
           return serie.get_weakness_number(depth);
        },
        _ => {}
    }
    0
}
