// I'm not happy about this but I do need to study.

use anyhow::{Context, anyhow, bail};
use std::ops::Mul;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("needs argument"))?)?;

    let p1_input = parse_part1(&input)?;
    println!("Part 1: {}", eval_ops(&p1_input));
    let p2_input = parse_part2(&input)?;
    println!("Part 2: {}", eval_ops(&p2_input));

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Add,
    Mul,
}

fn parse_part1(input: &str) -> anyhow::Result<Vec<(Operation, Vec<i64>)>> {
    let mut lines = input.lines().rev();
    let op_line = lines.next().ok_or(anyhow!("empty input"))?;
    let lines = lines.rev();

    let mut r: Vec<_> = op_line
        .split_whitespace()
        .map(|op| {
            match op {
                "+" => Ok(Operation::Add),
                "*" => Ok(Operation::Mul),
                _ => Err(anyhow!("bad operator `{}`", op)),
            }
            .map(|x| (x, Vec::new()))
        })
        .collect::<anyhow::Result<_>>()?;

    for line in lines {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>())
            .collect::<Result<_, _>>()
            .with_context(|| format!("in line `{}`", line))?;

        if nums.len() != r.len() {
            bail!("line `{}` contains incorrect amount of numbers", line);
        }

        for (i, &num) in nums.iter().enumerate() {
            r[i].1.push(num);
        }
    }

    Ok(r)
}

fn parse_part2(input: &str) -> anyhow::Result<Vec<(Operation, Vec<i64>)>> {
    let mut lines = input.lines().rev();
    let op_line = lines.next().ok_or(anyhow!("empty input"))?;
    let lines: Vec<Vec<_>> = lines.rev().map(|x| x.chars().collect()).collect();

    let mut r: Vec<_> = op_line
        .split_whitespace()
        .map(|op| {
            match op {
                "+" => Ok(Operation::Add),
                "*" => Ok(Operation::Mul),
                _ => Err(anyhow!("bad operator `{}`", op)),
            }
            .map(|x| (x, Vec::new()))
        })
        .collect::<anyhow::Result<_>>()?;

    if lines.is_empty() {
        return Ok(r);
    }

    let n = lines.iter().map(|x| x.len()).max().unwrap_or(0);
    let lines: Vec<_> = (0..n)
        .map(|i| {
            let line = lines
                .iter()
                .flat_map(|line| line.get(i))
                .collect::<String>();
            let line = line.trim();
            if line.is_empty() {
                Ok(None)
            } else {
                line.parse::<i64>()
                    .map(|x| Some(x))
                    .map_err(anyhow::Error::from)
            }
        })
        .collect::<anyhow::Result<_>>()?;

    for (i, set) in lines.split(|x| x.is_none()).enumerate() {
        if i >= r.len() {
            bail!("too many numbers");
        }
        r[i].1.extend(set.iter().cloned().map(Option::unwrap));
    }

    Ok(r)
}

fn eval_ops(input: &[(Operation, impl AsRef<[i64]>)]) -> i64 {
    input
        .iter()
        .map(|(op, n)| {
            let n = n.as_ref();
            match op {
                Operation::Add => n.iter().cloned().sum(),
                Operation::Mul => n.iter().cloned().reduce(i64::mul).unwrap_or(1),
            }
        })
        .sum()
}
