use crate::day9::common::{Day9Error, Serie};

pub fn exo(result: Result<Serie, Day9Error>, depth: usize) -> u64 {
    match result {
        Ok(serie) =>  {
            let mut weakness = serie.get_weakness_serie(depth);
            if weakness.len() > 0 {
                weakness.sort();
                let (min, max) = (weakness[0], weakness[weakness.len() - 1]);
                return min + max;
            }
        },
        _ => {}
    }
    0
}
