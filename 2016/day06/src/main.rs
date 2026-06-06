// Very easy

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::Path,
};

use anyhow::{Context, anyhow, bail};

fn main() -> anyhow::Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("no argument provided"))?;

    let input = read_input(input_path).context("failed to read input")?;

    let most_common_letters = get_most_common_letters(&input);
    println!("Part 1: {}", str::from_utf8(&most_common_letters)?);

    let least_common_letters = get_least_common_letters(&input);
    println!("Part 2: {}", str::from_utf8(&least_common_letters)?);

    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<Vec<u8>>> {
    let input = BufReader::new(
        File::open(&path)
            .with_context(|| format!("failed to open `{:?}` for reading", path.as_ref()))?,
    );
    let mut lines = input.lines();

    let first_line = lines.next().ok_or(anyhow!("no first line to read"))??;
    let mut v: Vec<Vec<u8>> = iter::repeat_n(Vec::new(), first_line.len()).collect();

    for line in iter::once(Ok(first_line)).chain(lines) {
        let line = line?;

        if line.len() != v.len() {
            bail!(
                "line `{}` did not match previous lines' length {}",
                line,
                v.len()
            );
        }

        for (ch, col) in line.bytes().zip(v.iter_mut()) {
            col.push(ch);
        }
    }

    Ok(v)
}

fn get_most_common_letters(input: &[impl AsRef<[u8]>]) -> Vec<u8> {
    let mut counts: Vec<_> = vec![[0usize; u8::MAX as usize + 1]; input.len()];

    for (col, count) in input.iter().zip(counts.iter_mut()) {
        for ch in col.as_ref() {
            count[*ch as usize] += 1;
        }
    }

    counts
        .iter()
        .map(|count| {
            count
                .iter()
                .enumerate()
                .max_by_key(|(_, v)| **v)
                .map(|(i, _)| i as u8)
                .unwrap() // iterator must be nonempty
        })
        .collect()
}

fn get_least_common_letters(input: &[impl AsRef<[u8]>]) -> Vec<u8> {
    let mut counts: Vec<_> = vec![[0usize; u8::MAX as usize + 1]; input.len()];

    for (col, count) in input.iter().zip(counts.iter_mut()) {
        for ch in col.as_ref() {
            count[*ch as usize] += 1;
        }
    }

    counts
        .iter()
        .map(|count| {
            count
                .iter()
                .enumerate()
                .filter(|(_, v)| **v > 0)
                .min_by_key(|(_, v)| **v)
                .map(|(i, _)| i as u8)
                .unwrap() // iterator must be nonempty
        })
        .collect()
}
