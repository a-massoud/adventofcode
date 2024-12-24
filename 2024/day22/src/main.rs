// Brute force my beloved :)

use anyhow::{bail, Context};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    env, fs,
    sync::{Arc, Mutex},
};

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

    let cache = Arc::new(Mutex::new(HashMap::<(i64, i64, i64, i64), i64>::new()));
    monkeys
        .par_iter()
        .map(|(_, changes)| {
            let mut m = 0;

            for seq in changes.windows(4) {
                let mut t = 0;
                let c = cache.lock().unwrap();
                if let Some(&x) = c.get(&(seq[0], seq[1], seq[2], seq[3])) {
                    t = x;
                    drop(c);
                } else {
                    drop(c);
                    for (p, c) in &monkeys {
                        t += p
                            .get(c.windows(4).position(|x| x == seq).unwrap_or(p.len()) + 4)
                            .unwrap_or(&0);
                    }
                    cache
                        .lock()
                        .unwrap()
                        .insert((seq[0], seq[1], seq[2], seq[3]), t);
                }

                if t > m {
                    m = t;
                }
            }

            m
        })
        .max()
        .unwrap_or(0)
}
