// Had to check online to get the hint to memoize. First one where I needed some spoilers. Once I
// did I'm now happy with this product. I wish I figured out a better way to memoize though. This
// is probably good enough.

use anyhow::{anyhow, bail};
use std::{collections::HashMap, env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input>", args[0]);
    }

    let input = fs::read_to_string(&args[1])?;
    let (towels, designs) = parse_input(&input)?;

    let mut cache = HashMap::new();
    let r: Vec<usize> = designs
        .iter()
        .map(|d| n_possible(d, &towels, &mut cache))
        .collect();

    println!("Part 1: {}", r.iter().filter(|&&n| n != 0).count());
    println!("Part 2: {}", r.iter().sum::<usize>());

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let (towels_input, designs_input) = input
        .split_once("\n\n")
        .ok_or(anyhow!("input not separable"))?;

    let towels: Vec<String> = towels_input
        .split(',')
        .map(|s| String::from(s.trim()))
        .collect();
    let designs: Vec<String> = designs_input
        .lines()
        .map(|s| String::from(s.trim()))
        .collect();
    Ok((towels, designs))
}

fn n_possible(design: &str, towels: &Vec<String>, cache: &mut HashMap<String, usize>) -> usize {
    let design = String::from(design);
    if cache.contains_key(&design) {
        return cache[&design];
    }
    if design.is_empty() {
        return 1;
    }

    let r = towels
        .iter()
        .filter(|&t| t.len() <= design.len() && *t == design[..t.len()])
        .map(|t| n_possible(&design[t.len()..], towels, cache))
        .sum();
    cache.insert(design, r);
    r
}
