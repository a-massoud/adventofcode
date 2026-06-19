// Very simple, nice and easy.

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result, anyhow, bail};
use num::Integer;
use regex::Regex;

fn main() -> Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("no argument provided"))?;
    let input_reader = BufReader::new(
        File::open(&input_path).with_context(|| format!("opening file `{}`", input_path))?,
    );
    let discs = parse_input(input_reader).context("reading input")?;

    let drop_time = get_drop_time(&discs).ok_or(anyhow!("no drop time found"))?;

    println!("===Part 1===");
    println!("Drop time: {}", drop_time);
    println!();

    let mut discs = discs;
    discs.reserve_exact(1);
    let new_pos_count = 11;
    let new_initial_pos = 0;
    let new_disc_no = discs.len() as u64 + 1;
    discs.push(Disc {
        pos_count: new_pos_count,
        initial_pos: new_initial_pos,
        target: (new_pos_count - (new_disc_no % new_pos_count)) % new_pos_count,
    });
    let discs = discs;

    let drop_time = get_drop_time(&discs).ok_or(anyhow!("no drop time found"))?;

    println!("===Part 2===");
    println!("Drop time: {}", drop_time);

    Ok(())
}

#[derive(Debug, Default, Clone, Copy)]
struct Disc {
    pos_count: u64,
    initial_pos: u64,
    target: u64,
}

fn parse_input(input: impl BufRead) -> Result<Vec<Disc>> {
    let disc_pattern = Regex::new(
        "^Disc #([0-9]+) has ([0-9]+) positions; at time=0, it is at position ([0-9]+)\\.$",
    )
    .expect("regex failed to compile");

    input
        .lines()
        .enumerate()
        .map(|(number, line)| {
            let line = line.context("reading line")?;

            let m = disc_pattern
                .captures(&line)
                .ok_or_else(|| anyhow!("line `{}` has invalid format", line))?;
            let (_, [disc, pos_count, initial_pos]) = m.extract();

            let disc_no: u64 = disc.parse().expect("matched regex");
            let pos_count = pos_count.parse().expect("matched regex");
            let initial_pos = initial_pos.parse().expect("matched regex");

            if initial_pos >= pos_count || disc_no as usize != number + 1 {
                bail!("disc {} is invalid", disc_no);
            }

            let target = (pos_count - (disc_no % pos_count)) % pos_count;

            Ok(Disc {
                pos_count,
                initial_pos,
                target,
            })
        })
        .collect()
}

fn get_drop_time(discs: &[Disc]) -> Option<u64> {
    let Some(max_disc) = discs.iter().max_by_key(|disc| disc.pos_count) else {
        return Some(0);
    };
    let step = max_disc.pos_count;
    let initial =
        ((max_disc.target + max_disc.pos_count) - max_disc.initial_pos) % max_disc.pos_count;

    let max = discs
        .iter()
        .map(|disc| disc.pos_count)
        .fold(1u64, |acc, count| acc.lcm(&count));

    (initial..=max).step_by(step as usize).find(|&time| {
        discs
            .iter()
            .all(|disc| (disc.initial_pos + time) % disc.pos_count == disc.target)
    })
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{get_drop_time, parse_input};

    const SAMPLE_INPUT: &str = "\
        Disc #1 has 5 positions; at time=0, it is at position 4.\n\
        Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn sample_input() {
        let discs = parse_input(Cursor::new(SAMPLE_INPUT)).expect("failed to read input");
        assert_eq!(get_drop_time(&discs), Some(5))
    }
}
