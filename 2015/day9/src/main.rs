use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return Ok(());
    }

    let lines: Vec<_> = BufReader::new(File::open(&args[1])?)
        .lines()
        .collect::<Result<_, _>>()?;

    let nodes = build_nodes(&lines)?;

    println!("Part 1 result: {}", get_shortest_path(&nodes));
    println!("Part 2 result: {}", get_longest_path(&nodes));

    Ok(())
}

fn build_nodes(
    lines: &Vec<String>,
) -> Result<HashMap<String, HashMap<String, i32>>, Box<dyn Error>> {
    let mut nodes = HashMap::new();

    for line in lines {
        let split_line: Vec<_> = line.split(' ').collect();
        if split_line.len() != 5 {
            return Err((String::from("failed to parse line `")
                + line
                + "`: invalid split length")
                .into());
        }

        if !nodes.contains_key(split_line[0]) {
            nodes.insert(String::from(split_line[0]), HashMap::new());
        }
        if !nodes.contains_key(split_line[2]) {
            nodes.insert(String::from(split_line[2]), HashMap::new());
        }

        nodes
            .get_mut(split_line[0])
            .unwrap()
            .insert(String::from(split_line[2]), split_line[4].parse::<i32>()?);
        nodes
            .get_mut(split_line[2])
            .unwrap()
            .insert(String::from(split_line[0]), split_line[4].parse::<i32>()?);
    }

    return Ok(nodes);
}

fn get_shortest_path(nodes: &HashMap<String, HashMap<String, i32>>) -> i32 {
    nodes
        .iter()
        .permutations(nodes.len())
        .map(|x| {
            x.windows(2)
                .fold(0, |acc, path| acc + path[0].1.get(path[1].0).unwrap())
        })
        .min()
        .unwrap()
}

fn get_longest_path(nodes: &HashMap<String, HashMap<String, i32>>) -> i32 {
    nodes
        .iter()
        .permutations(nodes.len())
        .map(|x| {
            x.windows(2)
                .fold(0, |acc, path| acc + path[0].1.get(path[1].0).unwrap())
        })
        .max()
        .unwrap()
}
