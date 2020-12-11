use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const FINAL: &str = "./inputs/exo3_final_input.txt";
const TEST: &str = "./inputs/exo3_test_input.txt";

pub(crate) struct Map {
    height: usize,
    width: usize,
    rows: Vec<Vec<bool>>
}

impl Map {
    fn grow(&self, right: u16, down: u16) -> (u16, u16) {
        let (mut trees, mut empty): (u16, u16) = (0, 0);
        let (mut i, mut j): (u16, u16) = (0, 0);
        loop {
            j += right;
            if j >= self.width as u16 {
                j = j - self.width as u16;
            }

            i += down;

            if i >= self.height as u16 {
                break
            }

            if self.rows[i as usize][j as usize] {
                trees += 1;
            } else {
                empty += 1;
            }
        }
        (trees, empty)
    }

    pub(crate) fn grow_part1(&self) -> (u16, u16) {
        self.grow(3,1)
    }

    pub(crate) fn grow_part2(&self) -> Vec<(u16, u16)> {
        let mut res = Vec::<(u16, u16)>::new();
        res.push(self.grow(1,1));
        res.push(self.grow(3,1));
        res.push(self.grow(5,1));
        res.push(self.grow(7,1));
        res.push(self.grow(1,2));
        res
    }
}

pub(crate) fn parse_test() -> Map {
    parse_file(TEST)
}

pub(crate) fn parse_final() -> Map {
    parse_file(FINAL)
}

fn parse_file(file: &str) -> Map {
    let path = Path::new(file);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let file = BufReader::new(file).lines();

    let mut rows = Vec::<Vec<bool>>::new();
    for line in file {
        let mut row = Vec::<bool>::new();
        match line {
            Ok(line) => {
                for char in line.chars() {
                    if char == '#' {
                        row.push(true);
                    } else {
                        row.push(false);
                    }
                }
            },
            _ => {}
        }
        rows.push(row);
    }

    Map {
        height: rows.len(),
        width: rows[0].len(),
        rows
    }
}
