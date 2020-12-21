use crate::day9::{common::parse_test, parse_final, part2::exo};

#[test]
fn result_with_test_input() {
    let result: u64 = 62;
    let ret = exo(parse_test(), 5);
    assert_eq!(result, ret);
}

#[test]
fn result_with_final_input() {
    let result: u64 = 438559930;
    let ret = exo(parse_final(), 25);
    assert_eq!(result, ret);
}
