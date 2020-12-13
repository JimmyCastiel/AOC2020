use crate::day5::{common::parse_test, parse_final, part2::exo};

#[test]
fn result_with_test_input() {
    let result: u32 = 120;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u32 = 519;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}