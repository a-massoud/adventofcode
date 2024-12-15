// Good little regex puzzle that

use std::{env, fs};

use anyhow::{bail, Context, Result};
use regex::Regex;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        bail!("Usage: {} <file>", args[0])
    }

    let input = fs::read_to_string(&args[1])?;

    println!("Part 1 result: {}", part1(&input)?);
    println!("Part 2 result: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let mul_pattern = Regex::new(r"mul\((?<X>\d+),(?<Y>\d+)\)")?;

    let mut total = 0;
    for (_, [x, y]) in mul_pattern.captures_iter(input).map(|c| c.extract()) {
        let x: i32 = x
            .parse()
            .with_context(|| format!("failed to parse `{}`", String::from(x)))?;
        let y: i32 = y
            .parse()
            .with_context(|| format!("failed to parse `{}`", String::from(y)))?;
        total += x * y;
    }

    Ok(total)
}

fn part2(input: &str) -> Result<i32> {
    let mul_pattern = Regex::new(r"mul\((?<X>\d+),(?<Y>\d+)\)|do\(\)|don't\(\)")?;

    let mut total = 0;
    let mut enabled = true;
    for c in mul_pattern.captures_iter(input) {
        let full = c.get(0).context("matched nothing")?.as_str();
        if enabled && full.as_bytes()[0] == b'm' {
            // is mul and is enabled
            let x = c.name("X").context("match incorrect: x")?.as_str();
            let y = c.name("Y").context("match incorrect: y")?.as_str();
            let x: i32 = x
                .parse()
                .with_context(|| format!("failed to parse `{}`", String::from(x)))?;
            let y: i32 = y
                .parse()
                .with_context(|| format!("failed to parse `{}`", String::from(y)))?;
            total += x * y;
        } else if full.as_bytes()[2] == b'(' {
            enabled = true;
        } else if full.as_bytes()[2] == b'n' {
            enabled = false;
        }
    }

    Ok(total)
}
