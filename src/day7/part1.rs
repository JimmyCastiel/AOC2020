use crate::day7::common::{Colors, Day7Error};
use std::convert::TryInto;

pub fn exo(colors: Result<Colors, Day7Error>) -> u32 {
    let mut num: u32 = 0;
    match colors {
        Ok(colors) => {
            num = colors.how_many_bags_can_contain("shiny gold".to_string().try_into().unwrap());
        },
        Err(_) => {}
    }
    num
}
