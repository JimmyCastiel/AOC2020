#[cfg(test)]
use crate::day4::common::TEST_P2;

use crate::day4::common::FINAL_P2;

use regex::Regex;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::fmt::{Formatter, Display, Result as FmtResult};
use std::str::FromStr;
use std::error::Error;

#[derive(Debug, Clone)]
enum Day4ErrorKind {
    Parse,
    HeightParse,
    ParseU16
}

impl Display for Day4ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg;
        match self {
            Self::Parse => msg = "Generic parse type",
            Self::HeightParse => msg = "Cannot parse height",
            Self::ParseU16 => msg = "Cannot parse integer",
        }
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct Dat4Error {
    kind: Day4ErrorKind,
    message: String
}

impl Display for Dat4Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error {}", self.kind)
    }
}

impl From<std::num::ParseIntError> for Dat4Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self {
            kind: Day4ErrorKind::ParseU16,
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
    type Err = Dat4Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Self::CM),
            "in" => Ok(Self::IN),
            &_ => Err(Self::Err {
                kind: Day4ErrorKind::HeightParse,
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
    type Error = Dat4Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"^(?P<height>\d{1,3})(?P<type>cm|in)$").unwrap();

        let err_captures: Self::Error = Self::Error {
            kind: Day4ErrorKind::HeightParse,
            message: "Given String do not match expected formats".to_string()
        };

        let captures = reg.captures(&value).ok_or(err_captures.clone())?;

        let (height, kind) = (captures.name("height"), captures.name("type"));

        Ok(Self {
            height: height.ok_or(err_captures.clone())?.as_str().parse()?,
            kind: kind.ok_or(err_captures.clone())?.as_str().parse()?
        })
    }
}

impl Error for Dat4Error {

}

#[derive(Debug)]
pub struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
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

impl TryFrom<String> for Passport {
    type Error = Dat4Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(' ').collect();

        let mut map = HashMap::new();
        for part in parts {
            let p: Vec<&str> = part.split(':').collect();
            if p.len() != 2 {
                return Err(Self::Error{kind: Day4ErrorKind::Parse, message: "cannot parse entry".to_string()});
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
            Ok(Passport {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
            })
        } else {
            Err(Self::Error{kind: Day4ErrorKind::Parse, message: "invalid passport".to_string()})
        }
    }
}

#[cfg(test)]
pub(crate) fn parse_test() -> Vec<Passport> {
    Passport::parse_file(TEST_P2)
}

pub fn parse_final() -> Vec<Passport> { Passport::parse_file(FINAL_P2) }

pub fn exo(passports: Vec<Passport>) -> u64 {
    passports.len() as u64
}
