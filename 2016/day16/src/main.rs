// There is absolutely a way to do this without calculating the full data, but it's only 35MB so I
// don't really care.

use std::env;

use anyhow::{Context, Result, anyhow};

fn main() -> Result<()> {
    let input = env::args().nth(1).ok_or(anyhow!("no input provided"))?;
    let seed = parse_input(&input).context("parsing input")?;

    println!("===Part 1===");
    let checksum = String::from_iter(
        calculate_checksum(&calculate_data(272, &seed))
            .into_iter()
            .map(|v| if v { '1' } else { '0' }),
    );
    println!("Checksum: {}", checksum);
    println!();

    println!("===Part 2===");
    let checksum = String::from_iter(
        calculate_checksum(&calculate_data(35_651_584, &seed))
            .into_iter()
            .map(|v| if v { '1' } else { '0' }),
    );
    println!("Checksum: {}", checksum);

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<bool>> {
    input
        .bytes()
        .map(|v| match v {
            b'1' => Ok(true),
            b'0' => Ok(false),
            _ => Err(anyhow!("invalid character: {}", v as char)),
        })
        .collect()
}

fn calculate_data(n: usize, seed: &[bool]) -> Vec<bool> {
    let mut full = Vec::from(seed);

    while full.len() < n {
        let old_len = full.len();
        full.reserve_exact(old_len + 1);
        full.push(false);
        for i in (0..old_len).rev() {
            full.push(!full[i]);
        }
    }

    full.truncate(n);

    full
}

fn reduce_checksum(seed: &[bool]) -> Vec<bool> {
    if !seed.len().is_multiple_of(2) {
        return Vec::from(seed);
    }

    seed.chunks(2).map(|chunk| chunk[0] == chunk[1]).collect()
}

fn calculate_checksum(input: &[bool]) -> Vec<bool> {
    let mut checksum = reduce_checksum(input);

    while checksum.len().is_multiple_of(2) {
        checksum = reduce_checksum(&checksum);
    }

    checksum
}

#[cfg(test)]
mod test {
    use crate::{calculate_checksum, calculate_data, parse_input};

    #[test]
    fn sample_input() {
        let seed = parse_input("10000").expect("sample input failed to parse");
        let data = calculate_data(20, &seed);
        assert_eq!(
            data,
            parse_input("10000011110010000111").expect("comparison should parse")
        );
        let checksum = calculate_checksum(&data);
        assert_eq!(
            checksum,
            parse_input("01100").expect("comparison should parse")
        );
    }
}
