use crate::day2::common::Matcher;

pub fn exo(matchers: Vec<Matcher>) -> u16 {
    let mut matches: u16 = 0;

    for m in matchers {
        if m.is_match_part1() {
            matches +=1;
        }
    }

    matches
}