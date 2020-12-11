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
    position: (u16, u16),
    id: u16
}

impl BoadingPass {
    pub fn get_id(&self) -> u16 {
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
        let (mut i, mut row, mut col) : (u16, (u16, u16), (u16, u16)) = (0, (0, 127), (0, 7));
        let mut chars = value.chars().into_iter();
        let rowRes: u16 = loop {
            let char = chars.next().unwrap_or('\0');
            i += 1;
            if i < 7 {
                let step = ((row.1 - row.0) / 2) + 1;
                match char {
                    'B' => row.0 += step,
                    'F' => row.1 -= step,
                    _ => return Err(Self::Error {
                        kind: Day5ErrorKind::Test,
                        message: "".to_string()
                    })
                }
            } else if i == 7 {
                match char {
                    'B' => {
                        break row.1;
                    },
                    'F' => {
                        break row.0;
                    },
                    _ => {} /*return Err(Self::Error {
                        kind: Day5ErrorKind::Test,
                        message: "".to_string()
                    })*/
                }
                break 0;
            }
        };
        let colRes: u16 = loop {
            let char = chars.next().unwrap_or('\0');
            i += 1;
            if i < 10 {
                let step = ((col.1 - col.0) / 2) + 1;
                match char {
                    'R' => col.0 += step,
                    'L' => col.1 -= step,
                    _ => return Err(Self::Error {
                        kind: Day5ErrorKind::Test,
                        message: "".to_string()
                    })
                }
            } else if i == 10 {
                match char {
                    'R' => {
                        break col.1;
                    },
                    'L' => {
                        break col.0;
                    },
                    _ => return Err(Self::Error {
                        kind: Day5ErrorKind::Test,
                        message: "".to_string()
                    })
                }
                break 0;
            }
        };

        Ok(Self {
            code: value,
            position: (rowRes, colRes),
            id: (rowRes * 8 + colRes)
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