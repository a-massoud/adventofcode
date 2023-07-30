use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return Ok(());
    }

    let input = fs::read_to_string(&args[1])?;

    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> i64 {
    let mut level = 0i64;

    for c in input.chars() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => (),
        }
    }

    level
}

fn part2(input: &str) -> usize {
    let mut level = 0i64;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => (),
        }

        if level < 0 {
            return i + 1;
        }
    }

    usize::MAX
}
