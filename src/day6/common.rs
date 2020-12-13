use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::convert::{TryInto, TryFrom};
use std::fmt::{Display, Formatter};
use std::collections::HashMap;

pub(crate) const FINAL: &str = "./inputs/exo6_final_input.txt";

#[cfg(test)]
pub(crate) const TEST: &str = "./inputs/exo6_test_input.txt";

#[derive(Debug, Clone)]
pub enum Day6ErrorKind {
    Test
}

impl Display for Day6ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg;
        match self {
            Self::Test => msg = "test",
        }
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct Day6Error {
    kind: Day6ErrorKind,
    message: String
}

impl Display for Day6Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.kind, self.message)
    }
}

#[derive(Debug, Clone)]
pub struct VotingGroup {
    voters: u16,
    scores: HashMap<char, u16>,
}

impl VotingGroup {
    pub fn nb_q_anyone_answered_yes(&self) -> u16 {
        self.scores.keys().len() as u16
    }

    pub fn nb_q_everyone_answered_yes(&self) -> u16 {
        self.scores.keys().fold(0, | acc, c | {
            acc + if *self.scores.get(c).unwrap_or(&0) == self.voters {
                1
            } else {
                0
            }
        })
    }
}

impl TryFrom<String> for VotingGroup {
    type Error = Day6Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (mut voters, mut scores) = (0, HashMap::new());
        let mut iter = value.chars();
        loop {
            let char = iter.next();
            match char {
                Some(char) => {
                    match char {
                        '\n' => voters += 1,
                        c => {
                            let s = *scores.get(&c).unwrap_or(&0);

                            scores.insert(c, s + 1);
                        },
                    }
                },
                None => break
            }
        };
        Ok(VotingGroup {
            voters,
            scores
        })
    }
}

#[cfg(test)]
pub(crate) fn parse_test() -> Vec<VotingGroup> { parse_file(TEST) }

pub fn parse_final() -> Vec<VotingGroup> {
    parse_file(FINAL)
}

fn parse_file(path: &str) -> Vec<VotingGroup> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();

    let mut voting_groups = Vec::<VotingGroup>::new();
    let mut entry: String = String::new();

    for line in file {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    let vg = entry.try_into();
                    if vg.is_ok() {
                        voting_groups.push(vg.unwrap());
                    }
                    entry = "".to_string();
                } else {
                    entry.push_str(line.as_str());
                    entry.push('\n');
                }
            },
            _ => {}
        }
    }

    if !entry.is_empty() {
        let vg = entry.try_into();
        if vg.is_ok() {
            voting_groups.push(vg.unwrap());
        }
    }

    voting_groups
}