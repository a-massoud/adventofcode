use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return Ok(());
    }

    let input: Vec<String> = BufReader::new(File::open(&args[1])?)
        .lines()
        .collect::<Result<_, _>>()?;

    println!(
        "Part 1 result: {}",
        part1(&input.iter().map(<_>::as_ref).collect::<Vec<_>>())
    );
    println!(
        "Part 2 results: {}",
        part2(&input.iter().map(<_>::as_ref).collect::<Vec<_>>())
    );

    Ok(())
}

fn parse_line(line: &str) -> Result<(i64, i64, i64), ()> {
    let split_line: Vec<_> = line.split('x').collect();

    if split_line.len() != 3 {
        return Err(());
    }

    let (l, w, h) = (
        split_line[0].parse::<i64>(),
        split_line[1].parse::<i64>(),
        split_line[2].parse::<i64>(),
    );

    if l.is_err() || w.is_err() || h.is_err() {
        return Err(());
    }

    return Ok((l.unwrap(), w.unwrap(), h.unwrap()));
}

fn part1(input: &[&str]) -> i64 {
    let mut area = 0;

    'line_loop: for line in input {
        let (l, w, h) = match parse_line(line) {
            Ok(v) => v,
            Err(_) => continue 'line_loop,
        };

        let sides = [l * w, l * h, w * h];

        area += sides.iter().fold(0, |acc, side| acc + 2 * side)
            + match sides.iter().min() {
                Some(v) => v,
                None => return 0,
            };
    }

    area
}

fn part2(input: &[&str]) -> i64 {
    let mut len = 0;

    'line_loop: for line in input {
        let (l, w, h) = match parse_line(line) {
            Ok(v) => v,
            Err(_) => continue 'line_loop,
        };

        let perims = [2 * l + 2 * w, 2 * l + 2 * h, 2 * w + 2 * h];

        len += match perims.iter().min() {
            Some(v) => v,
            None => return 0,
        } + l * w * h;
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_sample_txt() -> Vec<String> {
        BufReader::new(File::open("sample.txt").unwrap())
            .lines()
            .collect::<Result<_, _>>()
            .unwrap()
    }

    #[test]
    fn test_parsing() {
        assert_eq!(parse_line("5x6x9").unwrap(), (5, 6, 9));
        assert_eq!(parse_line("127x53x1").unwrap(), (127, 53, 1));
        assert!(parse_line("").is_err());
        assert!(parse_line("5x12x27x591").is_err());
        assert!(parse_line("asxbfxd").is_err());
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                &read_sample_txt()
                    .iter()
                    .map(<_>::as_ref)
                    .collect::<Vec<_>>()
            ),
            101
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                &read_sample_txt()
                    .iter()
                    .map(<_>::as_ref)
                    .collect::<Vec<_>>()
            ),
            48
        )
    }
}
