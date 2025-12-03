// Quite easy once I reread the prompt :)

use anyhow::{anyhow, bail};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        bail!("need argument")
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(i64, i64)>> {
    input
        .split(',')
        .map(|rng| {
            let rng = rng.split_once('-').ok_or(anyhow!("bad range: {rng}"))?;
            let start = rng.0.parse()?;
            let end = rng.1.parse()?;
            if end < start {
                bail!("bad range: {}-{}", rng.0, rng.1)
            }
            Ok::<_, anyhow::Error>((start, end))
        })
        .collect()
}

fn part1(input: &[(i64, i64)]) -> i64 {
    input
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&i| {
            let i = i.to_string();
            &i[..i.len() / 2] == &i[i.len() / 2..]
        })
        .sum()
}

fn part2(input: &[(i64, i64)]) -> i64 {
    input
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&i| {
            let i = i.to_string();
            let i = i.as_bytes();
            (1..=i.len()/2).any(|j| {
                i.len() % j == 0
                    && (j..i.len() - j + 1)
                        .step_by(j)
                        .all(|k| &i[0..j] == &i[k..k + j])
            })
        })
        .sum()
}
