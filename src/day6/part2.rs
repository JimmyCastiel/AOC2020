use crate::day6::common::VotingGroup;

pub fn exo(voting_groups: Vec<VotingGroup>) -> u16 {
    let mut res = 0;

    if !voting_groups.is_empty() {
        for vg in voting_groups {
            res += vg.nb_q_everyone_answered_yes();
        }
    }

    res
}