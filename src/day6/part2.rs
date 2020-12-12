use crate::day6::common::{parse_test, parse_final};

pub fn exo() -> u16 {
    let mut res = 0;
    let voting_groups = parse_final();
    if !voting_groups.is_empty() {
        for vg in voting_groups {
            res += vg.nb_q_everyone_answered_yes();
        }
    }
    res
}