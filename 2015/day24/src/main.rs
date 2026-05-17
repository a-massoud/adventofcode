use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;
use thiserror::Error;

fn main() {
    let input = match env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("no argument provided");
            return;
        }
    };

    let input = match parse_input(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse input: {}", e);
            return;
        }
    };

    let part1 = match min_size_and_qe(&input, 3) {
        Some(v) => v,
        None => {
            eprintln!("no part 1 solution found");
            return;
        }
    };
    println!("Part 1: {}", part1);

    let part2 = match min_size_and_qe(&input, 4) {
        Some(v) => v,
        None => {
            eprintln!("no part 2 solution found");
            return;
        }
    };
    println!("Part 2: {}", part2);
}

fn parse_input(input_file: impl AsRef<Path>) -> Result<Vec<i64>, ParseInputError> {
    let file = BufReader::new(File::open(input_file).map_err(ParseInputError::FileOpen)?);

    file.lines()
        .map(|line| {
            line.map_err(ParseInputError::ReadLine).and_then(|line| {
                line.parse()
                    .map_err(|_| ParseInputError::Parse(line.to_owned()))
            })
        })
        .collect()
}

#[derive(Debug, Error)]
enum ParseInputError {
    #[error("failed to open file: {0}")]
    FileOpen(io::Error),
    #[error("failed to read line: {0}")]
    ReadLine(io::Error),
    #[error("failed to parse line `{0}`")]
    Parse(String),
}

fn min_size_and_qe(input: &[i64], n_groups: usize) -> Option<i64> {
    let total_mass: i64 = input.iter().sum();
    if total_mass % n_groups as i64 != 0 {
        return None;
    }
    let group_mass = total_mass / n_groups as i64;

    (1..(input.len() / 3 + 1))
        .flat_map(|i| input.iter().combinations(i))
        .filter(|i| i.iter().copied().sum::<i64>() == group_mass)
        .fold((usize::MAX, Vec::new()), |(mut n, mut acc), group| {
            if group.len() < n {
                acc.clear();
                n = group.len();
            }

            if group.len() == n {
                acc.push(group);
            }

            (n, acc)
        })
        .1
        .iter()
        .map(|group| group.iter().copied().product())
        .min()
}
