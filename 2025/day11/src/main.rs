// Nothing a bit of dynamic programming can't solve!

use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let input = parse_input(&input)?;

    let mut cache = HashMap::new();

    let p1 = get_n_paths(&input, "you", "out", &mut cache);
    println!("Part 1: {}", p1);

    let p2 = get_n_paths(&input, "svr", "fft", &mut cache)
        * get_n_paths(&input, "fft", "dac", &mut cache)
        * get_n_paths(&input, "dac", "out", &mut cache)
        + get_n_paths(&input, "svr", "dac", &mut cache)
            * get_n_paths(&input, "dac", "fft", &mut cache)
            * get_n_paths(&input, "fft", "out", &mut cache);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<HashMap<String, HashSet<String>>> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            let (k, v) = line.split_once(':').ok_or(anyhow!("line not separated"))?;

            Ok((
                k.to_owned(),
                v.split_whitespace().map(str::to_owned).collect(),
            ))
        })
        .collect()
}

fn get_n_paths(
    graph: &HashMap<String, HashSet<String>>,
    start: &str,
    end: &str,
    cache: &mut HashMap<(String, String), usize>,
) -> usize {
    if let Some(&r) = cache.get(&(start.to_owned(), end.to_owned())) {
        return r;
    }

    if start == end {
        return 1;
    }

    let Some(edges) = graph.get(start) else {
        return 0;
    };
    let r = edges
        .iter()
        .map(|v| get_n_paths(graph, v.as_str(), end, cache))
        .sum();

    cache.insert((start.to_owned(), end.to_owned()), r);

    r
}
