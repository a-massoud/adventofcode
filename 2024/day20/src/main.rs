// I still am actually proud of my first solution but this is undeniably better

use anyhow::{bail, Context};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fmt::Display;
use std::fs;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhattan_dist(self, other: Vec2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

type Graph = HashMap<Vec2, HashSet<Vec2>>;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        bail!("usage: {} <input file> <cutoff difference>", args[0]);
    }

    let Ok(cutoff_diff) = args[2].parse() else {
        bail!("cutoff difference must be a positive integer")
    };

    let input = fs::read_to_string(&args[1]).context("reading input file")?;
    let (start, end, graph) = parse_input(&input).context("parsing input")?;

    println!(
        "Part 1: {}",
        n_with_glitches(start, end, &graph, 2, cutoff_diff)?
    );
    println!(
        "Part 2: {}",
        n_with_glitches(start, end, &graph, 20, cutoff_diff)?
    );

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<(Vec2, Vec2, Graph)> {
    let mut start = Vec2::default();
    let mut end = Vec2::default();
    let mut graph = HashMap::new();

    let lines: Vec<Vec<u8>> = input.lines().map(|x| x.into()).collect();
    for (y, line) in lines.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            match ch {
                b'S' => {
                    start = Vec2::new(x.try_into()?, y.try_into()?);
                }
                b'E' => {
                    end = Vec2::new(x.try_into()?, y.try_into()?);
                }
                b'#' => (),
                b'.' => (),
                _ => {
                    bail!("unexpected character in input: {}", ch);
                }
            }

            if ch != b'#' {
                let mut edges = HashSet::new();
                if y < lines.len() - 1 && lines[y + 1].get(x).is_some_and(|&x| x != b'#') {
                    edges.insert(Vec2::new(x.try_into()?, (y + 1).try_into()?));
                }
                if y > 0 && lines[y - 1].get(x).is_some_and(|&x| x != b'#') {
                    edges.insert(Vec2::new(x.try_into()?, (y - 1).try_into()?));
                }
                if line.get(x - 1).is_some_and(|&x| x != b'#') {
                    edges.insert(Vec2::new((x - 1).try_into()?, y.try_into()?));
                }
                if line.get(x + 1).is_some_and(|&x| x != b'#') {
                    edges.insert(Vec2::new((x + 1).try_into()?, y.try_into()?));
                }

                graph.insert(Vec2::new(x.try_into()?, y.try_into()?), edges);
            }
        }
    }

    Ok((start, end, graph))
}

fn get_distances(start: Vec2, graph: &Graph) -> HashMap<Vec2, usize> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start, (0, None));
    while let Some(v) = queue.pop_front() {
        let d = visited.get(&v).unwrap().0;
        for &e in graph.get(&v).expect("malformed graph") {
            visited.entry(e).or_insert_with(|| {
                queue.push_back(e);
                (d + 1, Some(v))
            });
        }
    }

    visited.into_iter().map(|(v, (d, _))| (v, d)).collect()
}

fn n_with_glitches(
    start: Vec2,
    end: Vec2,
    graph: &Graph,
    glitch_length: usize,
    cutoff_diff: usize,
) -> anyhow::Result<i64> {
    let start_distances = get_distances(start, graph);
    let Some(len) = start_distances.get(&end) else {
        bail!("no path found")
    };
    let target = len - cutoff_diff;

    let end_distances = get_distances(end, graph);

    let mut t = 0;
    for &v in graph.keys() {
        for &w in graph.keys() {
            let d: usize = v.manhattan_dist(w).try_into().unwrap();
            let Some(sd) = start_distances.get(&v) else {
                continue;
            };
            let Some(ed) = end_distances.get(&w) else {
                continue;
            };
            if d <= glitch_length && sd + d + ed <= target {
                t += 1;
            }
        }
    }

    Ok(t)
}
