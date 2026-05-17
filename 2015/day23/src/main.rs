use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use enum_map::{EnumMap, enum_map};
use thiserror::Error;

fn main() {
    let input_fname = match env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("no argument provided");
            return;
        }
    };

    let program = match parse_input(input_fname) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse input: {}", e);
            return;
        }
    };

    let state = State::new(&program);
    let regs = state.run();
    println!("Part 1: {}", regs[Register::B]);

    let state = State::new_with_state(&program, enum_map! { Register::A => 1, Register::B => 0 });
    let regs = state.run();
    println!("Part 2: {}", regs[Register::B]);
}

#[derive(Debug, Clone, Copy, enum_map::Enum)]
enum Register {
    A,
    B,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

fn parse_input(input_file: impl AsRef<Path>) -> Result<Vec<Instruction>, ParseInputError> {
    fn parse_register(reg: &str) -> Result<Register, ParseInputError> {
        match reg {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err(ParseInputError::BadRegister(reg.to_owned())),
        }
    }

    fn parse_offset(offset: &str) -> Result<isize, ParseInputError> {
        if offset.len() < 2 {
            return Err(ParseInputError::BadOffset(offset.to_owned()));
        }

        let ch = offset.bytes().next().unwrap(); // just verified above
        if ch != b'+' && ch != b'-' {
            return Err(ParseInputError::BadOffset(offset.to_owned()));
        }

        offset
            .parse()
            .map_err(|_| ParseInputError::BadOffset(offset.to_owned()))
    }

    let input = BufReader::new(File::open(input_file).map_err(ParseInputError::FileOpen)?);

    input
        .lines()
        .map(|line| {
            let line = match line {
                Ok(v) => v,
                Err(e) => return Err(ParseInputError::ReadLine(e)),
            };

            let (instruction, args) = match line.trim().split_once(char::is_whitespace) {
                Some(v) => v,
                None => return Err(ParseInputError::BadLine(line)),
            };

            match instruction.trim() {
                "hlf" => Ok(Instruction::Half(parse_register(args.trim())?)),
                "tpl" => Ok(Instruction::Triple(parse_register(args.trim())?)),
                "inc" => Ok(Instruction::Increment(parse_register(args.trim())?)),
                "jmp" => Ok(Instruction::Jump(parse_offset(args.trim())?)),
                "jie" => {
                    let (reg, offset) = match args.split_once(',') {
                        Some(v) => v,
                        None => return Err(ParseInputError::BadArgument(args.to_owned())),
                    };

                    Ok(Instruction::JumpIfEven(
                        parse_register(reg.trim())?,
                        parse_offset(offset.trim())?,
                    ))
                }
                "jio" => {
                    let (reg, offset) = match args.split_once(',') {
                        Some(v) => v,
                        None => return Err(ParseInputError::BadArgument(args.to_owned())),
                    };

                    Ok(Instruction::JumpIfOne(
                        parse_register(reg.trim())?,
                        parse_offset(offset.trim())?,
                    ))
                }
                _ => Err(ParseInputError::BadInstruction(instruction.to_owned())),
            }
        })
        .collect()
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("failed to open file: {0}")]
    FileOpen(io::Error),
    #[error("failed to read line: {0}")]
    ReadLine(io::Error),
    #[error("bad line: {0}")]
    BadLine(String),
    #[error("unrecognized instruction: {0}")]
    BadInstruction(String),
    #[error("bad argument in line: {0}")]
    BadArgument(String),
    #[error("bad register name: {0}")]
    BadRegister(String),
    #[error("bad offset: {0}")]
    BadOffset(String),
}

#[derive(Debug, Clone, Copy)]
struct State<'a> {
    registers: EnumMap<Register, i64>,
    idx: isize,
    program: &'a [Instruction],
}

impl<'a> State<'a> {
    fn new(program: &'a [Instruction]) -> Self {
        Self {
            registers: enum_map! {
                Register::A => 0,
                Register::B => 0
            },
            idx: 0,
            program,
        }
    }

    fn new_with_state(program: &'a [Instruction], state: EnumMap<Register, i64>) -> Self {
        Self {
            registers: state,
            idx: 0,
            program,
        }
    }

    fn halted(&self) -> bool {
        self.idx < 0 || self.program.len() <= (self.idx as usize)
    }

    fn step(&mut self) {
        if self.halted() {
            return;
        }

        let mut should_step = true;

        match self.program[self.idx as usize] {
            Instruction::Half(r) => {
                self.registers[r] /= 2;
            }
            Instruction::Triple(r) => {
                self.registers[r] *= 3;
            }
            Instruction::Increment(r) => {
                self.registers[r] += 1;
            }
            Instruction::Jump(offset) => {
                self.idx += offset;
                should_step = false;
            }
            Instruction::JumpIfEven(r, offset) => {
                if self.registers[r] % 2 == 0 {
                    self.idx += offset;
                    should_step = false;
                }
            }
            Instruction::JumpIfOne(r, offset) => {
                if self.registers[r] == 1 {
                    self.idx += offset;
                    should_step = false;
                }
            }
        }

        if should_step {
            self.idx += 1;
        }
    }

    fn run(mut self) -> EnumMap<Register, i64> {
        while !self.halted() {
            self.step();
        }

        self.registers
    }
}
