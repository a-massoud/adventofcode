// Lots of debugging because I missed a <= sign, but I think this is the first puzzle this year where brute force isn't the first answer.

use color_eyre::eyre;
use color_eyre::eyre::{Context, bail, eyre};
use std::{env, fs};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let input = fs::read_to_string(env::args().nth(1).ok_or(eyre!("no argument"))?)?;
    let (ranges, ids) = parse_input(&input)?;

    println!("Part 1: {}", part1(&ids, &ranges));
    println!("Part 2: {}", part2(&ranges));

    Ok(())
}

fn parse_input(input: &str) -> eyre::Result<(Vec<(i64, i64)>, Vec<i64>)> {
    let (input_ranges, input_ids) = input
        .split_once("\n\n")
        .or_else(|| input.split_once("\r\n\r\n"))
        .ok_or(eyre!("no double newline"))?;

    let ranges = input_ranges
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once('-')
                .ok_or(eyre!("range `{}` invalid", line))?;
            let a: i64 = a
                .parse()
                .with_context(|| format!("range `{}` failed to parse `{}`", line, a))?;
            let b: i64 = b
                .parse()
                .with_context(|| format!("range `{}` failed to parse `{}`", line, b))?;

            if b < a {
                bail!("range `{}-{}` is invalid", a, b);
            }

            Ok((a, b))
        })
        .collect::<eyre::Result<_>>()?;

    let ids = input_ids
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .with_context(|| format!("id `{}` failed to parse", line))
        })
        .collect::<eyre::Result<_>>()?;

    Ok((ranges, ids))
}

fn part1(ids: &[i64], ranges: &[(i64, i64)]) -> usize {
    ids.iter()
        .filter(|&id| ranges.iter().any(|&(a, b)| a <= *id && *id <= b))
        .count()
}

fn part2(ranges: &[(i64, i64)]) -> usize {
    let mut collected_ranges: Vec<(i64, i64)> = Vec::new();

    for &range in ranges {
        if collected_ranges.is_empty() {
            collected_ranges.push(range);
            continue;
        }

        let mut min = range.0;
        while min <= range.1 {
            let n = collected_ranges.iter().position(|rng| rng.0 > min);

            if let Some(i) = n {
                // i is now the index of the range that will be after our range
                if i > 0 && collected_ranges[i - 1].1 >= min {
                    min = collected_ranges[i - 1].1 + 1;
                } else if collected_ranges[i].0 <= range.1 {
                    collected_ranges.insert(i, (min, collected_ranges[i].0 - 1));
                    min = collected_ranges[i + 1].1 + 1;
                } else {
                    collected_ranges.insert(i, (min, range.1));
                    min = range.1 + 1;
                }
            } else {
                let prev_min = collected_ranges.last().unwrap().1;
                if min <= prev_min {
                    min = prev_min + 1;
                } else {
                    collected_ranges.push((min, range.1));
                    min = range.1 + 1;
                }
            }
        }
    }

    collected_ranges
        .iter()
        .map(|(a, b)| usize::try_from(b - a + 1).expect("range failed assumption"))
        .sum()
}
