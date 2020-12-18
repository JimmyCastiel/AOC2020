use crate::day9::common::Day9Error;

pub fn exo(result: Result<u64, Day9Error>) -> u64 {
    match result {
        Ok(num) =>  {
           return num;
        },
        _ => {}
    }
    0
}
