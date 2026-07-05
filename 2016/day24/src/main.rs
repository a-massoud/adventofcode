// Easy enough to just brute force

use std::{
    collections::{HashMap, HashSet, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::Path,
};

use eyre::{Context, OptionExt, Result, bail, eyre};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        bail!("No input provided");
    }

    let map = read_input(&args[1]).context("Failed to read input")?;
    let distances = map
        .into_distances()
        .context("Failed to create distance map")?;

    println!(" Part 1 ");
    println!("========");
    let (_, n_steps) = distances.find_shortest_path(false);
    println!("Shortest path: {} steps", n_steps);
    println!();

    println!(" Part 2 ");
    println!("========");
    let (_, n_steps) = distances.find_shortest_path(true);
    println!("Shortest path: {} steps", n_steps);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub const fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }

    pub fn neighbors_iter(&self) -> impl Iterator<Item = Coord> {
        const DIRECTIONS: [(i64, i64); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

        DIRECTIONS
            .into_iter()
            .map(|(dx, dy)| Coord::new(self.x + dx, self.y + dy))
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    targets: Vec<Coord>,
}

impl Map {
    pub fn new(tiles: Vec<Tile>, width: usize, height: usize, targets: Vec<Coord>) -> Self {
        Map {
            tiles,
            width,
            height,
            targets,
        }
    }

    fn distance_between(&self, start: Coord, end: Coord) -> Option<u64> {
        if start == end {
            return Some(0);
        }

        let mut queue = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::from([start]);

        while let Some((pos, n)) = queue.pop_front() {
            for new_pos in pos.neighbors_iter() {
                if new_pos.x < 0
                    || new_pos.y < 0
                    || new_pos.x as usize >= self.width
                    || new_pos.y as usize >= self.height
                {
                    continue;
                }

                let idx = (new_pos.x as usize) + (new_pos.y as usize) * self.width;
                if self.tiles[idx] == Tile::Wall {
                    continue;
                }

                if !visited.insert(new_pos) {
                    continue;
                }

                if new_pos == end {
                    return Some(n + 1);
                }

                queue.push_back((new_pos, n + 1));
            }
        }

        None
    }

    pub fn into_distances(self) -> Result<DistanceMap> {
        let n = self.targets.len();
        let mut distances = Vec::with_capacity(n * n);

        for (start_idx, start_coord) in self.targets.iter().enumerate() {
            for (end_idx, end_coord) in self.targets.iter().enumerate() {
                let dist = self
                    .distance_between(*start_coord, *end_coord)
                    .ok_or_else(|| eyre!("No path between {} and {} found", start_idx, end_idx))?;
                distances.push(dist);
            }
        }

        Ok(DistanceMap { distances, n })
    }
}

#[derive(Debug, Clone)]
struct DistanceMap {
    distances: Vec<u64>,
    n: usize,
}

impl DistanceMap {
    pub fn find_shortest_path(&self, should_return: bool) -> (Vec<usize>, u64) {
        (1..self.n)
            .permutations(self.n - 1)
            .map(|mut perm| {
                perm.insert(0, 0);
                if should_return {
                    perm.push(0);
                }
                perm
            })
            .map(|path| {
                let len = path
                    .windows(2)
                    .map(|link| self.distances[link[1] + link[0] * self.n])
                    .sum::<u64>();
                (path, len)
            })
            .min_by_key(|(_, len)| *len)
            .unwrap_or_else(|| (vec![0], 0))
    }
}

fn parse_input(input: impl BufRead) -> Result<Map> {
    let mut lines = input.lines().enumerate().map(|(no, line)| (no + 1, line));

    let (_, first_line) = lines.next().ok_or_eyre("Input is empty")?;
    let first_line = first_line.context("Failed to read line 1")?;
    let first_line_trimmed = first_line.trim();
    let width = first_line_trimmed.len();

    let mut tiles = Vec::new();
    let mut targets_map = HashMap::new();

    for (no, line) in iter::once((1, Ok(first_line))).chain(lines) {
        let line = line.with_context(|| format!("Failed to read line {}", no))?;
        let line = line.trim();

        if line.len() != width {
            bail!("Line {} has invalid width {}", no, line.len());
        }
        tiles.reserve(width);

        for (x, ch) in line.bytes().enumerate() {
            match ch {
                b'#' => tiles.push(Tile::Wall),
                b'.' => tiles.push(Tile::Empty),
                b'0'..=b'9' => {
                    targets_map.insert((ch - b'0') as usize, Coord::new(x as i64, (no as i64) - 1));
                    tiles.push(Tile::Empty)
                }
                _ => {
                    bail!("Invalid input character `{}` on line {}", ch as char, no);
                }
            }
        }
    }

    let height = tiles.len() / width;

    let n_targets = targets_map
        .keys()
        .copied()
        .max()
        .ok_or_eyre("No targets found")?;
    let targets: Vec<_> = (0..=n_targets)
        .map(|i| {
            targets_map
                .get(&i)
                .copied()
                .ok_or_else(|| eyre!("Target {} not found", i))
        })
        .collect::<Result<_>>()?;

    Ok(Map::new(tiles, width, height, targets))
}

fn read_input(path: impl AsRef<Path>) -> Result<Map> {
    let path = path.as_ref();
    let input = BufReader::new(
        File::open(path).with_context(|| format!("Failed to open `{}`", path.display()))?,
    );
    parse_input(input).with_context(|| format!("Failed to read `{}`", path.display()))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::parse_input;

    const SAMPLE_INPUT: &str = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    #[test]
    fn sample_input() {
        let map = parse_input(Cursor::new(SAMPLE_INPUT)).expect("Sample input parses");
        let distances = map
            .into_distances()
            .expect("Sample input forms distance map");
        let (path, len) = distances.find_shortest_path(false);
        assert_eq!(path, [0, 4, 1, 2, 3], "Sample input gives correct path");
        assert_eq!(len, 14, "Sample input gives correct length")
    }
}
