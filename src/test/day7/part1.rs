use crate::day7::{common::parse_test, parse_final, part1::exo};

#[test]
fn result_with_test_input() {
    let result: u32 = 4;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u32 = 126;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}