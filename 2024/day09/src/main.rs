// Probably still far from optimal, but I like it

use anyhow::{anyhow, bail};
use std::collections::BTreeMap;
use std::{env, fs, iter};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0])
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<BTreeMap<usize, (usize, Option<u32>)>> {
    let tmp = input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(anyhow!("input contains non-digit"))
                .map(|x| x as u8)
        })
        .collect::<anyhow::Result<Vec<u8>>>()?;
    Ok(tmp
        .into_iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if n == 0 {
                None
            } else if i % 2 == 0 {
                Some((n as usize, Some((i / 2) as u32)))
            } else {
                Some((n as usize, None))
            }
        })
        .fold((0usize, BTreeMap::new()), |(nexti, mut map), i| {
            map.insert(nexti, i);
            (nexti + i.0, map)
        })
        .1)
}

fn part1(disk: &BTreeMap<usize, (usize, Option<u32>)>) -> usize {
    let mut disk: Vec<Option<u32>> = disk
        .iter()
        .flat_map(|(_, (n, v))| iter::repeat_n(*v, *n))
        .collect();
    let mut idx = 0;
    let mut edx = disk.len() - 1;

    while idx < edx && !disk[idx].is_none() {
        idx += 1;
    }
    while idx < edx {
        while disk[edx].is_none() && idx < edx {
            edx -= 1;
        }

        disk.swap(idx, edx);

        while idx < edx && !disk[idx].is_none() {
            idx += 1;
        }
    }

    disk.iter()
        .enumerate()
        .fold(0usize, |acc, (i, x)| acc + i * (x.unwrap_or(0) as usize))
}

fn part2(disk: &BTreeMap<usize, (usize, Option<u32>)>) -> usize {
    let mut disk = disk.clone();

    let mut edx = disk.iter().rev().find(|(_, (_, v))| v.is_some());
    while edx.is_some() && edx.unwrap().0 > &0 {
        let block = (*edx.unwrap().0, *edx.unwrap().1);

        let idx = disk
            .range(..block.0)
            .find(|(_, (n, v))| v.is_none() && n >= &block.1 .0);
        if let Some(idx) = idx {
            let (idx, nblock) = (*idx.0, *idx.1);
            disk.remove(&idx);
            if nblock.0 == block.1 .0 {
                disk.insert(idx, block.1);
            } else {
                disk.insert(idx, block.1);
                disk.insert(idx + block.1 .0, (nblock.0 - block.1 .0, None));
            }

            disk.remove(&block.0);
            let mut new_null_start = block.0;
            let mut new_null_len = block.1 .0;
            let before = disk.range(..block.0).rev().next();
            if let Some(before) = before {
                let before = (*before.0, *before.1);

                if before.1 .1.is_none() {
                    disk.remove(&before.0);
                    new_null_start = before.0;
                    new_null_len += before.1 .0;
                }
            }
            let after = disk.range(block.0..).next();
            if let Some(after) = after {
                let after = (*after.0, *after.1);

                if after.1 .1.is_none() {
                    disk.remove(&after.0);
                    new_null_len += after.1 .0;
                }
            }

            disk.insert(new_null_start, (new_null_len, None));
        }

        edx = disk.range(..block.0).rev().find(|(_, (_, v))| v.is_some());
    }

    disk.iter()
        .flat_map(|(_, (n, v))| iter::repeat_n(*v, *n))
        .enumerate()
        .fold(0usize, |acc, (i, x)| acc + i * (x.unwrap_or(0) as usize))
}
