use crate::day3::{common::parse_test, common::parse_final, part1::exo};

#[test]
fn result_with_test_input() {
    let result: u16 = 7;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u16 = 225;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}