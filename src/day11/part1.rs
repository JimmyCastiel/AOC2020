use crate::day11::common::{Day11Error, Boat};

pub fn exo(result: Result<Boat, Day11Error>) -> usize {
    match result {
        Ok(mut boat) =>  {
            return boat.go_to_stable();
        },
        _ => {}
    }
    0
}
