use crate::day2::{common::parse_test, parse_final, part2::exo};

#[test]
fn result_with_test_input() {
    let result: u16 = 1;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u16 = 267;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}