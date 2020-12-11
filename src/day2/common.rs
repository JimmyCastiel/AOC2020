use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const FINAL: &str = "./inputs/exo2_final_input.txt";
const TEST: &str = "./inputs/exo2_test_input.txt";

#[derive(Debug)]
pub enum Error {
    Parse,
}

#[derive(Debug)]
pub(crate) struct Matcher {
    min: u8,
    max: u8,
    car: char,
    password: String
}

impl Matcher {
    pub(crate) fn is_match_part1(&self) -> bool {
        let mut count: u8 = 0;
        for car in self.password.chars() {
            if car == self.car {
                count += 1;
            }
        }
        count >= self.min && count <= self.max
    }

    pub(crate) fn is_match_part2(&self) -> bool {
        let mut chars = self.password.chars();
        let first = chars.nth((self.min - 1) as usize).unwrap_or('\0');
        let second = chars.nth(((self.max - 1) - self.min) as usize).unwrap_or('\0');

        (first == self.car) ^ (second == self.car)
    }
}

impl FromStr for Matcher {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 3 {
            eprintln!("missing parts");
            return Err(Self::Err::Parse);
        }

        let range: Vec<&str> = parts[0].split('-').collect();
        if range.len() != 2 {
            eprintln!("malformed range");
            return Err(Self::Err::Parse);
        }
        let (min, max) = (range[0].parse::<u8>().unwrap_or(u8::MAX),
                          range[1].parse::<u8>().unwrap_or(u8::MAX));
        if min == u8::MAX || max == u8::MAX {
            eprintln!("non int range");
            return Err(Self::Err::Parse);
        }

        let car = parts[1].chars().next().unwrap_or('\0');
        if car == '\0' {
            eprintln!("cannot read char");
            return Err(Self::Err::Parse);
        }

        Ok(Matcher {
            min,
            max,
            car,
            password: parts[2].to_string()
        })
    }
}

pub(crate) fn parse_test() -> Vec<Matcher> {
    parse_file(TEST)
}

pub(crate) fn parse_final() -> Vec<Matcher> {
    parse_file(FINAL)
}

fn parse_file(file: &str) -> Vec<Matcher> {
    let path = Path::new(file);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();
    let mut matchers = Vec::<Matcher>::new();
    for line in file {
        match line {
            Ok(line) => {
                let m = line.parse::<Matcher>();
                if m.is_ok() {
                    matchers.push(m.unwrap());
                }
            },
            _ => {}
        }
    }

    matchers
}