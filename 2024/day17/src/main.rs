// I caved and used recursion. Why did it take me so long to implement DFS?????

use anyhow::{anyhow, bail, Context};
use std::{env, fs};

#[derive(Debug, Default, Clone, Copy)]
struct State {
    a: u64,
    b: u64,
    c: u64,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0]);
    }

    let input = fs::read_to_string(&args[1])
        .with_context(|| format!("failed reading from `{}`", args[1]))?;
    let (state, ops) = parse_input(&input).context("parsing input")?;

    println!(
        "Part 1: {}",
        String::from(ProgramIt::new_with_state(state, &ops))
    );

    println!("Part 2: {}", find_min_a(&ops)?);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<(State, Vec<u8>)> {
    let mut lines = input.lines();
    let a = lines
        .next()
        .ok_or(anyhow!("input too short"))?
        .split_once(": ")
        .ok_or(anyhow!("malformed line"))?
        .1
        .parse()?;
    let b = lines
        .next()
        .ok_or(anyhow!("input too short"))?
        .split_once(": ")
        .ok_or(anyhow!("malformed line"))?
        .1
        .parse()?;
    let c = lines
        .next()
        .ok_or(anyhow!("input too short"))?
        .split_once(": ")
        .ok_or(anyhow!("malformed line"))?
        .1
        .parse()?;

    let ops = lines
        .nth(1)
        .ok_or(anyhow!("input too short"))?
        .split_once(": ")
        .ok_or(anyhow!("malformed line"))?
        .1
        .split(',')
        .map(|i| i.parse::<u8>())
        .collect::<Result<_, _>>()?;

    Ok((State { a, b, c }, ops))
}

fn combo_op(op: u8, state: &State) -> u64 {
    match op {
        0..=3 => op as u64,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => u64::MAX,
    }
}

#[derive(Debug, Clone, Copy)]
struct ProgramIt<'a> {
    state: State,
    ip: usize,
    program: &'a [u8],
}

impl<'a> ProgramIt<'a> {
    fn new_with_state(state: State, program: &'a [u8]) -> Self {
        Self {
            state,
            ip: 0,
            program,
        }
    }

    fn new_with_a(a: u64, program: &'a [u8]) -> Self {
        Self {
            state: State { a, b: 0, c: 0 },
            ip: 0,
            program,
        }
    }
}

impl Iterator for ProgramIt<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.ip < self.program.len() - 1 {
            let op = self.program[self.ip + 1];
            match self.program[self.ip] {
                0 => {
                    self.state.a >>= combo_op(op, &self.state);
                }
                1 => {
                    self.state.b ^= op as u64;
                }
                2 => {
                    self.state.b = combo_op(op, &self.state) % 8;
                }
                3 => {
                    if self.state.a != 0 {
                        self.ip = (op as usize).wrapping_sub(2);
                    }
                }
                4 => self.state.b ^= self.state.c,
                5 => {
                    self.ip += 2;
                    return Some((combo_op(op, &self.state) % 8) as u8);
                }
                6 => {
                    self.state.b = self.state.a >> combo_op(op, &self.state);
                }
                7 => {
                    self.state.c = self.state.a >> combo_op(op, &self.state);
                }
                _ => (),
            }

            self.ip = self.ip.wrapping_add(2);
        }

        None
    }
}

impl From<ProgramIt<'_>> for String {
    fn from(value: ProgramIt<'_>) -> Self {
        let mut s = value
            .map(|i| i.to_string())
            .fold(String::new(), |acc, i| acc + &i + ",");
        s.pop();
        s
    }
}

fn find_min_a(program: &[u8]) -> anyhow::Result<u64> {
    fn recurse(a: u64, k: usize, program: &[u8]) -> Option<u64> {
        if k > program.len() {
            return Some(a);
        }

        ((a << 3)..((a + 1) << 3))
            .filter_map(|na| {
                if program
                    .iter()
                    .skip(program.len() - k)
                    .cloned()
                    .eq(ProgramIt::new_with_a(na, program))
                {
                    recurse(na, k + 1, program)
                } else {
                    None
                }
            })
            .next()
    }

    if program.is_empty() {
        return Ok(0);
    }

    if program.chunks(2).any(|x| x[0] == 0 && x[1] != 3) {
        bail!("program not operating on A in chunks of 3");
    }

    (0..(1 << 3))
        .filter_map(|a| recurse(a, 1, program))
        .next()
        .ok_or(anyhow!("failed to find valid A"))
}
