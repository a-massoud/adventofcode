use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use ndarray::Array;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return Ok(());
    }

    let lines: Vec<String> = BufReader::new(File::open(&args[1])?)
        .lines()
        .collect::<Result<_, _>>()?;

    println!("Part 1 result: {}", run_part1(&lines));
    println!("Part 2 result: {}", run_part2(&lines));

    Ok(())
}

fn run_part1(lines: &Vec<String>) -> usize {
    let mut lights = Array::from_shape_simple_fn((1000, 1000), || false);

    for line in lines {
        let command = match parse_line(line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        match command {
            Command::On(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights[[i, j]] = true;
                    }
                }
            }
            Command::Off(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights[[i, j]] = false;
                    }
                }
            }
            Command::Toggle(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights[[i, j]] = !lights[[i, j]];
                    }
                }
            }
        };
    }

    lights.iter().filter(|&&x| x).count()
}

fn run_part2(lines: &Vec<String>) -> u32 {
    let mut lights = Array::from_shape_simple_fn((1000, 1000), || 0u32);

    for line in lines {
        let command = match parse_line(line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        match command {
            Command::On(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights[[i, j]] += 1;
                    }
                }
            }
            Command::Off(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        if lights[[i, j]] > 0 {
                            lights[[i, j]] -= 1;
                        }
                    }
                }
            }
            Command::Toggle(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        lights[[i, j]] += 2;
                    }
                }
            }
        };
    }

    lights.iter().fold(0, |acc, x| acc + x)
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    On((usize, usize), (usize, usize)),
    Off((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

fn parse_line(line: &str) -> Result<Command, Box<dyn Error>> {
    let split: Vec<&str> = line.split(' ').collect();
    if split.len() != 4 && split.len() != 5 {
        return Err("invalid input".into());
    }

    match split[0] {
        "toggle" => {
            let split_start: Vec<&str> = split[1].split(',').collect();
            let split_end: Vec<&str> = split[3].split(',').collect();
            if split_start.len() != 2 || split_end.len() != 2 {
                return Err("failed to parse start and end".into());
            }

            let start: (usize, usize) = (split_start[0].parse()?, split_start[1].parse()?);
            let end: (usize, usize) = (split_end[0].parse()?, split_end[1].parse()?);

            Ok(Command::Toggle(start, end))
        }
        "turn" => {
            let split_start: Vec<&str> = split[2].split(',').collect();
            let split_end: Vec<&str> = split[4].split(',').collect();
            if split_start.len() != 2 || split_end.len() != 2 {
                return Err("failed to parse start and end".into());
            }

            let start: (usize, usize) = (split_start[0].parse()?, split_start[1].parse()?);
            let end: (usize, usize) = (split_end[0].parse()?, split_end[1].parse()?);

            match split[1] {
                "on" => Ok(Command::On(start, end)),
                "off" => Ok(Command::Off(start, end)),
                _ => Err("invalid action".into()),
            }
        }
        _ => Err("invalid verb".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_handle_correct() {
        assert_eq!(
            parse_line("turn on 0,0 through 999,999").unwrap(),
            Command::On((0, 0), (999, 999))
        );

        assert_eq!(
            parse_line("toggle 0,0 through 999,0").unwrap(),
            Command::Toggle((0, 0), (999, 0))
        );

        assert_eq!(
            parse_line("turn off 499,499 through 500,500").unwrap(),
            Command::Off((499, 499), (500, 500))
        );
    }

    #[test]
    fn parse_line_handle_fail() {
        assert!(parse_line("turn on").is_err());

        assert!(parse_line("turn on a,b through 1024,1024").is_err());

        assert!(parse_line("bleh on 194,194 through 500,500").is_err());
    }
}
