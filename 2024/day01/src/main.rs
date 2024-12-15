// Good first puzzle!

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return;
    }

    let input = read_file(&args[1]);
    let Ok(input) = input else {
        eprintln!("Error: {}", input.err().unwrap());
        return;
    };

    println!("Part 1 results: {}", part1(&input));
    println!("Part 2 results: {}", part2(&input));
}

fn read_file(fname: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let Ok(file) = File::open(fname) else {
        return Err(format!("failed to open {} for reading", fname).into());
    };
    let mut file = BufReader::new(file);

    let mut input = String::new();
    let Ok(_) = file.read_to_string(&mut input) else {
        return Err(format!("failed to read from {}", fname).into());
    };

    let list1 = input
        .split_whitespace()
        .enumerate()
        .filter_map(|(i, v)| if i % 2 == 0 { v.parse().ok() } else { None })
        .collect();
    let list2 = input
        .split_whitespace()
        .enumerate()
        .filter_map(|(i, v)| if i % 2 == 1 { v.parse().ok() } else { None })
        .collect();

    Ok((list1, list2))
}

fn part1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut list1 = input.0.clone();
    let mut list2 = input.1.clone();

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .fold(0i32, |acc, (&i1, &i2)| acc + (i1 - i2).abs())
}

fn part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for i in &input.1 {
        match counts.get_mut(i) {
            Some(v) => *v += 1,
            None => {
                counts.insert(*i, 1);
            }
        }
    }

    input.0.iter().fold(0i32, |acc, v| {
        acc + match counts.get(v) {
            Some(&c) => c * v,
            None => 0,
        }
    })
}
