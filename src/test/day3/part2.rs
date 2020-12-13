use crate::day3::{common::parse_test, common::parse_final, part2::exo};

#[test]
fn result_with_test_input() {
    let result: u32 = 336;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u32 = 1115775000;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}