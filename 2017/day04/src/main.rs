use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        bail!("No argument provided");
    }

    let passphrases = read_input(&args[1]).context("Failed to read input")?;

    println!("Part 1");
    println!("======");
    println!(
        "Valid passphrases: {}",
        passphrases.iter().filter(|pass| pass.is_valid_p1()).count()
    );
    println!();

    println!("Part 2");
    println!("======");
    println!(
        "Valid passphrases: {}",
        passphrases.iter().filter(|pass| pass.is_valid_p2()).count()
    );

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct Passphrase(Vec<String>);

impl Passphrase {
    fn is_valid_p1(&self) -> bool {
        let mut set = HashSet::new();
        for s in &self.0 {
            if !set.insert(s) {
                return false;
            }
        }

        true
    }

    fn is_valid_p2(&self) -> bool {
        let mut set = HashSet::new();
        for s in &self.0 {
            let mut b: Vec<_> = s.bytes().collect();
            b.sort_unstable();
            if !set.insert(b) {
                return false;
            }
        }

        true
    }
}

fn parse_input(input: impl BufRead) -> Result<Vec<Passphrase>> {
    input
        .lines()
        .enumerate()
        .map(|(no, line)| {
            let no = no + 1;
            let line = line.with_context(|| format!("Failed to read line {}", no))?;

            Ok(Passphrase(
                line.split_whitespace().map(String::from).collect(),
            ))
        })
        .collect()
}

fn read_input(path: impl AsRef<Path>) -> Result<Vec<Passphrase>> {
    let path = path.as_ref();
    let input = BufReader::new(
        File::open(path).with_context(|| format!("Failed to open file `{}`", path.display()))?,
    );
    parse_input(input).with_context(|| format!("Failed to parse file `{}`", path.display()))
}
