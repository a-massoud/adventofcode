// Nice and simple again

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result, anyhow, bail};

fn main() -> Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("no input provided"))?;
    let first_row = read_input(input_path).context("parsing input")?;

    println!("===Part 1===");
    let count = count_safe_tiles(&first_row, 40);
    println!("Number of safe tiles: {}", count);
    println!();

    println!("===Part 2===");
    let count = count_safe_tiles(&first_row, 400_000);
    println!("Number of safe tiles: {}", count);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}

fn parse_input(input: impl BufRead) -> Result<Vec<Tile>> {
    let mut lines = input.lines();

    let row = lines
        .next()
        .ok_or(anyhow!("no first line"))?
        .context("reading first line")?;
    if lines.next().is_some() {
        bail!("input had more than one line");
    }

    row.into_bytes()
        .into_iter()
        .map(|v| match v {
            b'.' => Ok(Tile::Safe),
            b'^' => Ok(Tile::Trap),
            _ => Err(anyhow!("bad character: {}", v as char)),
        })
        .collect()
}

fn read_input(path: impl AsRef<Path>) -> Result<Vec<Tile>> {
    let input = BufReader::new(
        File::open(&path)
            .with_context(|| format!("opening `{}` for reading", path.as_ref().display()))?,
    );
    parse_input(input).with_context(|| format!("reading from `{}`", path.as_ref().display()))
}

fn generate_next_row(current_row: &[Tile]) -> Vec<Tile> {
    current_row
        .iter()
        .enumerate()
        .map(|(i, &center)| {
            let left = i
                .checked_sub(1)
                .and_then(|j| current_row.get(j).copied())
                .unwrap_or(Tile::Safe);
            let right = current_row.get(i + 1).copied().unwrap_or(Tile::Safe);

            match (left, center, right) {
                (Tile::Trap, Tile::Trap, Tile::Safe)
                | (Tile::Safe, Tile::Trap, Tile::Trap)
                | (Tile::Trap, Tile::Safe, Tile::Safe)
                | (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
                _ => Tile::Safe,
            }
        })
        .collect()
}

fn count_safe_tiles(first_row: &[Tile], n_rows: usize) -> usize {
    if n_rows == 0 {
        return 0;
    }

    let mut current_row = Vec::from(first_row);
    let mut count = 0;

    for _ in 0..(n_rows - 1) {
        count += current_row.iter().filter(|&&v| v == Tile::Safe).count();
        current_row = generate_next_row(&current_row);
    }
    count += current_row.iter().filter(|&&v| v == Tile::Safe).count();

    count
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{count_safe_tiles, parse_input};

    #[test]
    fn short_sample() {
        let first_row = parse_input(Cursor::new("..^^.")).expect("sample input should work");
        assert_eq!(count_safe_tiles(&first_row, 3), 6);
    }

    #[test]
    fn long_sample() {
        let first_row = parse_input(Cursor::new(".^^.^.^^^^")).expect("sample input should work");
        assert_eq!(count_safe_tiles(&first_row, 10), 38);
    }
}
