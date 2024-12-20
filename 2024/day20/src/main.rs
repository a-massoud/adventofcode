// I still am actually proud of my first solution but this is undeniably better

use anyhow::{anyhow, bail, Context};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fmt::Display;
use std::fs;
use std::ops::{Add, AddAssign, Sub, SubAssign};

const CUTOFF_VALUE: usize = 100;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhattan_dist(self, other: Vec2) -> i64 {
        let d = self - other;
        d.x.abs() + d.y.abs()
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
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
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0]);
    }

    let input = fs::read_to_string(&args[1]).context("reading input file")?;
    let (start, end, graph) = parse_input(&input).context("parsing input")?;

    println!("Part 1: {}", n_with_glitches(start, end, &graph, 2)?);
    println!("Part 2: {}", n_with_glitches(start, end, &graph, 20)?);

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

fn get_shortest_path(start: Vec2, end: Vec2, graph: &Graph) -> Option<Vec<Vec2>> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start, None);
    while let Some(v) = queue.pop_front() {
        if v == end {
            continue;
        }
        for &e in graph.get(&v).expect("malformed graph") {
            visited.entry(e).or_insert_with(|| {
                queue.push_back(e);
                Some(v)
            });
        }
    }
    if !visited.contains_key(&end) {
        return None;
    }

    let mut c = Some(end);
    let mut path = Vec::new();
    while let Some(v) = c {
        path.push(v);
        c = visited[&v];
    }
    path.reverse();

    Some(path)
}

fn n_with_glitches(
    start: Vec2,
    end: Vec2,
    graph: &Graph,
    glitch_length: usize,
) -> anyhow::Result<i64> {
    let path = get_shortest_path(start, end, graph).ok_or(anyhow!("no path found"))?;

    let mut t = 0;
    for (i, &v) in path.iter().enumerate() {
        for (j, &w) in path.iter().enumerate().skip(i + CUTOFF_VALUE) {
            let d: usize = v.manhattan_dist(w).try_into().unwrap();
            if d <= glitch_length && j - i >= CUTOFF_VALUE + d {
                t += 1;
            }
        }
    }

    Ok(t)
}
