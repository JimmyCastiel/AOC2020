use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::convert::{TryInto, TryFrom};
use std::fmt::{Display, Formatter};
use std::collections::{HashMap, VecDeque};

pub(crate) const FINAL: &str = "./inputs/exo7_final_input.txt";

#[cfg(test)]
pub(crate) const TEST: &str = "./inputs/exo7_test_input.txt";

#[derive(Debug, Clone)]
pub enum Day7ErrorKind {
    Test,
    Parse
}

impl Display for Day7ErrorKind {
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
pub struct Day7Error {
    kind: Day7ErrorKind,
    message: String
}

impl Display for Day7Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.kind, self.message)
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Color {
    name: String
}

impl TryFrom<String> for Color {
    type Error = Day7Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value
        })
    }
}

pub struct Colors {
    colors: HashMap<Color, Option<Vec<(u8,Color)>>>
}

impl Colors {
    pub fn how_many_bags_can_contain(&self, color: Color) -> u32 {
        let mut res: u32 = 0;

        for col in self.colors.keys() {
            if *col == color {
                continue
            }
            let mut colors: VecDeque<&Color> = VecDeque::new();
            colors.push_front(&col);
            loop {
                let col = colors.pop_front();
                match col {
                    Some(col) => {
                        if *col == color {
                            res += 1;
                            colors.clear();
                            break;
                        } else {
                            match self.colors.get(&col) {
                                Some(c) => {
                                    match c {
                                        Some(c) => {
                                            for c in c {
                                                if !colors.contains(&&c.1) {
                                                    colors.push_front(&c.1);
                                                }
                                            }
                                        },
                                        None => {}
                                    }
                                },
                                None => {}
                            }
                        }
                    },
                    None => break
                }
            }
        }

        res
    }
}

impl<'a> TryFrom<String> for Colors {
    type Error = Day7Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut colors: HashMap<Color, Option<Vec<(u8, Color)>>> = HashMap::new();
        let value: Vec<&str> = value.split('\n').collect();

        for sentence in value {
            let mut colors_list: Vec<(u8, Color)> = Vec::new();
            let sentence: Vec<&str> = sentence.split(" contain ").collect();

            let color: Vec<&str> = sentence[0].split(' ').collect();
            let containee: Vec<&str> = sentence[1].split(' ').collect();

            let color: Color = color[0..2].join(" ").try_into()?;
            let containee: String = containee.join(" ");

            if containee == "no other bags." {
                colors.insert(color, None);
            } else {
                let containee_sentences: Vec<&str> = containee.split(", ").collect();

                for sentence in containee_sentences {
                    let color: Vec<&str> = sentence.split(' ').collect();
                    let num: u8 = color[0].to_string().parse::<u8>().unwrap_or(0);
                    let color: Color = color[1..3].join(" ").try_into()?;
                    colors_list.push((num, color));
                }

                colors.insert(color, Some(colors_list));
            }
        }

        Ok(
            Colors {
                colors
            }
        )
    }
}

#[cfg(test)]
pub(crate) fn parse_test() ->  Result<Colors, Day7Error> {
    parse_file(TEST)
}

pub fn parse_final() ->  Result<Colors, Day7Error> { parse_file(FINAL) }

fn parse_file(path: &str) ->  Result<Colors, Day7Error> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();

    let mut buf: String = String::new();
    for line in file {
        match line {
            Ok(line) => {
                buf.push_str(&line);
                buf.push_str("\n");
            },
            _ => {}
        }
    }
    buf.pop();

    buf.try_into()
}
