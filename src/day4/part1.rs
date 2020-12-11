use crate::day4::common::{TEST_P1, FINAL_P1};

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    Parse,
}

#[derive(Debug)]
struct PassportP1 {
    byr: u128,
    iyr: u128,
    eyr: u128,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: u128,
}

impl PassportP1 {
    fn parse_file(file: &str) -> Vec<Self> {
        let path = Path::new(file);
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let file = BufReader::new(file).lines();

        let mut passports = Vec::<Self>::new();
        let mut entry: String = String::new();

        for line in file {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        let p = entry.parse::<Self>();
                        if p.is_ok() {
                            passports.push(p.unwrap());
                        }
                        entry.clear();
                    }
                    if !entry.is_empty() {
                        entry.push(' ');
                    }
                    entry.push_str(line.as_str());
                },
                _ => {}
            }
        }

        if !entry.is_empty() {
            let p = entry.parse::<Self>();
            if p.is_ok() {
                passports.push(p.unwrap());
            }
            entry.clear();
        }

        passports
    }
}

impl FromStr for PassportP1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let mut map = HashMap::new();
        for part in parts {
            let p: Vec<&str> = part.split(':').collect();
            if p.len() != 2 {
                return Err(Self::Err::Parse);
            }
            map.insert(p[0], p[1]);
        }

        return if map.keys().len() == 8 {
            let (byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
                cid) = (map.get("byr").unwrap().parse::<u128>().unwrap(),
                        map.get("iyr").unwrap().parse::<u128>().unwrap(),
                        map.get("eyr").unwrap().parse::<u128>().unwrap(),
                        map.get("hgt").unwrap().to_string(),
                        map.get("hcl").unwrap().to_string(),
                        map.get("ecl").unwrap().to_string(),
                        map.get("pid").unwrap().to_string(),
                        map.get("cid").unwrap().parse::<u128>().unwrap()
            );
            Ok(PassportP1 {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
                cid
            })
        } else if map.keys().len() == 7 && !map.contains_key("cid") {
            let (byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid) = (map.get("byr").unwrap().parse::<u128>().unwrap(),
                        map.get("iyr").unwrap().parse::<u128>().unwrap(),
                        map.get("eyr").unwrap().parse::<u128>().unwrap(),
                        map.get("hgt").unwrap().to_string(),
                        map.get("hcl").unwrap().to_string(),
                        map.get("ecl").unwrap().to_string(),
                        map.get("pid").unwrap().to_string(),
            );
            Ok(PassportP1 {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
                cid: u128::MAX
            })
        } else {
            return Err(Self::Err::Parse);
        }
    }
}

fn parse_test_p1() -> Vec<PassportP1> {
    PassportP1::parse_file(TEST_P1)
}

fn parse_final_p1() -> Vec<PassportP1> {
    PassportP1::parse_file (FINAL_P1)
}

pub fn exo() -> u64{
    let passports = parse_final_p1();

    passports.len() as u64
}