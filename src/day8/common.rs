use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::convert::{TryInto, TryFrom};
use std::fmt::{Display, Formatter};

pub(crate) const FINAL: &str = "./inputs/exo8_final_input.txt";

#[cfg(test)]
pub(crate) const TEST: &str = "./inputs/exo8_test_input.txt";

#[derive(Debug, Clone)]
pub enum Day8ErrorKind {
    Test,
    Parse,
}

impl Display for Day8ErrorKind {
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
pub struct Day8Error {
    kind: Day8ErrorKind,
    message: String
}

impl Display for Day8Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.kind, self.message)
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum InstructionKind {
    NOP,
    JMP,
    ACC,
}

impl TryFrom<String> for InstructionKind {
    type Error = Day8Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "nop" => Ok(Self::NOP),
            "jmp" => Ok(Self::JMP),
            "acc" => Ok(Self::ACC),
            _ => Err(Self::Error { kind: Day8ErrorKind::Parse, message: "Unknown instruction".to_string()})
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Instruction {
    kind: InstructionKind,
    value: i16
}

impl TryFrom<String> for Instruction {
    type Error = Day8Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: Vec<&str> = value.split(' ').collect();
        let kind: InstructionKind = value[0].to_string().try_into().unwrap();
        let value: i16 = value[1].parse::<i16>().unwrap_or(0);
        Ok(Self {
            kind,
            value
        })
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
    acc: i16,
}

impl Program {
    pub fn get_acc(&self) -> i16 {
        self.acc.clone()
    }

    pub fn run(&mut self) {
        let mut ptr: i16 = 0;
        let mut history: Vec<i16> = Vec::new();
        loop {
            if history.contains(&ptr) {
                break
            }
            let instruction: &Instruction = &self.instructions[ptr as usize];
            match instruction.kind {
                InstructionKind::NOP => {},
                InstructionKind::JMP => {
                    history.push(ptr.clone());
                    ptr += instruction.value;
                    continue
                }
                InstructionKind::ACC => {
                    self.acc += instruction.value;
                },
            }
            history.push(ptr.clone());
            ptr += 1;
        }
    }
}

impl<'a> TryFrom<String> for Program {
    type Error = Day8Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut instructions: Vec<Instruction> = Vec::new();
        let value = value.split('\n');
        for v in value {
            let instruction: Instruction = v.to_string().try_into()?;
            instructions.push(instruction);
        }

        Ok(
            Self {
                instructions,
                acc: 0
            }
        )
    }
}

#[cfg(test)]
pub(crate) fn parse_test() ->  Result<Program, Day8Error> {
    parse_file(TEST)
}

pub fn parse_final() ->  Result<Program, Day8Error> { parse_file(FINAL) }

fn parse_file(path: &str) ->  Result<Program, Day8Error> {
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
