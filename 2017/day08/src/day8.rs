use std::{
    collections::{HashMap, hash_map}, fmt::Display, io::{self, BufRead}, num::ParseIntError, ops::{Index, IndexMut}, str::FromStr
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonOp {
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,
}

#[derive(Debug, Clone)]
struct Comparison {
    register: Register,
    cmp: ComparisonOp,
    value: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseComparisonOpError {
    #[error("Bad operation")]
    BadOp,
}

impl FromStr for ComparisonOp {
    type Err = ParseComparisonOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::Lt),
            "<=" => Ok(Self::Le),
            "==" => Ok(Self::Eq),
            "!=" => Ok(Self::Ne),
            ">=" => Ok(Self::Ge),
            ">" => Ok(Self::Gt),
            _ => Err(ParseComparisonOpError::BadOp),
        }
    }
}

impl Comparison {
    fn test(&self, memory: &Memory) -> bool {
        let register = memory.0.get(&self.register).copied().unwrap_or(0);
        match self.cmp {
            ComparisonOp::Lt => register < self.value,
            ComparisonOp::Le => register <= self.value,
            ComparisonOp::Eq => register == self.value,
            ComparisonOp::Ne => register != self.value,
            ComparisonOp::Ge => register >= self.value,
            ComparisonOp::Gt => register > self.value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Register(pub String);

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Inc { target: Register, amt: i64 },
    Dec { target: Register, amt: i64 },
}

impl Operation {
    fn apply(&self, memory: &mut Memory) {
        match self {
            Operation::Inc { target, amt } => {
                if let Some(v) = memory.0.get_mut(target) {
                    *v += *amt;
                } else {
                    memory.0.insert(target.to_owned(), *amt);
                }
            }
            Operation::Dec { target, amt } => {
                if let Some(v) = memory.0.get_mut(target) {
                    *v -= *amt;
                } else {
                    memory.0.insert(target.to_owned(), -*amt);
                }
            }
        }
    }

    fn target(&self) -> &Register {
        match self {
            Operation::Inc { target, amt: _ } => target,
            Operation::Dec { target, amt: _ } => target,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Memory(HashMap<Register, i64>);

impl Memory {
    pub fn new() -> Self {
        Memory(HashMap::default())
    }

    pub fn get(&self, r: &Register) -> &i64 {
        if let Some(v) = self.0.get(r) { v } else { &0 }
    }

    pub fn get_mut(&mut self, r: &Register) -> &mut i64 {
        if !self.0.contains_key(r) {
            self.0.insert(r.clone(), 0);
        }
        self.0.get_mut(r).expect("Inserted above")
    }
}

impl Index<&Register> for Memory {
    type Output = i64;

    fn index(&self, index: &Register) -> &Self::Output {
        self.get(index)
    }
}

impl IndexMut<&Register> for Memory {
    fn index_mut(&mut self, index: &Register) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl IntoIterator for Memory {
    type Item = (Register, i64);

    type IntoIter = hash_map::IntoIter<Register, i64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Operation,
    cmp: Comparison,
}

impl Instruction {
    fn run(&self, memory: &mut Memory) {
        if self.cmp.test(memory) {
            self.op.apply(memory);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn run(&self) -> (Memory, Memory) {
        let mut mem = Memory::new();
        let mut max_mem = Memory::new();

        for instr in &self.instructions {
            instr.run(&mut mem);
            if !max_mem.0.contains_key(instr.op.target()) || mem[instr.op.target()] > max_mem[instr.op.target()] {
                max_mem[instr.op.target()] = mem[instr.op.target()];
            }
        }

        (mem, max_mem)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReadProgramError {
    #[error("Failed to read line {no}: {e}")]
    Io { no: usize, e: io::Error },
    #[error("Unexpected newline on line {no}")]
    UnexpectedEol { no: usize },
    #[error("Unexpected token `{tok}` on line {no}")]
    UnexpectedToken { no: usize, tok: String },
    #[error("Failed to parse integer `{v}` on line {no}: {e}")]
    ParseInt {
        no: usize,
        v: String,
        e: ParseIntError,
    },
    #[error("Failed to parse comparison operator `{op}` on line {no}: {e}")]
    ParseComparisonOp {
        no: usize,
        op: String,
        e: ParseComparisonOpError,
    },
}

pub fn read_program(input: impl BufRead) -> Result<Program, ReadProgramError> {
    let instructions = input
        .lines()
        .enumerate()
        .map(|(it, line)| {
            let no = it + 1;
            let line = line.map_err(|e| ReadProgramError::Io { no, e })?;
            let mut split = line.split_whitespace();

            let target = Register(
                split
                    .next()
                    .ok_or(ReadProgramError::UnexpectedEol { no })?
                    .to_owned(),
            );
            let op_str = split.next().ok_or(ReadProgramError::UnexpectedEol { no })?;
            let amt_str = split.next().ok_or(ReadProgramError::UnexpectedEol { no })?;
            let amt = amt_str.parse().map_err(|e| ReadProgramError::ParseInt {
                no,
                v: amt_str.to_owned(),
                e,
            })?;

            let op = match op_str {
                "inc" => Operation::Inc { target, amt },
                "dec" => Operation::Dec { target, amt },
                _ => {
                    return Err(ReadProgramError::UnexpectedToken {
                        no,
                        tok: op_str.to_owned(),
                    });
                }
            };

            let if_str = split.next().ok_or(ReadProgramError::UnexpectedEol { no })?;
            if if_str != "if" {
                return Err(ReadProgramError::UnexpectedToken {
                    no,
                    tok: if_str.to_owned(),
                });
            }

            let cmp_reg = Register(
                split
                    .next()
                    .ok_or(ReadProgramError::UnexpectedEol { no })?
                    .to_owned(),
            );
            let cmp_op_str = split.next().ok_or(ReadProgramError::UnexpectedEol { no })?;
            let cmp_op = cmp_op_str
                .parse()
                .map_err(|e| ReadProgramError::ParseComparisonOp {
                    no,
                    op: cmp_op_str.to_owned(),
                    e,
                })?;
            let cmp_val_str = split.next().ok_or(ReadProgramError::UnexpectedEol { no })?;
            let cmp_val = cmp_val_str
                .parse()
                .map_err(|e| ReadProgramError::ParseInt {
                    no,
                    v: cmp_val_str.to_owned(),
                    e,
                })?;

            let cmp = Comparison {
                register: cmp_reg,
                cmp: cmp_op,
                value: cmp_val,
            };

            if let Some(tok) = split.next() {
                return Err(ReadProgramError::UnexpectedToken { no, tok: tok.to_owned() });
            }

            Ok(Instruction { op, cmp })
        })
        .collect::<Result<_, _>>()?;

    Ok(Program { instructions })
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

use crate::day8::{self, Register};

    #[test]
    fn sample_input() {
        let program = day8::read_program(Cursor::new("\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10")).expect("Sample input parses");
        let (final_mem, max_mem) = program.run();

        assert_eq!(final_mem.get(&Register("a".to_owned())), &1);
        assert_eq!(final_mem.get(&Register("b".to_owned())), &0);
        assert_eq!(final_mem.get(&Register("c".to_owned())), &-10);
        assert_eq!(max_mem.get(&Register("c".to_owned())), &10);
    }
}
