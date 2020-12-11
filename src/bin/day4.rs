use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

use std::error::Error as StdError;

use regex::{Regex, Captures, Match};
use std::convert::{TryFrom, TryInto};

const FINAL_P1: &str = "./inputs/exo4_final_input_p1.txt";
const TEST_P1: &str = "./inputs/exo4_test_input_p1.txt";

const FINAL_P2: &str = "./inputs/exo4_final_input_p2.txt";
const TEST_P2: &str = "./inputs/exo4_test_input_p2.txt";

// Part 1
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

// Part 2

#[derive(Debug, Clone)]
enum P2ErrorKind {
    Parse,
    HeightParse,
    ParseU16
}

impl Display for P2ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut msg: &str= "";
        match self {
            Self::Parse => msg = "Generic parse type",
            Self::HeightParse => msg = "Cannot parse height",
            Self::ParseU16 => msg = "Cannot parse integer",
            &_ => {}
        }
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct P2Error {
    kind: P2ErrorKind,
    message: String
}

impl Display for P2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error {}", self.kind)
    }
}

impl From<std::num::ParseIntError> for P2Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self {
            kind: P2ErrorKind::ParseU16,
            message: e.to_string()
        }
    }
}

#[derive(Debug, Clone)]
enum HeightKind {
    CM,
    IN
}

impl FromStr for HeightKind {
    type Err = P2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Self::CM),
            "in" => Ok(Self::IN),
            &_ => Err(Self::Err {
                kind: P2ErrorKind::HeightParse,
                message: "cannot parse height type".to_string()
            })
        }
    }
}

#[derive(Debug, Clone)]
struct Height {
    kind: HeightKind,
    height: u16
}

impl Height {
    fn is_valid(&self) -> bool {
        match self.kind {
            HeightKind::CM => self.height >= 150 && self.height <= 193,
            HeightKind::IN => self.height >= 59 && self.height <= 76,
        }
    }
}

impl TryFrom<String> for Height {
    type Error = P2Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"^(?P<height>\d{1,3})(?P<type>cm|in)$").unwrap();

        let err_captures: Self::Error = Self::Error {
            kind: P2ErrorKind::HeightParse,
            message: "Given String do not match expected formats".to_string()
        };

        let captures: Captures = reg.captures(&value).ok_or(err_captures.clone())?;

        let (height, kind): (Option<Match>, Option<Match>) = (captures.name("height"), captures.name("type"));

        Ok(Self {
            height: height.ok_or(err_captures.clone())?.as_str().parse()?,
            kind: kind.ok_or(err_captures.clone())?.as_str().parse()?
        })
    }
}

impl StdError for P2Error {

}

#[derive(Debug)]
struct PassportP2 {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: String,
}

impl PassportP2 {
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
                        let p = entry.try_into();
                        if p.is_ok() {
                            passports.push(p.unwrap());
                        }
                        entry = String::new();
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
            let p = entry.try_into();
            if p.is_ok() {
                passports.push(p.unwrap());
            }
        }

        passports
    }
}

impl TryFrom<String> for PassportP2 {
    type Error = P2Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(' ').collect();

        let mut map = HashMap::new();
        for part in parts {
            let p: Vec<&str> = part.split(':').collect();
            if p.len() != 2 {
                return Err(Self::Error{kind: P2ErrorKind::Parse, message: "cannot parse entry".to_string()});
            }
            map.insert(p[0], p[1]);
        }

        let hgt: Height = map
            .get("hgt").unwrap_or(&"\0")
            .to_string()
            .try_into()?;

        let (byr,
            iyr,
            eyr,
            hcl,
            ecl,
            pid
        ) = (map.get("byr").unwrap_or(&format!("{}", u16::MAX).as_str())
                     .parse::<u16>().unwrap_or(u16::MAX),
                map.get("iyr").unwrap_or(&format!("{}", u16::MAX).as_str())
                    .parse::<u16>().unwrap_or(u16::MAX),
                map.get("eyr").unwrap_or(&format!("{}", u16::MAX).as_str())
                     .parse::<u16>().unwrap_or(u16::MAX),
                map.get("hcl").unwrap_or(&"\0").to_string(),
                map.get("ecl").unwrap_or(&"\0").to_string(),
                map.get("pid").unwrap_or(&"\0").to_string(),
        );
        let reg = Regex::new("^#[0-9a-f]{6}$").unwrap();
        let reg2 = Regex::new("^[0-9]{9}$").unwrap();
        let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if byr >= 1920 && byr <= 2002
            && iyr >= 2010 && iyr <= 2020
            && eyr >= 2020 && eyr <= 2030
            && reg.is_match(hcl.as_str())
            && colors.contains(&ecl.as_str())
            && reg2.is_match(&pid.as_str()) &&
            hgt.is_valid()
        {
            Ok(PassportP2 {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
            })
        } else {
            Err(Self::Error{kind: P2ErrorKind::Parse, message: "invalid passport".to_string()})
        }
    }
}

fn parse_test_p1() -> Vec<PassportP1> {
    PassportP1::parse_file(TEST_P1)
}

fn parse_final_p1() -> Vec<PassportP1> {
    PassportP1::parse_file (FINAL_P1)
}

fn parse_test_p2() -> Vec<PassportP2> {
    PassportP2::parse_file(TEST_P2)
}

fn parse_final_p2() -> Vec<PassportP2> {
    PassportP2::parse_file(FINAL_P2)
}

fn part1() {
    let passports = parse_final_p1();

    println!("{}", passports.len());
}

fn part2() {
    let passports = parse_final_p2();

    println!("{}", passports.len());
}

fn main () {
    println!("Part 1:");
    part1();
    println!("Part 2:");
    part2();
}