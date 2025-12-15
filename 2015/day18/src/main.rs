use anyhow::{anyhow, bail};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    let input = read_input(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;

    let p1 = state_after_n(&input, 100, false)
        .iter()
        .flatten()
        .filter(|&&i| i)
        .count();
    println!("Part 1: {}", p1);

    let p2 = state_after_n(&input, 100, true)
        .iter()
        .flatten()
        .filter(|&&i| i)
        .count();
    println!("Part 2: {}", p2);

    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<Vec<bool>>> {
    let reader = BufReader::new(File::open(path)?);

    reader
        .lines()
        .map(|line| {
            line?
                .trim()
                .bytes()
                .map(|ch| match ch {
                    b'.' => Ok(false),
                    b'#' => Ok(true),
                    _ => bail!("bad character `{}`", ch as char),
                })
                .collect()
        })
        .collect()
}

fn get_n_neighbors(i: usize, j: usize, state: &[impl AsRef<[bool]>]) -> u8 {
    let i = i as isize;
    let j = j as isize;
    let mut r = 0;

    for y in (i - 1)..=(i + 1) {
        for x in (j - 1)..=(j + 1) {
            if y >= 0
                && (y as usize) < state.len()
                && x >= 0
                && (x as usize) < state[y as usize].as_ref().len()
                && !(y == i && x == j)
                && state[y as usize].as_ref()[x as usize]
            {
                r += 1;
            }
        }
    }

    r
}

fn step_gol(state: &[impl AsRef<[bool]>], fix_corners: bool) -> Vec<Vec<bool>> {
    state
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.as_ref()
                .iter()
                .enumerate()
                .map(|(j, &v)| {
                    let n = get_n_neighbors(i, j, state);
                    (fix_corners
                        && (i == 0 || i == state.len() - 1)
                        && (j == 0 || j == state[i].as_ref().len() - 1))
                        || (!v && n == 3)
                        || (v && (n == 2 || n == 3))
                })
                .collect()
        })
        .collect()
}

fn state_after_n(initial: &[impl AsRef<[bool]>], n: usize, fix_corners: bool) -> Vec<Vec<bool>> {
    let mut state: Vec<Vec<_>> = initial.iter().map(|i| i.as_ref().to_vec()).collect();
    if fix_corners && state.len() > 0 {
        if state[0].len() > 0 {
            let rowlen = state[0].len();
            state[0][0] = true;
            state[0][rowlen - 1] = true;
        }
        if state[state.len() - 1].len() > 0 {
            let statelen = state.len();
            let rowlen = state[statelen - 1].len();
            state[statelen - 1][0] = true;
            state[statelen - 1][rowlen - 1] = true;
        }
    }

    for _i in 0..n {
        state = step_gol(&state, fix_corners);
    }

    state
}
