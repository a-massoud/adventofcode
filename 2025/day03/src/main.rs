// not bad at all, I enjoyed this one. always nice when part 2 rewrites your part 1 solution more cleanly

use anyhow::{anyhow, bail};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        bail!("incorrect arguments");
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    let p1: usize = input.iter().map(|i| calc_joltage(i, 2)).sum();
    println!("Part 1: {}", p1);

    let p2: usize = input.iter().map(|i| calc_joltage(i, 12)).sum();
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|i| {
                    if b'0' <= i && i <= b'9' {
                        Ok(i - b'0')
                    } else {
                        Err(anyhow!("bad input `{}`", line))
                    }
                })
                .collect()
        })
        .collect()
}

fn calc_joltage(battery: &[u8], n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    if battery.len() <= n {
        return battery
            .iter()
            .enumerate()
            .map(|(i, &j)| 10usize.pow((battery.len() - i - 1) as u32) * j as usize)
            .sum();
    }

    let (i, &m) = battery[..battery.len() - n + 1]
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    10usize.pow(n as u32 - 1) * (m as usize) + calc_joltage(&battery[i + 1..], n - 1)
}
