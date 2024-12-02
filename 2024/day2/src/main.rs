// It's not pretty but it works; just brute forced part 2

use anyhow::{anyhow, bail, Context, Result};
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

    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));

    Ok(())
}

fn read_input(fname: &str) -> Result<Vec<Report>> {
    let mut file = BufReader::new(
        File::open(fname).with_context(|| format!("failed to open file {}", fname))?,
    );

    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| match v.parse::<i32>() {
                    Err(_) => Err(anyhow!("failed to parse {}", v)),
                    Ok(v) => Ok(v),
                })
                .collect::<Result<Report>>()
        })
        .collect::<Result<Vec<Report>>>();

    reports
}

fn part1(input: &Vec<Report>) -> i32 {
    input
        .iter()
        .map(|report| {
            if report.len() < 2 {
                return true;
            }

            let sign = (report[1] - report[0]).signum();
            if sign == 0 {
                return false;
            }

            for i in 1..report.len() {
                let diff = report[i] - report[i - 1];
                if diff.signum() != sign || diff.abs() > 3 || diff.abs() < 1 {
                    return false;
                }
            }

            return true;
        })
        .filter(|&x| x)
        .count()
        .try_into()
        .expect("too many elements")
}

fn part2(input: &Vec<Report>) -> i32 {
    input
        .iter()
        .map(|report| {
            iter::repeat(report)
                .take(report.len() + 1)
                .zip(0..=report.len())
                .map(|(report, r)| {
                    let mut report = report.clone();
                    if r != report.len() {
                        report.remove(r);
                    }
                    if report.len() < 2 {
                        return true;
                    }

                    let sign = (report[1] - report[0]).signum();
                    if sign == 0 {
                        return false;
                    }

                    for i in 1..report.len() {
                        let diff = report[i] - report[i - 1];
                        if diff.signum() != sign || diff.abs() > 3 || diff.abs() < 1 {
                            return false;
                        }
                    }

                    return true;
                })
                .any(|v| v)
        })
        .filter(|&x| x)
        .count()
        .try_into()
        .expect("too many elements")
}
