// I'm actually proud of this one

use anyhow::{anyhow, bail, Context, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{env, fs};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Usage: {} <input file>", args[0]);
    }

    let input = fs::read_to_string(&args[1])?;

    let (ords, lists) = parse_input(&input)?;

    println!("Part 1 results: {}", part1(&ords, &lists));
    println!("Part 2 results: {}", part2(&ords, &lists));

    Ok(())
}

fn parse_input(input: &str) -> Result<(HashMap<(i32, i32), Ordering>, Vec<Vec<i32>>)> {
    let mut ords = HashMap::new();
    let mut lists = Vec::new();

    let mut lines = input.lines();

    let mut line = lines.next().ok_or(anyhow!("input is blank"))?;
    while line != "" {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            bail!("incorrectly formated line: `{}`", line);
        }

        let i: i32 = parts[0]
            .parse()
            .with_context(|| format!("incorrectly formatted line: `{}`", line))?;
        let j: i32 = parts[1]
            .parse()
            .with_context(|| format!("incorrectly formatted line: `{}`", line))?;

        ords.insert((i, j), Ordering::Less);
        ords.insert((j, i), Ordering::Greater);

        line = lines.next().ok_or(anyhow!("EOF before blank line"))?;
    }

    for line in lines {
        let list = line
            .split(',')
            .map(|i| i.parse().map_err(|_| anyhow!("failed to parse {}", i)))
            .collect::<Result<Vec<i32>>>()
            .with_context(|| format!("incorrectly formatted line: `{}`", line))?;
        lists.push(list);
    }

    Ok((ords, lists))
}

fn part1(ords: &HashMap<(i32, i32), Ordering>, lists: &Vec<Vec<i32>>) -> i32 {
    lists
        .iter()
        .filter(|&list| {
            let mut sorted_list = list.clone();
            sorted_list.sort_by(|&i, &j| {
                if let Some(ord) = ords.get(&(i, j)) {
                    *ord
                } else {
                    Ordering::Equal
                }
            });
            sorted_list == *list
        })
        .map(|list| list.get(list.len() / 2).unwrap_or(&0))
        .sum()
}

fn part2(ords: &HashMap<(i32, i32), Ordering>, lists: &Vec<Vec<i32>>) -> i32 {
    lists
        .iter()
        .filter_map(|list| {
            let mut sorted_list = list.clone();
            sorted_list.sort_by(|&i, &j| {
                if let Some(ord) = ords.get(&(i, j)) {
                    *ord
                } else {
                    Ordering::Equal
                }
            });
            if sorted_list != *list {
                Some(sorted_list)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|list| list.get(list.len() / 2).unwrap_or(&0))
        .sum()
}
