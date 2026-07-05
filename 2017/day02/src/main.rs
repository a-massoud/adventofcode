use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use eyre::{Context, Result, bail};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        bail!("No input provided");
    }

    let sheet = read_input(&args[1]).context("Failed to read input")?;

    let checksum = sheet.checksum();
    println!("Part 1");
    println!("======");
    println!("Checksum: {}", checksum);
    println!();

    let divisions = sheet.sum_even_divisions();
    println!("Part 2");
    println!("======");
    println!("Calculation result: {}", divisions);

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct Spreadsheet(Vec<Vec<i64>>);

impl Spreadsheet {
    fn checksum(&self) -> i64 {
        self.0
            .iter()
            .filter_map(|row| {
                let (min, max) = row.iter().fold((None, None), |(min, max), &v| {
                    (
                        min.map_or(Some(v), |min: i64| Some(min.min(v))),
                        max.map_or(Some(v), |max: i64| Some(max.max(v))),
                    )
                });
                let min = min?;
                let max = max?;
                Some(max - min)
            })
            .sum()
    }

    fn sum_even_divisions(&self) -> i64 {
        self.0
            .iter()
            .filter_map(|row| {
                row.iter()
                    .flat_map(|a| row.iter().map(move |b| (a, b)))
                    .find(|&(&a, &b)| a != b && a != 0 && b != 0 && a % b == 0)
            })
            .map(|(a, b)| a / b)
            .sum()
    }
}

fn parse_input(input: impl BufRead) -> Result<Spreadsheet> {
    Ok(Spreadsheet(
        input
            .lines()
            .enumerate()
            .map(|(no, line)| {
                let no = no + 1;
                let line = line.with_context(|| format!("Failed to read line {}", no))?;

                line.split_whitespace()
                    .map(|v| {
                        v.parse()
                            .with_context(|| format!("Failed to parse `{}` on line {}", v, no))
                    })
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?,
    ))
}

fn read_input(path: impl AsRef<Path>) -> Result<Spreadsheet> {
    let path = path.as_ref();
    let input = BufReader::new(
        File::open(path).with_context(|| format!("Failed to open file `{}`", path.display()))?,
    );
    parse_input(input).with_context(|| format!("Failed to parse file `{}`", path.display()))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::parse_input;

    const SAMPLE_INPUT_P1: &str = "\
5 1 9 5
7 5 3
2 4 6 8";

    #[test]
    fn sample_input_p1() {
        let sheet = parse_input(Cursor::new(SAMPLE_INPUT_P1)).expect("Sample input parses");
        assert_eq!(sheet.checksum(), 18);
    }

    const SAMPLE_INPUT_P2: &str = "\
5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn sample_input_p2() {
        let sheet = parse_input(Cursor::new(SAMPLE_INPUT_P2)).expect("Sample input parses");
        assert_eq!(sheet.sum_even_divisions(), 9);
    }
}
