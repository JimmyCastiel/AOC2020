use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter};

pub(crate) const FINAL: &str = "./inputs/exo9_final_input.txt";

#[cfg(test)]
pub(crate) const TEST: &str = "./inputs/exo9_test_input.txt";

#[derive(Debug, Clone)]
pub enum Day9ErrorKind {
    Test,
    Parse,
}

impl Display for Day9ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg;
        match self {
            Self::Test => msg = "test",
            Self::Parse => msg = "parse",
        }
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct Day9Error {
    kind: Day9ErrorKind,
    message: String
}

impl Display for Day9Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.kind, self.message)
    }
}

pub struct Serie {
    numbers: Vec<u64>
}

impl Serie {
    pub fn get_weakness_number(&self, depth: usize) -> u64 {
        let mut idx: usize = 0;
        loop {
            let mut sums = Vec::new();
            if idx >= self.numbers.len() {
                break
            }
            if idx >= depth {
                for num in &self.numbers[(idx - depth)..idx] {
                    for n in &self.numbers[(idx - depth)..idx] {
                        let sum = num + n;
                        if !sums.contains(&sum) {
                            sums.push(sum);
                        }
                    }
                }
                if !sums.contains(&self.numbers[idx]) {
                    return *(&self.numbers[idx]);
                }
            }
            idx += 1;
        }
        0
    }

    pub fn get_weakness_serie(&self, depth: usize) -> Vec<u64> {
        let weakness: u64 = self.get_weakness_number(depth);
        let mut idx: usize = 0;
        let mut found: bool = false;
        loop {
            let mut j: usize = idx + 1;
            if idx >= (self.numbers.len() - 1) {
                break
            }
            loop {
                if j >= (self.numbers.len() - 1) {
                    break
                }
                let sum: u64 = self.numbers[idx..=j].iter().sum();
                if sum == weakness {
                    found = true;
                } else if found {
                    return self.numbers[idx..j].to_vec();
                }
                j += 1;
            }
            idx += 1;
        }
        Vec::new()
    }
}

#[cfg(test)]
pub(crate) fn parse_test() -> Result<Serie, Day9Error> { parse_file(TEST) }

pub fn parse_final() -> Result<Serie, Day9Error> { parse_file(FINAL) }

fn parse_file(path: &str) -> Result<Serie, Day9Error> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();
    let mut numbers = Vec::new();

    for line in file {
        match line {
            Ok(line) => {
                let number = line.as_str().parse::<u64>().unwrap_or(0);
                numbers.push(number);
            },
            _ => {}
        }
    }

    Ok(Serie { numbers })
}
