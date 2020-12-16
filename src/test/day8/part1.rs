use crate::day8::{common::parse_test, parse_final, part1::exo};

#[test]
fn result_with_test_input() {
    let result: i16 = 5;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: i16 = 1941;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}