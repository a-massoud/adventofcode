// These are honestly getting easier?

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result, anyhow, bail};

fn main() -> Result<()> {
    let input_path = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("No input provided"))?;
    let blacklist =
        read_input(&input_path).with_context(|| anyhow!("Reading input from `{}`", input_path))?;

    println!("===Part 1===");
    let min_ip = blacklist.first_allowed_ip();
    if let Some(min_ip) = min_ip {
        println!("Minimum IP: {}", min_ip);
    } else {
        println!("No IPs are allowed");
    }
    println!();

    println!("===Part 2===");
    let n_allowed = blacklist.count_allowed_ips();
    println!("Allowed IPs: {}", n_allowed);

    Ok(())
}

#[derive(Debug, Clone)]
struct Blacklist(Vec<(u32, u32)>);

impl Blacklist {
    fn from_ranges(mut ranges: Vec<(u32, u32)>) -> Result<Self> {
        for (a, b) in &ranges {
            if a > b {
                bail!("Bad range {}-{}", a, b);
            }
        }

        ranges.sort_unstable_by(|(a_start, a_end), (b_start, b_end)| {
            a_start.cmp(b_start).then(a_end.cmp(b_end))
        });

        let mut merged: Vec<(u32, u32)> = Vec::with_capacity(ranges.len());
        for (start, end) in ranges {
            if let Some(last) = merged.last_mut()
                && start <= last.1.saturating_add(1)
            {
                last.1 = last.1.max(end);
                continue;
            }
            merged.push((start, end));
        }

        Ok(Self(merged))
    }

    fn first_allowed_ip(&self) -> Option<u32> {
        match self.0.first() {
            None => Some(0),
            Some(&(start, _)) if start > 0 => Some(0),
            Some(&(_, end)) if end < u32::MAX => Some(end + 1),
            _ => None,
        }
    }

    fn count_allowed_ips(&self) -> u64 {
        if self.0.is_empty() {
            return u32::MAX as u64 + 1;
        }

        let mut count = self.0[0].0 as u64 + (u32::MAX - self.0[self.0.len() - 1].1) as u64;

        for [(_, lo), (hi, _)] in self.0.array_windows() {
            count += (hi - lo - 1) as u64;
        }

        count
    }
}

fn parse_input(input: impl BufRead) -> Result<Blacklist> {
    let ranges: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(no, line)| {
            let no = no + 1;
            let line = line.with_context(|| format!("Reading line {}", no))?;

            let (start, end) = line
                .split_once('-')
                .ok_or_else(|| anyhow!("Line {} does not contain '-'", no))?;

            let start = start.trim();
            let end = end.trim();

            let start: u32 = start
                .parse()
                .with_context(|| format!("Parsing `{}` in line {}", start, no))?;
            let end: u32 = end
                .parse()
                .with_context(|| format!("Parsing `{}` in line {}", end, no))?;

            Ok((start, end))
        })
        .collect::<Result<_>>()?;

    Blacklist::from_ranges(ranges)
}

fn read_input(path: impl AsRef<Path>) -> Result<Blacklist> {
    let path = path.as_ref();
    let input = BufReader::new(
        File::open(path).with_context(|| format!("Opening file `{}`", path.display()))?,
    );
    parse_input(input).with_context(|| format!("Parsing file `{}`", path.display()))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::parse_input;

    const SAMPLE_INPUT: &str = "\
5-8
0-2
4-7";

    #[test]
    fn sample_input() {
        let blacklist = parse_input(Cursor::new(SAMPLE_INPUT)).expect("Sample input parses");
        assert_eq!(blacklist.0, vec![(0, 2), (4, 8)]);
    }
}
