use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::convert::{TryInto, TryFrom};
use std::fmt::{Display, Formatter};
use std::cmp::Ordering;

pub(crate) const FINAL: &str = "./inputs/exo5_final_input.txt";
pub(crate) const TEST: &str = "./inputs/exo5_test_input.txt";

#[derive(Debug, Clone)]
pub enum Day5ErrorKind {
    Test
}

impl Display for Day5ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = "";
        match self {
            Self::Test => msg = "test",
        }
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct Day5Error {
    kind: Day5ErrorKind,
    message: String
}

impl Display for Day5Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.kind, self.message)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct BoadingPass {
    code: String,
    position: (u32, u32),
    id: u32
}

impl BoadingPass {
    pub fn get_id(&self) -> u32 {
        self.id.clone()
    }
}

impl Ord for BoadingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for BoadingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<String> for BoadingPass {
    type Error = Day5Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let first_half = value[..7].to_string();
        let second_half = value[7..].to_string();

        let first = first_half.chars().fold(0, | acc, c |{
            acc * 2 + if c != 'F' {
                1
            } else {
                0
            }
        });

        let second = second_half.chars().fold(0, | acc, c |{
            acc * 2 + if c != 'L' {
                1
            } else {
                0
            }
        });

        Ok(Self {
            code: value,
            position: (first, second),
            id: ((first * 8) + second)
        })
    }
}

pub(crate) fn parse_test() -> Vec<BoadingPass> {
    parse_file(TEST)
}

pub(crate) fn parse_final() -> Vec<BoadingPass> {
    parse_file(FINAL)
}

fn parse_file(path: &str) -> Vec<BoadingPass> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();

    let mut boardingPasses = Vec::<BoadingPass>::new();
    for line in file {
        let bp = line.unwrap_or("".to_string()).try_into();
        if bp.is_ok() {
            boardingPasses.push(bp.unwrap());
        }
    }

    boardingPasses
}