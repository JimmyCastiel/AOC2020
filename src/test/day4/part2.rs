use crate::day4::{part2::parse_test, parse_final_p2, part2::exo};

#[test]
fn result_with_test_input() {
    let result: u64 = 4;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u64 = 101;
    let ret = exo(parse_final_p2());
    assert_eq!(result, ret);
}