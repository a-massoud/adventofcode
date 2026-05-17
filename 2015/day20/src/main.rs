use std::{env, iter};

fn main() {
    let input = match env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("no argument provided");
            return;
        }
    };

    let input: i64 = match input.parse::<i64>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse `{}`: {}", input, e);
            return;
        }
    };

    let part1 = first_greater(input);
    println!("Part 1: {}", part1);

    let part2 = first_greater_lazy(input);
    println!("Part 2: {}", part2);
}

fn first_greater(input: i64) -> usize {
    if input < 0 {
        return 0;
    }
    let mut bound = (input as usize) / 20 + 1;

    loop {
        let mut houses: Vec<i64> = iter::repeat_n(0, bound).collect();

        for elf in 1..=houses.len() {
            for i in (elf..=houses.len()).step_by(elf) {
                houses[i - 1] += elf as i64 * 10;
            }
        }

        if let Some(j) = houses.iter().position(|&v| v >= input) {
            return j + 1;
        } else {
            bound *= 2;
        }
    }
}

fn first_greater_lazy(input: i64) -> usize {
    if input < 0 {
        return 0;
    }
    let mut bound = (input as usize) / 20 + 1;

    loop {
        let mut houses: Vec<i64> = iter::repeat_n(0, bound).collect();

        for elf in 1..=houses.len() {
            for i in 1..=50 {
                if elf * i > houses.len() {
                    break;
                }

                houses[i * elf - 1] += elf as i64 * 11;
            }
        }

        if let Some(j) = houses.iter().position(|&v| v >= input) {
            return j + 1;
        } else {
            bound *= 2;
        }
    }
}
