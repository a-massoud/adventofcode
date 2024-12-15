// It's not pretty but it works; just brute forced part 2

use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::{BufReader, Read};
use std::{env, iter};

type Report = Vec<i32>;

fn main() -> Result<()> {
    let argv: Vec<_> = env::args().collect();
    if argv.len() < 2 {
        bail!("Usage: {} <file>", argv[0]);
    }

    let input = read_input(&argv[1])?;

    println!("Part 1 result: {}", part1(&input)?);
    println!("Part 2 result: {}", part2(&input)?);

    Ok(())
}

fn read_input(fname: &str) -> Result<Vec<Report>> {
    let mut file = BufReader::new(
        File::open(fname)
            .with_context(|| format!("failed to open file {}", String::from(fname)))?,
    );

    let mut input = String::new();
    file.read_to_string(&mut input)?;

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| {
                    v.parse()
                        .with_context(|| format!("failed to parse `{}`", String::from(v)))
                })
                .collect()
        })
        .collect()
}

fn is_safe(report: &Report) -> bool {
    if report.len() < 2 {
        return true;
    }

    let sign = (report[1] - report[0]).signum();
    if sign == 0 {
        return false;
    }

    !report.windows(2).any(|i| {
        let diff = i[1] - i[0];
        diff.signum() != sign || diff.abs() > 3 || diff.abs() < 1
    })
}

fn part1(input: &Vec<Report>) -> Result<i32> {
    input
        .iter()
        .map(is_safe)
        .filter(|&x| x)
        .count()
        .try_into()
        .context("result too large")
}

fn part2(input: &Vec<Report>) -> Result<i32> {
    input
        .iter()
        .map(|report| {
            iter::repeat(report)
                .take(report.len())
                .zip(0..report.len())
                .map(|(report, r)| {
                    let mut report = report.clone();
                    report.remove(r);
                    is_safe(&report)
                })
                .any(|v| v)
        })
        .filter(|&x| x)
        .count()
        .try_into()
        .context("result too large")
}
