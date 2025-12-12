// well, that was disappointing. I'm going to have to go back through and figure out a better way
// to do this; one that hopefully involves solving the sample LOL

use anyhow::anyhow;
use regex::Regex;
use std::cell::LazyCell;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let input = parse_input(&input)?;

    println!("Part 1: {}", part1(&input));

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<((usize, usize), Vec<usize>)>> {
    let lines: Vec<_> = input.lines().collect();

    let split: Vec<_> = lines.split(|s| s.trim().is_empty()).collect();

    let last = split.last().ok_or(anyhow!("no double newlines"))?;

    const RE: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"^([0-9]+)x([0-9]+):((:? [0-9]+)+)$").unwrap());

    last.iter()
        .map(|s| {
            let (_, [a, b, c, _]) = RE.captures(s).ok_or(anyhow!("bad line `{}`", s))?.extract();

            let dim = a.parse()?;
            let dim2 = b.parse()?;

            let v = c
                .split_whitespace()
                .map(str::parse::<usize>)
                .collect::<Result<_, _>>()?;

            Ok(((dim, dim2), v))
        })
        .collect()
}

fn part1(input: &[((usize, usize), impl AsRef<[usize]>)]) -> usize {
    input.iter().filter(|((a, b), c)| {
        let c = c.as_ref();
        a * b >= c.iter().map(|i| i * 9).sum()
    }).count()
}
