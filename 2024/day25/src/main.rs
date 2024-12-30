use anyhow::{bail, Context};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0]);
    }

    let input = fs::read_to_string(&args[1]).context("failed to read input file")?;
    let (locks, keys) = parse_input(&input).context("failed to parse input")?;

    println!("Number of fitting pairs: {}", get_fitting_count(&locks, &keys));

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<[i32; 5]>, Vec<[i32; 5]>)> {
    let complete: Vec<_> = input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<Vec<u8>> = block.lines().map(Vec::from).collect();
            if lines.len() != 7 {
                bail!("invalid lock/key {}", block);
            }
            for line in &lines {
                if line.len() != 5 {
                    bail!("invalid lock/key {}", block);
                }
            }
            let is_key;
            if lines[0] == [b'#', b'#', b'#', b'#', b'#']
                && lines[6] == [b'.', b'.', b'.', b'.', b'.']
            {
                is_key = false;
            } else if lines[6] == [b'#', b'#', b'#', b'#', b'#']
                && lines[0] == [b'.', b'.', b'.', b'.', b'.']
            {
                is_key = true;
            } else {
                bail!("invalid lock/key {}", block);
            }

            let mut vals = [0i32; 5];
            for (i, val) in vals.iter_mut().enumerate() {
                if is_key {
                    for line in lines.iter().take(6).skip(1).rev() {
                        if line[i] == b'.' {
                            break;
                        }
                        *val += 1;
                    }
                } else {
                    for line in lines.iter().take(6).skip(1) {
                        if line[i] == b'.' {
                            break;
                        }
                        *val += 1;
                    }
                }
            }

            Ok((is_key, vals))
        })
        .collect::<anyhow::Result<_>>()?;

    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for (is_key, vals) in complete {
        if is_key {
            keys.push(vals);
        } else {
            locks.push(vals);
        }
    }

    Ok((locks, keys))
}

fn get_fitting_count(locks: &[[i32; 5]], keys: &[[i32; 5]]) -> usize {
    locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| lock.iter().zip(key.iter()).all(|(x, y)| x + y <= 5))
                .count()
        })
        .sum()
}
