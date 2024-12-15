use anyhow::{bail, Context};
use std::collections::HashMap;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0])
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    let part1 = simulate(&input, 25);
    let part2 = simulate(&part1, 50);
    println!(
        "Part 1: {}",
        part1.iter().fold(0u64, |acc, (_, count)| acc + count)
    );
    println!(
        "Part 2: {}",
        part2.iter().fold(0u64, |acc, (_, count)| acc + count)
    );

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<HashMap<u64, u64>> {
    let mut stones = HashMap::new();

    for stone in input.split_whitespace() {
        let stone: u64 = stone
            .parse()
            .with_context(|| format!("invalid stone: `{}`", String::from(stone)))?;
        if let Some(v) = stones.get_mut(&stone) {
            *v += 1;
        } else {
            stones.insert(stone, 1);
        }
    }

    Ok(stones)
}

fn get_base10_len(x: u64) -> u32 {
    let mut l = 0;
    let mut z = x;

    while z != 0 {
        l += 1;
        z /= 10;
    }

    l
}

fn blink(state: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let ins_or_inc = |s: &mut HashMap<u64, u64>, k: u64, v: u64| {
        if let Some(ptr) = s.get_mut(&k) {
            *ptr += v;
        } else {
            s.insert(k, v);
        }
    };
    let mut next = HashMap::new();

    for (stone, count) in state {
        if stone == 0 {
            ins_or_inc(&mut next, 1, count);
            continue;
        }

        let l = get_base10_len(stone);
        if l % 2 == 0 {
            let div = 10u64.pow(l / 2);
            ins_or_inc(&mut next, stone / div, count);
            ins_or_inc(&mut next, stone % div, count);
        } else {
            ins_or_inc(&mut next, stone * 2024, count);
        }
    }

    next
}

fn simulate(input: &HashMap<u64, u64>, n: u64) -> HashMap<u64, u64> {
    let mut state = input.clone();

    for _ in 0..n {
        state = blink(state);
    }

    state
}
