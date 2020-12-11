use crate::day2::common::parse_final;

pub fn exo() -> u16{
    let matchers = parse_final();

    let (mut ok, mut ko): (u16, u16) = (0, 0);
    for m in matchers {
        if m.is_match_part1() {
            ok +=1;
        } else {
            ko +=1;
        }
    }

    ok
}