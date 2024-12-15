// Brute force!

use anyhow::{anyhow, bail, Context};
use rayon::prelude::*;
use std::{env, fs};

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mult,
    Concat,
}

impl Operator {
    pub fn exec(&self, x: i128, y: i128) -> i128 {
        match *self {
            Operator::Add => x + y,
            Operator::Mult => x * y,
            Operator::Concat => x * 10i128.pow(((y as f64).log10().floor() as u32) + 1) + y,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Usage: {}", args[0])
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    println!(
        "Part 1 results: {}",
        check_input(&input, &[Operator::Add, Operator::Mult])
    );
    println!(
        "Part 2 results: {}",
        check_input(&input, &[Operator::Add, Operator::Mult, Operator::Concat])
    );

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(Vec<i128>, i128)>> {
    let mut data = Vec::new();

    for line in input.lines() {
        let (res, args) = line
            .split_once(':')
            .ok_or_else(|| anyhow!("invalid line: `{}`", String::from(line)))?;
        let res: i128 = res.parse()?;
        let args: Vec<i128> = args
            .split_whitespace()
            .map(|x| {
                x.parse::<i128>()
                    .with_context(|| anyhow!("invalid line: `{}`", String::from(line)))
            })
            .collect::<anyhow::Result<_>>()?;
        data.push((args, res));
    }

    Ok(data)
}

fn is_possible(args: &Vec<i128>, res: i128, ops: &[Operator]) -> bool {
    let n: u128 = args.len().try_into().unwrap();
    let radix = ops.len() as u128;

    (0..((ops.len() as u128).pow((n + 1) as u32)))
        .into_par_iter()
        .map(|i| {
            (0u128..(n - 1))
                .map(|j| {
                    let x = if j != 0 { i / (radix.pow(j as u32)) } else { i };
                    let m = x % radix;
                    ops[m as usize]
                })
                .enumerate()
                .fold(args[0], |acc, (j, op)| op.exec(acc, args[j + 1]))
                == res
        })
        .any(|i| i)
}

fn check_input(input: &Vec<(Vec<i128>, i128)>, ops: &[Operator]) -> i128 {
    input
        .iter()
        .filter_map(|(args, res)| {
            if is_possible(args, *res, ops) {
                Some(res)
            } else {
                None
            }
        })
        .fold(0i128, |acc, i| acc + i)
}
