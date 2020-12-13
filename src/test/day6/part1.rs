use crate::day6::{common::parse_test, parse_final, part1::exo};

#[test]
fn result_with_test_input() {
    let result: u16 = 11;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u16 = 6382;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}