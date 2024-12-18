// Fast now

use anyhow::{anyhow, bail, Context};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    fmt::Display,
    fs,
};

const PART1_AMT: usize = 1024;
const GRID_MAX: i64 = 70;
const START: Vec2 = Vec2::new(0, 0);
const END: Vec2 = Vec2::new(GRID_MAX, GRID_MAX);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0]);
    }

    let input =
        fs::read_to_string(&args[1]).with_context(|| format!("reading file `{}`", args[1]))?;
    let input = parse_input(&input).with_context(|| "parsing input")?;

    println!(
        "Part 1: {}",
        dijkstra(START, END, &simulate_falling(&input, PART1_AMT))
            .ok_or(anyhow!("failed to find path"))?
            - 1
    );

    println!(
        "Part 2: {}",
        find_blocking_coord(&input).ok_or(anyhow!("path never blocked"))?
    );

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec2>> {
    let mut bytes = Vec::new();

    for line in input.lines() {
        let (x, y) = line.split_once(',').ok_or(anyhow!("line has no `,`"))?;
        let x = x.parse()?;
        let y = y.parse()?;
        bytes.push(Vec2::new(x, y));
    }

    Ok(bytes)
}

fn simulate_falling(input: &[Vec2], n: usize) -> HashSet<Vec2> {
    input.iter().take(n).cloned().collect()
}

fn dijkstra(start: Vec2, end: Vec2, walls: &HashSet<Vec2>) -> Option<usize> {
    const DIRS: [Vec2; 4] = [
        Vec2::new(1, 0),
        Vec2::new(0, 1),
        Vec2::new(-1, 0),
        Vec2::new(0, -1),
    ];

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    struct DijkstraItem {
        p: i64,
        k: Vec2,
    }

    impl DijkstraItem {
        const fn new(priority: i64, position: Vec2) -> Self {
            Self {
                p: priority,
                k: position,
            }
        }
    }

    impl Ord for DijkstraItem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.p.cmp(&self.p).then(self.k.cmp(&other.k))
        }
    }

    impl PartialOrd for DijkstraItem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut visited: HashMap<Vec2, (i64, Option<Vec2>)> = HashMap::new();
    let mut queue = BinaryHeap::new();
    visited.insert(start, (0, None));
    queue.push(DijkstraItem::new(0, start));

    while !queue.is_empty() {
        let x = queue.pop().unwrap().k;
        if x == end {
            break;
        }
        let (p, _) = *visited.get(&x).expect("point in queue but not visited");

        for dir in DIRS {
            let nx = Vec2::new(x.x + dir.x, x.y + dir.y);
            if walls.contains(&nx) || nx.x < 0 || nx.y < 0 || nx.x > GRID_MAX || nx.y > GRID_MAX {
                continue;
            }

            if !visited.contains_key(&nx) || visited[&nx].0 > p + 1 {
                visited.insert(nx, (p + 1, Some(x)));
                queue.push(DijkstraItem::new(p + 1, nx));
            }
        }
    }

    let mut r = 1;
    let mut c = visited.get(&end)?.1;
    while c.is_some() {
        r += 1;
        c = visited.get(&c.unwrap()).expect("parent not visited").1;
    }

    Some(r)
}

fn find_blocking_coord(input: &[Vec2]) -> Option<Vec2> {
    let mut max = input.len();
    let mut min = 0;
    while min < max - 1 {
        let c = (min + max) / 2;
        if dijkstra(START, END, &simulate_falling(input, c)).is_some() {
            min = c;
        } else {
            max = c;
        }
    }

    if max == input.len() {
        None
    } else {
        Some(input[min])
    }
}
