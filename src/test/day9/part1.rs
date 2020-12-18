use crate::day9::{common::parse_test, parse_final, part1::exo};

#[test]
fn result_with_test_input() {
    let result: u64 = 127;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u64 = 3199139634;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}