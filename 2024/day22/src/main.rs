// Brute force my beloved :)

use anyhow::{bail, Context};
use rayon::prelude::*;
use std::{collections::HashMap, env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0]);
    }

    let input = fs::read_to_string(&args[1]).context("failed to read file")?;
    let input: Vec<i64> = input
        .lines()
        .map(|x| x.parse().context("parsing input"))
        .collect::<anyhow::Result<_>>()?;

    println!(
        "Part 1: {}",
        input.iter().map(|&x| nth_secret(x, 2000)).sum::<i64>()
    );

    println!("Part 2: {}", max_bananas(&input));

    Ok(())
}

fn next_secret(mut x: i64) -> i64 {
    x ^= x * 64;
    x %= 16777216;
    x ^= x / 32;
    x %= 16777216;
    x ^= x * 2048;
    x %= 16777216;

    x
}

fn nth_secret(mut x: i64, n: u64) -> i64 {
    for _ in 0..n {
        x = next_secret(x);
    }

    x
}

fn max_bananas(input: &[i64]) -> i64 {
    let mut monkeys = Vec::new();
    for &secret in input {
        let mut prices = Vec::new();
        let mut x = secret;
        for _ in 0..2000 {
            prices.push(x % 10);
            x = next_secret(x);
        }

        let changes = prices.windows(2).map(|c| c[1] - c[0]).collect::<Vec<_>>();
        monkeys.push((prices, changes));
    }
    let monkeys = monkeys;

    let amts = monkeys
        .into_par_iter()
        .flat_map(|(prices, changes)| {
            changes
                .par_windows(4)
                .enumerate()
                .fold(HashMap::new, move |mut amts, (i, seq)| {
                    if let Some(x) = amts.get_mut(&(seq[0], seq[1], seq[2], seq[3])) {
                        *x += prices[i + 4];
                    } else {
                        amts.insert((seq[0], seq[1], seq[2], seq[3]), prices[i + 4]);
                    }

                    amts
                })
                .collect_vec_list()
        })
        .flatten()
        .reduce(HashMap::new, |mut acc, amts| {
            for (seq, amt) in amts {
                if let Some(x) = acc.get_mut(&seq) {
                    *x += amt;
                } else {
                    acc.insert(seq, amt);
                }
            }
            acc
        });

    amts.into_par_iter().map(|(_, x)| x).max().unwrap_or(0)
}
