// Ok, this is the first one this year that is still slow in release mode. Not too proud of this.

use anyhow::{anyhow, bail};
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

fn parse_input(input: &str) -> anyhow::Result<Vec<Option<usize>>> {
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
        .flat_map(|(i, n)| {
            if i % 2 == 0 {
                iter::repeat_n(Some(i / 2), n as usize)
            } else {
                iter::repeat_n(None, n as usize)
            }
        })
        .collect())
}

fn part1(disk: &Vec<Option<usize>>) -> usize {
    let mut disk = disk.clone();
    let mut idx = 0;
    let mut edx = disk.len() - 1;

    while idx <= edx && !disk[idx].is_none() {
        idx += 1;
    }
    while idx <= edx {
        while disk[edx].is_none() && idx <= edx {
            edx -= 1;
        }

        disk.swap(idx, edx);

        while idx <= edx && !disk[idx].is_none() {
            idx += 1;
        }
    }

    disk.iter()
        .enumerate()
        .fold(0usize, |acc, (i, x)| acc + i * x.unwrap_or(0))
}

fn part2(disk: &Vec<Option<usize>>) -> usize {
    let mut disk = disk.clone();
    let mut edx = disk.len() - 1;

    while 0 < edx {
        while disk[edx].is_none() && 0 < edx {
            edx -= 1;
        }

        let fend = edx;
        let mut fstart = fend;
        while disk[fstart] == disk[fend] && fstart > 0 {
            fstart -= 1;
        }
        let fstart = fstart + 1;
        let flen = fend - fstart + 1;

        let mut ostart = 0;
        while !disk.iter().skip(ostart).take(flen).all(|x| x.is_none()) && ostart < fstart {
            ostart += 1;
        }

        if ostart < fstart {
            for i in 0..flen {
                disk.swap(ostart + i, fstart + i);
            }
        }
        if 0 < fstart {
            edx = fstart - 1;
        } else {
            break;
        }
    }

    disk.iter()
        .enumerate()
        .fold(0usize, |acc, (i, x)| acc + i * x.unwrap_or(0))
}
