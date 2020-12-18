use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter};
use std::collections::VecDeque;

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

#[cfg(test)]
pub(crate) fn parse_test() -> Result<u64, Day9Error> { parse_file(TEST, 5) }

pub fn parse_final() -> Result<u64, Day9Error> { parse_file(FINAL, 25) }

fn parse_file(path: &str, depth: usize) -> Result<u64, Day9Error> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();
    let mut numbers = VecDeque::new();
    let mut idx: usize = 0;

    for line in file {
        if numbers.len() > depth {
            numbers.pop_back();
        }
        match line {
            Ok(line) => {
                let mut sums = Vec::new();
                let number = line.as_str().parse::<u64>().unwrap_or(0);
                for num in &numbers {
                    for n in &numbers {
                        let sum = num + n;
                        if !sums.contains(&sum) {
                            sums.push(sum);
                        }

                    }
                }
                numbers.push_front(number);
                if idx > (depth - 1) && !sums.contains(&number) {
                    return Ok(number);
                }
            },
            _ => {}
        }
        idx += 1;
    }

    Ok(0)
}
