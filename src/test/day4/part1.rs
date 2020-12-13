use crate::day4::{part1::parse_test, parse_final_p1, part1::exo};

#[test]
fn result_with_test_input() {
    let result: u64 = 2;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u64 = 192;
    let ret = exo(parse_final_p1());
    assert_eq!(result, ret);
}