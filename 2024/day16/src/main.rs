use anyhow::{anyhow, bail, Context};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::{env, fs, i64};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Vec2 {
    type Output = Self;

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
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign<i64> for Vec2 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<i64> for Vec2 {
    type Output = Self;

    fn mul(mut self, rhs: i64) -> Self::Output {
        self *= rhs;
        self
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0]);
    }

    let input =
        fs::read_to_string(&args[1]).with_context(|| format!("reading file `{}`", args[1]))?;
    let (walls, start, target) = parse_input(&input).with_context(|| format!("parsing input"))?;

    let (score, paths) = dijkstra(&walls, start, target).ok_or(anyhow!("failed to find path"))?;
    println!("Part 1: {}", score);
    println!("Part 2: {}", paths.len());

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<(HashSet<Vec2>, Vec2, Vec2)> {
    let mut walls = HashSet::new();
    let mut start = Vec2::default();
    let mut target = Vec2::default();

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.bytes().enumerate() {
            match ch {
                b'#' => {
                    walls.insert(Vec2::new(j.try_into()?, i.try_into()?));
                }
                b'S' => {
                    start = Vec2::new(j.try_into()?, i.try_into()?);
                }
                b'E' => {
                    target = Vec2::new(j.try_into()?, i.try_into()?);
                }
                b'.' => (),
                _ => bail!("unexpected character in input"),
            }
        }
    }

    Ok((walls, start, target))
}

#[derive(Debug, Default, PartialEq, Eq)]
struct DijkstraState {
    penalty: i64,
    pos: Vec2,
    dir: Vec2,
}

impl DijkstraState {
    pub fn new(penalty: i64, pos: Vec2, dir: Vec2) -> Self {
        Self { penalty, pos, dir }
    }
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .penalty
            .cmp(&self.penalty)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn dijkstra(walls: &HashSet<Vec2>, start: Vec2, target: Vec2) -> Option<(i64, HashSet<Vec2>)> {
    const DIRS: [Vec2; 4] = [
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: 1 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: -1 },
    ];

    let mut cellmap: HashMap<(Vec2, Vec2), (i64, Vec<(Vec2, Vec2)>)> = HashMap::new();
    let mut queue: BinaryHeap<DijkstraState> = BinaryHeap::new();

    cellmap.insert((start, Vec2::new(1, 0)), (0, Vec::new()));
    queue.push(DijkstraState::new(0, start, Vec2::new(1, 0)));
    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        let p = state.pos;
        let d = state.dir;
        if p == target {
            break;
        }
        let (penalty, _) = cellmap
            .get(&(p, d))
            .expect("value in queue but not cellmap");

        for (np, nd, npen) in [
            (p + d, d, penalty + 1),
            (p, Vec2::new(-d.y, d.x), penalty + 1000),
            (p, Vec2::new(d.y, -d.x), penalty + 1000),
        ] {
            if !walls.contains(&np) {
                if !cellmap.contains_key(&(np, nd)) || cellmap[&(np, nd)].0 > npen {
                    cellmap.insert((np, nd), (npen, vec![(p, d)]));
                    queue.push(DijkstraState::new(npen, np, nd));
                    continue;
                }
                if cellmap[&(np, nd)].0 == npen {
                    cellmap.get_mut(&(np, nd)).unwrap().1.push((p, d));
                }
            }
        }
    }

    let score = DIRS
        .iter()
        .filter_map(|&d| cellmap.get(&(target, d)).map(|&(p, _)| p))
        .min()?;
    let mut path = HashSet::new();
    let mut queue: Vec<(Vec2, Vec2)> = DIRS
        .iter()
        .filter_map(|&d| {
            if let Some((p, _)) = cellmap.get(&(target, d)) {
                if *p == score {
                    Some((target, d))
                } else {
                    None
                }
            } else {
                return None;
            }
        })
        .collect();
    while !queue.is_empty() {
        let (p, d) = queue.pop().unwrap();
        path.insert(p);
        queue.append(&mut cellmap[&(p, d)].1.clone());
    }

    Some((score, path))
}
