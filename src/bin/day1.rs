use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const FINAL: &str = "./inputs/exo1_final_input.txt";
const TEST: &str = "./inputs/exo1_test_input.txt";

fn parse_test() -> Vec<u128> {
    parse_file(TEST)
}

fn parse_final() -> Vec<u128> {
    parse_file(FINAL)
}

fn parse_file(path: &str) -> Vec<u128> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();

    let mut values = Vec::<u128>::new();
    for line in file {
        let value: u128 = line.unwrap_or("".to_string()).parse::<u128>().unwrap_or(u128::MAX);
        if value != u128::MAX {
            values.push(value);
        }
    }
    values.sort();

    values
}

fn part1() {
    let values = parse_final();
    let mut vals = values.clone();

    loop {
        if vals.is_empty() {
            break;
        }
        let start = vals.pop().unwrap_or(u128::MAX);
        if start == u128::MAX {
            break;
        }
        match check2Sum(start, vals.clone(), 2020) {
            Some(res) => {
                println!("Result is : {}", res);
                return;
            },
            _ => {}
        }
    }
}

fn check2Sum(start: u128, values: Vec<u128>, sum: u128) -> Option<u128>{
    for val in values.clone() {
        if start + val == sum {
            return Some(start * val);
        }
    }
    return None;
}

fn part2() {
    let values = parse_final();

    let mut vals = values.clone();
    loop {
        if vals.is_empty() {
            break;
        }
        let start = vals.pop().unwrap_or(u128::MAX);
        if start == u128::MAX {
            break;
        }
        for val in vals.clone() {
            match check3Sum(start, val, vals.clone(), 2020) {
                Some(res) => {
                    println!("Result is : {}", res);
                    return;
                },
                _ => {}
            }
        }
    }
}

fn check3Sum(start: u128, mark: u128, values: Vec<u128>, sum: u128) -> Option<u128>{
    for val in values.clone() {
        if start + mark + val == sum {
            return Some(start * mark * val);
        }
    }
    return None;
}

fn main () {
    println!("Part 1:");
    part1();
    println!("Part 2:");
    part2();
}