use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::ops::RangeInclusive;

pub(crate) const FINAL: &str = "./inputs/exo11_final_input.txt";

#[cfg(test)]
pub(crate) const TEST: &str = "./inputs/exo11_test_input.txt";

#[derive(Debug, Clone)]
pub enum Day11ErrorKind {
    Test,
    Parse,
}

impl Display for Day11ErrorKind {
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
pub struct Day11Error {
    kind: Day11ErrorKind,
    message: String
}

impl Display for Day11Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.kind, self.message)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct Pos {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Seat {
    Empty,
    Occupied,
    Floor
}

impl TryFrom<char> for Seat {
    type Error = Day11Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Empty),
            '.' => Ok(Self::Floor),
            '#' => Ok(Self::Occupied),
            _ => Err(Self::Error { kind: Day11ErrorKind::Parse, message: "Unknown seat state".to_string() })
        }
    }
}

impl Into<char> for &Seat {
    fn into(self) -> char {
        match *self {
            Seat::Empty => 'L',
            Seat::Floor => '.',
            Seat::Occupied => '#',
        }
    }
}

#[derive(Debug, Clone)]
pub struct Boat {
    max_x: usize,
    max_y: usize,
    seats: HashMap<Pos,Seat>,
    changes: bool
}

impl Into<String> for Boat {
    fn into(self) -> String {
        let mut res: String = String::new();
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                let p: Pos = Pos {x, y};
                let c: char = self.seats.get(&p).unwrap().into();
                res.push(c);
            }
            res.push('\n');
        }
        res
    }
}

impl Boat {
    fn get_adjacent_seats(&self, pos: &Pos) -> Vec<&Seat> {
        let mut res: Vec<&Seat> = Vec::new();
        let (x_range, y_range): (RangeInclusive<usize>, RangeInclusive<usize>);
        {
            let (min_x, max_x): (usize, usize);
            let (min_y, max_y): (usize, usize);
            if pos.x == 0 {
                min_x = 0;
            } else {
                min_x = pos.x - 1;
            }
            if pos.x == self.max_x - 1 {
                max_x = self.max_x - 1;
            } else {
                max_x = pos.x + 1;
            }
            x_range = min_x..=max_x;
            if pos.y == 0 {
                min_y = 0;
            } else {
                min_y = pos.y - 1;
            }
            if pos.y == self.max_y - 1 {
                max_y = self.max_y - 1;
            } else {
                max_y = pos.y + 1;
            }
            y_range = min_y..=max_y;
        }
        for x in x_range {
            for y in y_range.clone() {
                let p: Pos = Pos {
                    x,
                    y
                };
                if self.seats.contains_key(&p) && &p != pos {
                    res.push(self.seats.get(&p).unwrap());
                }
            }
        }
        res
    }

    pub fn go_to_stable(&mut self) -> usize {
        loop {
            if !self.changes {
                break
            }
            self.changes = false;
            let mut seats: HashMap<Pos, Seat> = HashMap::new();
            let mut keys: Vec<&Pos> = self.seats.keys().collect();
            keys.sort();
            for pos in keys {
                let adj = self.get_adjacent_seats(&pos);
                match self.seats.get(pos).unwrap() {
                    Seat::Empty => {
                        if adj.iter().filter(|s| ***s == Seat::Occupied).count() == 0 {
                            seats.insert(*pos, Seat::Occupied);
                            self.changes = true;
                        } else {
                            seats.insert(*pos, self.seats.get(pos).unwrap().clone());
                        }
                    },
                    Seat::Occupied => {
                        if adj.iter().filter(|s| ***s == Seat::Occupied).count() >= 4 {
                            seats.insert(*pos, Seat::Empty);
                            self.changes = true;
                        } else {
                            seats.insert(*pos, self.seats.get(pos).unwrap().clone());
                        }
                    },
                    Seat::Floor => {
                        seats.insert(pos.clone(), Seat::Floor);
                    }
                }
            }
            self.seats = seats;
        }
        self.seats.values()
            .filter(|seat| **seat == Seat::Occupied)
            .count()
    }
}

#[cfg(test)]
pub(crate) fn parse_test() -> Result<Boat, Day11Error> { parse_file(TEST) }

pub fn parse_final() -> Result<Boat, Day11Error> { parse_file(FINAL) }

fn parse_file(path: &str) -> Result<Boat, Day11Error> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();
    let mut seats: HashMap<Pos, Seat> = HashMap::new();

    let (mut x, mut y): (usize, usize) = (0, 0);
    for line in file {
        match line {
            Ok(line) => {
                y = 0;
                for c in line.chars() {
                    seats.insert(Pos { x, y }, c.try_into()?);
                    y += 1;
                }
            },
            _ => {}
        }
        x += 1;
    }

    Ok(Boat { max_x: x, max_y: y, seats, changes: true })
}
