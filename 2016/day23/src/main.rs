// Fun optimization problem

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, IndexMut},
    path::Path,
    str::FromStr,
};

use anyhow::{Context, Result, anyhow, bail};
use enum_map::{Enum, EnumMap, enum_map};

fn main() -> Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("No argument provided"))?;
    let program = read_input(input_path).context("Failed to read input")?;

    println!("=== Part 1 ===");
    let state = program.exec_with_state(State::from_map(enum_map! {
        Register::A => 7,
        Register::B => 0,
        Register::C => 0,
        Register::D => 0
    }));
    let reg_a = state[Register::A];
    println!("Register a: {}", reg_a);
    println!();

    println!("=== Part 2 ===");
    let state = program.exec_with_state(State::from_map(enum_map! {
        Register::A => 12,
        Register::B => 0,
        Register::C => 0,
        Register::D => 0
    }));
    let reg_a = state[Register::A];
    println!("Register a: {}", reg_a);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 1 {
            bail!("`{}` is not a single-character register", s);
        }

        match s.as_bytes()[0] {
            b'a' => Ok(Register::A),
            b'b' => Ok(Register::B),
            b'c' => Ok(Register::C),
            b'd' => Ok(Register::D),
            _ => Err(anyhow!("`{}` is not a valid register", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arg {
    Register(Register),
    Value(i64),
}

impl Arg {
    fn eval(self, state: &State) -> i64 {
        match self {
            Arg::Register(r) => state[r],
            Arg::Value(v) => v,
        }
    }
}

impl FromStr for Arg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(reg) = s.parse() {
            Ok(Arg::Register(reg))
        } else {
            let Ok(v) = s.parse() else {
                bail!("Invalid argument");
            };

            Ok(Arg::Value(v))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Copy { src: Arg, dest: Arg },
    Jnz { src: Arg, delta: Arg },
    Inc { r: Arg },
    Dec { r: Arg },
    Tgl { delta: Arg },
}

impl Instruction {
    fn try_to_delta(self) -> Option<(Register, i64)> {
        match self {
            Instruction::Inc {
                r: Arg::Register(r),
            } => Some((r, 1)),
            Instruction::Dec {
                r: Arg::Register(r),
            } => Some((r, -1)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Program(Vec<Instruction>);

#[derive(Debug, Default, Clone)]
struct State(EnumMap<Register, i64>);

impl State {
    fn new() -> Self {
        State::default()
    }

    fn from_map(map: EnumMap<Register, i64>) -> Self {
        State(map)
    }
}

impl Index<Register> for State {
    type Output = i64;

    fn index(&self, index: Register) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<Register> for State {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Program {
    #[allow(dead_code)]
    pub fn exec(&self) -> State {
        let state = State::new();
        self.exec_with_state(state)
    }

    fn try_optimize(instructions: &[Instruction], state: &mut State, ip: isize) -> Option<isize> {
        if ip > 0
            && (ip as usize) < instructions.len() - 5
            && let Instruction::Copy {
                src,
                dest: Arg::Register(add_counter_cpy),
            } = instructions[ip as usize]
            && let Instruction::Jnz {
                src: Arg::Register(add_counter),
                delta: Arg::Value(-2),
            } = instructions[ip as usize + 3]
            && let Instruction::Jnz {
                src: Arg::Register(mul_counter),
                delta: Arg::Value(-5),
            } = instructions[ip as usize + 5]
            && add_counter == add_counter_cpy
            && add_counter != mul_counter
            && Arg::Register(mul_counter) != src
            && Arg::Register(add_counter) != src
        {
            let first = instructions[ip as usize + 1].try_to_delta()?;
            let second = instructions[ip as usize + 2].try_to_delta()?;
            let mul = instructions[ip as usize + 4].try_to_delta()?;

            if mul.0 != mul_counter {
                return None;
            }

            let mul_delta = mul.1;
            if state[mul_counter].signum() != -mul_delta {
                return None;
            }

            let add_delta = if first.0 == add_counter {
                first.1
            } else if second.0 == add_counter {
                second.1
            } else {
                return None;
            };

            let (acc, acc_delta) = if first.0 != add_counter {
                first
            } else if second.0 != add_counter {
                second
            } else {
                return None;
            };

            if Arg::Register(acc) == src {
                return None;
            }

            // once confirmed that src is untouched
            let src = src.eval(state);

            if src.signum() != -add_delta {
                return None;
            }
            // we repeat state[counter] times whether it's positive or negative
            let n = src.abs();

            state[acc] += acc_delta * n * state[mul_counter].abs();
            state[mul_counter] = 0;
            state[add_counter] = 0;
            return Some(6)
        }

        if ip > 0
            && (ip as usize) < instructions.len() - 2
            && let Instruction::Jnz {
                src: Arg::Register(counter),
                delta: Arg::Value(-2),
            } = instructions[ip as usize + 2]
        {
            let first = instructions[ip as usize].try_to_delta()?;
            let second = instructions[ip as usize + 1].try_to_delta()?;

            let counter_delta = if first.0 == counter {
                first.1
            } else if second.0 == counter {
                second.1
            } else {
                return None;
            };

            if state[counter].signum() != -counter_delta {
                return None;
            }
            // we repeat state[counter] times whether it's positive or negative
            let n = state[counter].abs();

            let (acc, acc_delta) = if first.0 != counter {
                first
            } else if second.0 != counter {
                second
            } else {
                return None;
            };

            if acc_delta < 0 {
                state[acc] -= n;
            } else {
                state[acc] += n;
            }

            return Some(3);
        }

        None
    }

    pub fn exec_with_state(&self, mut state: State) -> State {
        let mut instructions = self.0.clone();
        let mut ip = 0isize;

        while 0 <= ip && (ip as usize) < instructions.len() {
            if let Some(delta_ip) = Program::try_optimize(&instructions, &mut state, ip) {
                ip += delta_ip;
                continue;
            }

            let mut delta_ip = 1;

            match instructions[ip as usize] {
                Instruction::Copy { src, dest } => {
                    let src = src.eval(&state);

                    if let Arg::Register(dest) = dest {
                        state[dest] = src;
                    }
                }
                Instruction::Jnz { src, delta } => {
                    let r = src.eval(&state);
                    let delta = delta.eval(&state);

                    if r != 0 {
                        delta_ip = delta as isize;
                    }
                }
                Instruction::Inc { r } => {
                    if let Arg::Register(r) = r {
                        state[r] += 1;
                    }
                }
                Instruction::Dec { r } => {
                    if let Arg::Register(r) = r {
                        state[r] -= 1;
                    }
                }
                Instruction::Tgl { delta } => {
                    let delta = delta.eval(&state);

                    let idx = ip + delta as isize;

                    if !(idx < 0 || idx >= instructions.len() as isize) {
                        instructions[idx as usize] = match instructions[idx as usize] {
                            Instruction::Copy { src, dest } => {
                                Instruction::Jnz { src, delta: dest }
                            }
                            Instruction::Jnz { src, delta } => {
                                Instruction::Copy { src, dest: delta }
                            }
                            Instruction::Inc { r } => Instruction::Dec { r },
                            Instruction::Dec { r } => Instruction::Inc { r },
                            Instruction::Tgl { delta } => Instruction::Inc { r: delta },
                        }
                    }
                }
            }

            ip += delta_ip;
        }

        state
    }
}

fn parse_input(input: impl BufRead) -> Result<Program> {
    let instructions: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(no, line)| {
            let line = line?;

            let mut split = line.split_whitespace();
            let instr = split
                .next()
                .ok_or_else(|| anyhow!("No instruction on line {}", no))?;

            match instr {
                "cpy" => {
                    let src_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("cpy on line {} has no arguments", no))?;
                    let dest_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("cpy on line {} has no destination", no))?;

                    let dest = dest_raw
                        .parse()
                        .with_context(|| format!("cpy on line {} has invalid dest", no))?;

                    let src = src_raw
                        .parse()
                        .with_context(|| format!("cpy on line {} has invalid source", no))?;

                    Ok(Instruction::Copy { src, dest })
                }
                "inc" => {
                    let reg_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("inc on line {} has no arguments", no))?;

                    let r = reg_raw
                        .parse()
                        .with_context(|| format!("inc on line {} has invalid register", no))?;

                    Ok(Instruction::Inc { r })
                }
                "dec" => {
                    let reg_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("dec on line {} has no arguments", no))?;

                    let r = reg_raw
                        .parse()
                        .with_context(|| format!("dec on line {} has invalid register", no))?;

                    Ok(Instruction::Dec { r })
                }
                "jnz" => {
                    let src_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("jnz on line {} has no arguments", no))?;
                    let delta_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("jnz on line {} has no delta", no))?;

                    let src = src_raw
                        .parse()
                        .with_context(|| format!("jnz on line {} has invalid source", no))?;

                    let delta = delta_raw
                        .parse()
                        .with_context(|| format!("jnz on line {} has invalid delta", no))?;

                    Ok(Instruction::Jnz { src, delta })
                }
                "tgl" => {
                    let src_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("tgl on line {} has no arguments", no))?;

                    let src = src_raw
                        .parse()
                        .with_context(|| format!("tgl on line {} has invalid argument", no))?;

                    Ok(Instruction::Tgl { delta: src })
                }
                _ => {
                    bail!("Invalid instruction on line {}: `{}`", no, instr);
                }
            }
        })
        .collect::<Result<_>>()?;

    Ok(Program(instructions))
}

fn read_input(path: impl AsRef<Path>) -> Result<Program> {
    let path = path.as_ref();
    let input = BufReader::new(
        File::open(path).with_context(|| format!("Failed to open `{}`", path.display()))?,
    );
    parse_input(input).with_context(|| format!("Failed to parse `{}`", path.display()))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{Register, parse_input};

    const ORIGINAL_SAMPLE_INPUT: &str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn original_sample_input() {
        let program = parse_input(Cursor::new(ORIGINAL_SAMPLE_INPUT)).expect("Sample input parses");
        let state = program.exec();
        assert_eq!(state[Register::A], 42);
    }

    const SAMPLE_INPUT_WITH_TOGGLE: &str = "\
cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    #[test]
    fn sample_input_with_toggle() {
        let program =
            parse_input(Cursor::new(SAMPLE_INPUT_WITH_TOGGLE)).expect("Sample input parses");
        let state = program.exec();
        assert_eq!(state[Register::A], 3);
    }
}
