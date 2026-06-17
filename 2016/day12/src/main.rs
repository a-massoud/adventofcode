use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result, anyhow, bail};

fn main() -> Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("no argument provided"))?;
    let program = read_input(input_path).context("reading input")?;

    println!("=== Part 1 ===");
    let state = program.exec();
    let reg_a = state
        .0
        .get(&Register('a'))
        .ok_or(anyhow!("register `a` not set"))?;
    println!("`a`: {}", reg_a);
    println!();

    println!("=== Part 2 ===");
    let state = program.exec_with_state(State(HashMap::from([(Register('c'), 1)])));
    let reg_a = state
        .0
        .get(&Register('a'))
        .ok_or(anyhow!("register `a` not set"))?;
    println!("`a`: {}", reg_a);
    println!();

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Register(char);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Source {
    Register(Register),
    Value(i64),
}

#[derive(Debug, Clone)]
enum Instruction {
    Copy { src: Source, dest: Register },
    Jnz { src: Source, delta: isize },
    Inc { r: Register },
    Dec { r: Register },
}

#[derive(Debug, Clone)]
struct Program(Vec<Instruction>);

#[derive(Debug, Clone)]
struct State(HashMap<Register, i64>);

impl Program {
    pub fn exec(&self) -> State {
        let state = State(HashMap::new());
        self.exec_with_state(state)
    }

    pub fn exec_with_state(&self, mut state: State) -> State {
        let mut ip = 0isize;
        while 0 <= ip && (ip as usize) < self.0.len() {
            let mut delta_ip = 1;

            match self.0[ip as usize] {
                Instruction::Copy { src, dest } => {
                    let src = match src {
                        Source::Register(register) => state.0.get(&register).copied().unwrap_or(0),
                        Source::Value(v) => v,
                    };
                    state.0.insert(dest, src);
                }
                Instruction::Jnz { src, delta } => {
                    let r = match src {
                        Source::Register(register) => state.0.get(&register).copied().unwrap_or(0),
                        Source::Value(v) => v,
                    };
                    if r != 0 {
                        delta_ip = delta;
                    }
                }
                Instruction::Inc { r } => {
                    state.0.entry(r).and_modify(|v| *v += 1).or_insert(1);
                }
                Instruction::Dec { r } => {
                    state.0.entry(r).and_modify(|v| *v -= 1).or_insert(-1);
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
                .ok_or_else(|| anyhow!("no instruction on line {}", no))?;

            match instr {
                "cpy" => {
                    let src_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("cpy on line {} has no arguments", no))?;
                    let dest_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("cpy on line {} has no destination", no))?;

                    let mut dest_chars = dest_raw.chars();
                    let dest_name = dest_chars
                        .next()
                        .ok_or_else(|| anyhow!("cpy on line {} has no destination register", no))?;
                    if dest_chars.next().is_some() {
                        bail!(
                            "cpy on line {} has non-register destination `{}`",
                            no,
                            dest_raw
                        );
                    }
                    let dest = Register(dest_name);

                    let src: Source;
                    if let Ok(v) = src_raw.parse() {
                        src = Source::Value(v);
                    } else {
                        let mut src_chars = src_raw.chars();
                        let src_name = src_chars
                            .next()
                            .ok_or_else(|| anyhow!("cpy on line {} has no source register", no))?;
                        if src_chars.next().is_some() {
                            bail!("cpy on line {} has non-register source `{}`", no, src_raw);
                        }
                        src = Source::Register(Register(src_name));
                    }

                    Ok(Instruction::Copy { src, dest })
                }
                "inc" => {
                    let reg_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("inc on line {} has no arguments", no))?;

                    let mut reg_chars = reg_raw.chars();
                    let reg_name = reg_chars
                        .next()
                        .ok_or_else(|| anyhow!("inc on line {} has no register", no))?;
                    if reg_chars.next().is_some() {
                        bail!("inc on line {} has non-register argument `{}`", no, reg_raw);
                    }

                    Ok(Instruction::Inc {
                        r: Register(reg_name),
                    })
                }
                "dec" => {
                    let reg_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("dec on line {} has no arguments", no))?;

                    let mut reg_chars = reg_raw.chars();
                    let reg_name = reg_chars
                        .next()
                        .ok_or_else(|| anyhow!("dec on line {} has no register", no))?;
                    if reg_chars.next().is_some() {
                        bail!("dec on line {} has non-register argument `{}`", no, reg_raw);
                    }

                    Ok(Instruction::Dec {
                        r: Register(reg_name),
                    })
                }
                "jnz" => {
                    let src_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("jnz on line {} has no arguments", no))?;
                    let delta_raw = split
                        .next()
                        .ok_or_else(|| anyhow!("jnz on line {} has no delta", no))?;

                    let reg: Source;
                    if let Ok(v) = src_raw.parse() {
                        reg = Source::Value(v);
                    } else {
                        let mut src_chars = src_raw.chars();
                        let src_name = src_chars
                            .next()
                            .ok_or_else(|| anyhow!("jnz on line {} has no source register", no))?;
                        if src_chars.next().is_some() {
                            bail!("jnz on line {} has non-register source `{}`", no, src_raw);
                        }
                        reg = Source::Register(Register(src_name));
                    }

                    let delta = delta_raw.parse().with_context(|| {
                        anyhow!("jmp on line {} has invalid delta `{}`", no, delta_raw)
                    })?;

                    Ok(Instruction::Jnz { src: reg, delta })
                }
                _ => {
                    bail!("invalid instruction on line {}: `{}`", no, instr);
                }
            }
        })
        .collect::<Result<_>>()?;

    Ok(Program(instructions))
}

fn read_input(path: impl AsRef<Path>) -> Result<Program> {
    let input = BufReader::new(
        File::open(&path).with_context(|| format!("opening `{}`", path.as_ref().display()))?,
    );
    parse_input(input)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{Register, parse_input};

    const SAMPLE_INPUT: &str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn sample_input() {
        let program = parse_input(Cursor::new(SAMPLE_INPUT)).expect("failed to parse input");
        let state = program.exec();
        assert_eq!(state.0.get(&Register('a')), Some(&42));
    }
}
