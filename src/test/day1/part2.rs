use crate::day1::{common::parse_test, parse_final, part2::exo};

#[test]
fn result_with_test_input() {
    let result: u64 = 241861950;
    let ret = exo(parse_test());
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u64 = 199132160;
    let ret = exo(parse_final());
    assert_eq!(result, ret);
}