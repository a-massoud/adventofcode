// VERY slow. But, it works. The much better solution is to precompute the distances along the
// path, then just check if any two points are manhattan distance <= 20 apart & if their values
// differ by > 100

use anyhow::{anyhow, bail, Context};
use rayon::prelude::*;
use std::{
    collections::{BinaryHeap, HashMap},
    env,
    fmt::Display,
    fs,
    ops::{Add, AddAssign, Sub, SubAssign},
};

const CUTOFF_VALUE: i64 = 100;

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

type Graph = HashMap<Vec2, HashMap<Vec2, i64>>;

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
                let mut edges = HashMap::new();
                if y < lines.len() - 1 && lines[y + 1].get(x).is_some_and(|&x| x != b'#') {
                    edges.insert(Vec2::new(x.try_into()?, (y + 1).try_into()?), 1);
                }
                if y > 0 && lines[y - 1].get(x).is_some_and(|&x| x != b'#') {
                    edges.insert(Vec2::new(x.try_into()?, (y - 1).try_into()?), 1);
                }
                if line.get(x - 1).is_some_and(|&x| x != b'#') {
                    edges.insert(Vec2::new((x - 1).try_into()?, y.try_into()?), 1);
                }
                if line.get(x + 1).is_some_and(|&x| x != b'#') {
                    edges.insert(Vec2::new((x + 1).try_into()?, y.try_into()?), 1);
                }

                graph.insert(Vec2::new(x.try_into()?, y.try_into()?), edges);
            }
        }
    }

    Ok((start, end, graph))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DijkstraNode {
    v: Vec2,
    w: i64,
}

impl DijkstraNode {
    fn new(v: Vec2, w: i64) -> Self {
        Self { v, w }
    }
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.w.cmp(&self.w).then(self.v.cmp(&other.v))
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: Vec2, end: Vec2, graph: &Graph) -> HashMap<Vec2, (i64, Option<Vec2>)> {
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(DijkstraNode::new(start, 0));
    visited.insert(start, (0, None));
    while let Some(v) = queue.pop() {
        if v.v == end {
            break;
        }

        for (&e, &weight) in graph.get(&v.v).expect("malformed graph") {
            if visited.get(&e).is_some_and(|&(w, _)| w <= v.w + weight) {
                continue;
            }

            visited.insert(e, (v.w + weight, Some(v.v)));
            queue.push(DijkstraNode::new(e, v.w + weight));
        }
    }

    visited
}

fn get_shortest_path(start: Vec2, end: Vec2, graph: &Graph) -> Option<Vec<Vec2>> {
    let visited = dijkstra(start, end, graph);

    let mut c = Some(end);
    let mut path = Vec::new();
    while let Some(v) = c {
        path.push(v);
        c = visited[&v].1;
    }

    Some(path)
}

fn get_path_len(start: Vec2, end: Vec2, graph: &Graph) -> Option<i64> {
    Some(dijkstra(start, end, graph).get(&end)?.0)
}

fn n_with_glitches(
    start: Vec2,
    end: Vec2,
    graph: &Graph,
    glitch_length: i64,
) -> anyhow::Result<i64> {
    let path = get_shortest_path(start, end, graph).ok_or(anyhow!("no path found"))?;

    let t = path
        .par_iter()
        .map(|v| (v, &graph[v]))
        .map(|(&v, original)| {
            let mut t = 0;
            let mut mgraph = graph.clone();
            for e in (v.x - glitch_length..v.x + glitch_length + 1)
                .flat_map(|x| {
                    (v.y - glitch_length..v.y + glitch_length + 1).map(move |y| Vec2::new(x, y))
                })
                .filter(|&e| {
                    v.manhattan_dist(e) <= glitch_length
                        && graph.contains_key(&e)
                        && !graph[&v].contains_key(&e)
                })
            {
                *mgraph.get_mut(&v).unwrap() = HashMap::from([(e, v.manhattan_dist(e))]);

                let Some(glitch_len) = get_path_len(start, end, &mgraph) else {
                    continue;
                };
                if glitch_len <= path.len() as i64 - CUTOFF_VALUE {
                    t += 1;
                }
            }
            *mgraph.get_mut(&v).unwrap() = original.clone();

            t
        })
        .sum();

    Ok(t)
}
