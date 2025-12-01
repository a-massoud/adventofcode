// Another year! There's some modular arithmetic way to do part 2, but my brain is shot, so...

use anyhow::bail;
use std::{env, fs, iter};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        bail!("do better with args");
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    let mut r = Vec::new();

    for line in input.lines() {
        if line.len() < 2 {
            bail!("invalid line `{}`", line);
        }

        let m = if line.starts_with('L') {
            -1
        } else if line.starts_with('R') {
            1
        } else {
            bail!("invalid line `{}`", line)
        };
        let c: i32 = line.split_at(1).1.parse()?;

        r.push(m * c);
    }

    Ok(r)
}

fn part1(input: &[i32]) -> usize {
    input
        .iter()
        .scan(50i32, |acc, i| {
            *acc = (*acc + i).rem_euclid(100);
            Some(*acc)
        })
        .filter(|&x| x == 0)
        .count()
}

fn part2(input: &[i32]) -> usize {
    input
        .iter()
        .flat_map(|x| iter::repeat_n(x.signum(), x.abs() as usize))
        .scan(50i32, |acc, i| {
            *acc = (*acc + i).rem_euclid(100);
            Some(*acc)
        })
        .filter(|&x| x == 0)
        .count()
}
