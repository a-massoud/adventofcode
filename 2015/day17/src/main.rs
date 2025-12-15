use anyhow::anyhow;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    let input = read_input(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;

    println!("Part 1: {}", get_ways_to_sum(&input, 150));
    println!("Part 2: {}", get_ways_to_sum_min(&input, 150).unwrap_or(0));

    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<i64>> {
    let reader = BufReader::new(File::open(path)?);

    reader.lines().map(|line| Ok(line?.parse()?)).collect()
}

fn get_ways_to_sum(input: &[i64], target: i64) -> usize {
    (0..(1 << input.len()))
        .filter(|i| {
            input
                .iter()
                .enumerate()
                .filter_map(|(j, v)| (i & (1 << j) != 0).then(|| *v))
                .sum::<i64>()
                == target
        })
        .count()
}

fn get_ways_to_sum_min(input: &[i64], target: i64) -> Option<usize> {
    (0..(1 << input.len()))
        .filter_map(|i| {
            let r: Vec<_> = input
                .iter()
                .enumerate()
                .filter_map(|(j, v)| (i & (1 << j) != 0).then(|| *v))
                .collect();
            (r.iter().sum::<i64>() == target).then(move || r.len())
        })
        .fold(None, |acc, n| match acc {
            Some((len, count)) => {
                if n < len {
                    Some((n, 1))
                } else if n == len {
                    Some((len, count + 1))
                } else {
                    Some((len, count))
                }
            }
            None => Some((n, 1)),
        })
        .map(|(_, count)| count)
}
