// Definitely not the best solution

use anyhow::{anyhow, bail, Result};
use std::{env, fs};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        bail!("Usage: {} <input>", args[0]);
    }

    let input = fs::read_to_string(&args[1])?;

    println!("Part 1 result: {}", part1(&input)?);
    println!("Part 2 results: {}", part2(&input));

    Ok(())
}

fn find_horizontal(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        acc + line
            .as_bytes()
            .windows(4)
            .filter(|&x| x == "XMAS".as_bytes())
            .count()
    })
}

fn find_horizontal_rev(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        acc + line
            .as_bytes()
            .windows(4)
            .filter(|&x| x == "SAMX".as_bytes())
            .count()
    })
}

fn find_vertical(input: &str) -> usize {
    let mut cols = Vec::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if i == cols.len() {
                cols.push(String::from(c));
            } else {
                cols[i].push(c);
            }
        }
    }

    let transposed = cols
        .iter()
        .fold(String::new(), |acc, line| format!("{}\n{}", acc, line));

    find_horizontal(&transposed)
}

fn find_vertical_rev(input: &str) -> usize {
    let mut cols = Vec::new();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if i == cols.len() {
                cols.push(String::from(c));
            } else {
                cols[i].push(c);
            }
        }
    }

    let transposed = cols
        .iter()
        .fold(String::new(), |acc, line| format!("{}\n{}", acc, line));

    find_horizontal_rev(&transposed)
}

fn find_right_diagonals(input: &str) -> Result<usize> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    for i in 0..(lines.len() - 3) {
        for j in 0..(lines[i].len() - 3) {
            if *lines
                .get(i)
                .ok_or(anyhow!("0-i index out of range"))?
                .get(j)
                .ok_or(anyhow!("0-j index out of range"))?
                == 'X'
                && *lines
                    .get(i + 1)
                    .ok_or(anyhow!("1-i index out of range"))?
                    .get(j + 1)
                    .ok_or(anyhow!("1-j index out of range"))?
                    == 'M'
                && *lines
                    .get(i + 2)
                    .ok_or(anyhow!("2-i index out of range"))?
                    .get(j + 2)
                    .ok_or(anyhow!("2-j index out of range"))?
                    == 'A'
                && *lines
                    .get(i + 3)
                    .ok_or(anyhow!("3-i index out of range"))?
                    .get(j + 3)
                    .ok_or(anyhow!("3-j index out of range"))?
                    == 'S'
            {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn find_right_diagonals_rev(input: &str) -> Result<usize> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    for i in 0..(lines.len() - 3) {
        for j in 0..(lines[i].len() - 3) {
            if *lines
                .get(i)
                .ok_or(anyhow!("0-i index out of range"))?
                .get(j)
                .ok_or(anyhow!("0-j index out of range"))?
                == 'S'
                && *lines
                    .get(i + 1)
                    .ok_or(anyhow!("1-i index out of range"))?
                    .get(j + 1)
                    .ok_or(anyhow!("1-j index out of range"))?
                    == 'A'
                && *lines
                    .get(i + 2)
                    .ok_or(anyhow!("2-i index out of range"))?
                    .get(j + 2)
                    .ok_or(anyhow!("2-j index out of range"))?
                    == 'M'
                && *lines
                    .get(i + 3)
                    .ok_or(anyhow!("3-i index out of range"))?
                    .get(j + 3)
                    .ok_or(anyhow!("3-j index out of range"))?
                    == 'X'
            {
                total += 1;
            }
        }
    }

    Ok(total)
}
fn find_left_diagonals(input: &str) -> Result<usize> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    for i in 3..lines.len() {
        for j in 0..(lines[i].len() - 3) {
            if *lines
                .get(i)
                .ok_or(anyhow!("0-i index out of range"))?
                .get(j)
                .ok_or(anyhow!("0-j index out of range"))?
                == 'X'
                && *lines
                    .get(i - 1)
                    .ok_or(anyhow!("-1-i index out of range"))?
                    .get(j + 1)
                    .ok_or(anyhow!("1-j index out of range"))?
                    == 'M'
                && *lines
                    .get(i - 2)
                    .ok_or(anyhow!("-2-i index out of range"))?
                    .get(j + 2)
                    .ok_or(anyhow!("2-j index out of range"))?
                    == 'A'
                && *lines
                    .get(i - 3)
                    .ok_or(anyhow!("-3-i index out of range"))?
                    .get(j + 3)
                    .ok_or(anyhow!("3-j index out of range"))?
                    == 'S'
            {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn find_left_diagonals_rev(input: &str) -> Result<usize> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    for i in 3..lines.len() {
        for j in 0..(lines[i].len() - 3) {
            if *lines
                .get(i)
                .ok_or(anyhow!("0-i index out of range"))?
                .get(j)
                .ok_or(anyhow!("0-j index out of range"))?
                == 'S'
                && *lines
                    .get(i - 1)
                    .ok_or(anyhow!("-1-i index out of range"))?
                    .get(j + 1)
                    .ok_or(anyhow!("1-j index out of range"))?
                    == 'A'
                && *lines
                    .get(i - 2)
                    .ok_or(anyhow!("-2-i index out of range"))?
                    .get(j + 2)
                    .ok_or(anyhow!("2-j index out of range"))?
                    == 'M'
                && *lines
                    .get(i - 3)
                    .ok_or(anyhow!("-3-i index out of range"))?
                    .get(j + 3)
                    .ok_or(anyhow!("3-j index out of range"))?
                    == 'X'
            {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn part1(input: &str) -> Result<usize> {
    let horizontal = find_horizontal(input);
    let horizontal_rev = find_horizontal_rev(input);
    let vertical = find_vertical(input);
    let vertical_rev = find_vertical_rev(input);
    let right_diagonals = find_right_diagonals(input)?;
    let right_diagonals_rev = find_right_diagonals_rev(input)?;
    let left_diagonals = find_left_diagonals(input)?;
    let left_diagonals_rev = find_left_diagonals_rev(input)?;

    Ok(horizontal
        + horizontal_rev
        + vertical
        + vertical_rev
        + right_diagonals
        + right_diagonals_rev
        + left_diagonals
        + left_diagonals_rev)
}

fn is_x_mas(input: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i == 0
        || j == 0
        || input.len() <= i + 1
        || input[i - 1].len() <= j + 1
        || input[i + 1].len() <= j + 1
        || input[i][j] != 'A'
    {
        return false;
    }

    (input[i - 1][j - 1] == 'M'
        && input[i - 1][j + 1] == 'M'
        && input[i + 1][j - 1] == 'S'
        && input[i + 1][j + 1] == 'S')
        || (input[i - 1][j - 1] == 'M'
            && input[i + 1][j - 1] == 'M'
            && input[i - 1][j + 1] == 'S'
            && input[i + 1][j + 1] == 'S')
        || (input[i + 1][j - 1] == 'M'
            && input[i + 1][j + 1] == 'M'
            && input[i - 1][j - 1] == 'S'
            && input[i - 1][j + 1] == 'S')
        || (input[i - 1][j + 1] == 'M'
            && input[i + 1][j + 1] == 'M'
            && input[i - 1][j - 1] == 'S'
            && input[i + 1][j - 1] == 'S')
}

fn part2(input: &str) -> usize {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut n = 0;

    for (i,line) in input.iter().enumerate() {
        for j in 0..line.len() {
            n += if is_x_mas(&input, i, j) { 1 } else { 0 }
        }
    }

    n
}
