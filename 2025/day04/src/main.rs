// Suboptimal, but not bad at all.

use anyhow::{anyhow, bail};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("needs argument"))?)?;
    let input = parse_input(&input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Paper,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<Tile>>> {
    let r: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| match ch {
                    '@' => Ok(Tile::Paper),
                    '.' => Ok(Tile::Empty),
                    _ => Err(anyhow!("bad character: {}", ch)),
                })
                .collect::<anyhow::Result<_>>()
        })
        .collect::<anyhow::Result<_>>()?;

    if let Some(len) = r.get(0).map(|x| x.len()) {
        if let Some((line, _)) = r.iter().enumerate().find(|(_, x)| x.len() != len) {
            bail!("line {} has bad length", line);
        }
    }

    Ok(r)
}

fn get_n_neighbors(i: isize, j: isize, a: &[impl AsRef<[Tile]>]) -> u32 {
    let mut r = 0;

    for k in (i - 1)..=(i + 1) {
        for l in (j - 1)..=(j + 1) {
            if !(k == i && l == j)
                && 0 <= k
                && k < a.len() as isize
                && 0 <= l
                && l < a[k as usize].as_ref().len() as isize
                && a[k as usize].as_ref()[l as usize] == Tile::Paper
            {
                r += 1;
            }
        }
    }

    r
}

fn part1(input: &[impl AsRef<[Tile]>]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.as_ref()
                .iter()
                .enumerate()
                .filter(|(j, tile)| {
                    **tile == Tile::Paper && get_n_neighbors(i as isize, *j as isize, &input) < 4
                })
                .count()
        })
        .sum()
}

fn part2(input: &[impl AsRef<[Tile]>]) -> usize {
    let mut r = 0;
    let mut map: Vec<Vec<Tile>> = input.iter().map(|line| line.as_ref().to_vec()).collect();

    loop {
        let to_remove: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                let map = &map;
                line.iter().enumerate().flat_map(move |(j, tile)| {
                    if *tile == Tile::Paper && get_n_neighbors(i as isize, j as isize, map) < 4 {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect();

        if to_remove.is_empty() {
            break;
        }

        r += to_remove.len();
        for (i, j) in to_remove {
            map[i][j] = Tile::Empty;
        }
    }

    r
}
