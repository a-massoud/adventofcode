use std::{
    env,
    fs::{self},
    path::Path,
};

use eyre::{Context, Result, bail, eyre};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        bail!("No input file provided");
    }

    let captcha = read_input(&args[1]).context("Failed to read input")?;

    let checksum = captcha.calculate_linear_checksum();
    println!(" Part 1 ");
    println!("========");
    println!("Checksum: {}", checksum);
    println!();

    let checksum = captcha.calculate_circular_checksum();
    println!(" Part 2 ");
    println!("========");
    println!("Checksum: {}", checksum);
    println!();

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct Captcha(Vec<u8>);

impl Captcha {
    pub fn calculate_linear_checksum(&self) -> u64 {
        self.0
            .iter()
            .zip(self.0.iter().cycle().skip(1))
            .filter_map(|(a, b)| if a == b { Some(*a as u64) } else { None })
            .sum()
    }

    pub fn calculate_circular_checksum(&self) -> u64 {
        self.0
            .iter()
            .zip(self.0.iter().cycle().skip(self.0.len() / 2))
            .filter_map(|(a, b)| if a == b { Some(*a as u64) } else { None })
            .sum()
    }
}

fn parse_input(input: &str) -> Result<Captcha> {
    let input = input.trim();

    let v: Vec<_> = input
        .bytes()
        .map(|b| match b {
            b'0'..=b'9' => Ok(b - b'0'),
            _ => Err(eyre!("Unexpected character `{}`", b as char)),
        })
        .collect::<Result<_>>()?;

    Ok(Captcha(v))
}

fn read_input(path: impl AsRef<Path>) -> Result<Captcha> {
    let path = path.as_ref();
    let input = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file `{}`", path.display()))?;
    parse_input(&input).with_context(|| format!("Failed to parse file `{}`", path.display()))
}

#[cfg(test)]
mod test {
    use crate::parse_input;

    #[test]
    fn linear_checksum() {
        let input = parse_input("1122").expect("Sample input parses");
        assert_eq!(input.calculate_linear_checksum(), 3);
        let input = parse_input("1111").expect("Sample input parses");
        assert_eq!(input.calculate_linear_checksum(), 4);
        let input = parse_input("1234").expect("Sample input parses");
        assert_eq!(input.calculate_linear_checksum(), 0);
        let input = parse_input("91212129").expect("Sample input parses");
        assert_eq!(input.calculate_linear_checksum(), 9);
    }

    #[test]
    fn circular_checksum() {
        let input = parse_input("1212").expect("Sample input parses");
        assert_eq!(input.calculate_circular_checksum(), 6);
        let input = parse_input("1221").expect("Sample input parses");
        assert_eq!(input.calculate_circular_checksum(), 0);
        let input = parse_input("123425").expect("Sample input parses");
        assert_eq!(input.calculate_circular_checksum(), 4);
        let input = parse_input("123123").expect("Sample input parses");
        assert_eq!(input.calculate_circular_checksum(), 12);
        let input = parse_input("12131415").expect("Sample input parses");
        assert_eq!(input.calculate_circular_checksum(), 4);
    }
}
