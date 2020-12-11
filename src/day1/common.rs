use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const FINAL: &str = "./inputs/exo1_final_input.txt";
const TEST: &str = "./inputs/exo1_test_input.txt";

pub(crate) fn parse_test() -> Vec<u64> {
    parse_file(TEST)
}

pub(crate) fn parse_final() -> Vec<u64> {
    parse_file(FINAL)
}

fn parse_file(path: &str) -> Vec<u64> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();

    let mut values = Vec::<u64>::new();
    for line in file {
        let value: u64 = line.unwrap_or("".to_string()).parse::<u64>().unwrap_or(u64::MAX);
        if value != u64::MAX {
            values.push(value);
        }
    }
    values.sort();

    values
}