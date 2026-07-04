// Fun puzzle

use std::{
    collections::{HashMap, VecDeque, hash_map::Entry},
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result, anyhow, bail};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        bail!("No input provided");
    }
    let grid = read_input(&args[1]).context("Failed to read input")?;

    println!("===Part 1===");
    let viable_pairs = grid.count_viable_pairs();
    println!("Viable pairs: {}", viable_pairs);
    println!();

    println!("===Part 2===");
    let minimum_path = grid
        .get_shortest_path()
        .ok_or_else(|| anyhow!("No path for data found"))?;
    println!("Minimum path length: {}", minimum_path.len());

    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    const fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }

    fn neighbors_iter(self) -> impl Iterator<Item = Self> {
        const DIRECTIONS: [Coord; 4] = [
            Coord::new(1, 0),
            Coord::new(0, -1),
            Coord::new(-1, 0),
            Coord::new(0, 1),
        ];
        DIRECTIONS
            .into_iter()
            .map(move |dir| Coord::new(self.x + dir.x, self.y + dir.y))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    size_tb: u16,
    used_tb: u16,
}

impl Node {
    fn new(size_tb: u16, used_tb: u16) -> Self {
        Self { size_tb, used_tb }
    }

    fn avail_tb(&self) -> u16 {
        self.size_tb - self.used_tb
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Grid {
    data: Vec<Node>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(map: HashMap<Coord, Node>) -> Result<Self> {
        let (max_x, max_y) = map.keys().try_fold((0, 0), |(max_x, max_y), coord| {
            if max_x >= 0 && max_y >= 0 {
                Ok((max_x.max(coord.x), max_y.max(coord.y)))
            } else {
                Err(anyhow!("Negative coordinate"))
            }
        })?;

        let width = (max_x + 1) as usize;
        let height = (max_y + 1) as usize;

        let data: Vec<_> = (0..height)
            .flat_map(|y| (0..width).map(move |x| Coord::new(x as i64, y as i64)))
            .map(|coord| {
                map.get(&coord)
                    .ok_or_else(|| anyhow!("Coordinate ({}, {}) does not exist", coord.x, coord.y))
                    .copied()
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            data,
            width,
            height,
        })
    }

    fn count_viable_pairs(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, node)| node.used_tb > 0)
            .flat_map(|a| self.data.iter().enumerate().map(move |b| (a, b)))
            .filter(|((a_coord, a_node), (b_coord, b_node))| {
                a_coord != b_coord && a_node.used_tb <= b_node.avail_tb()
            })
            .count()
    }

    fn get(&self, coord: Coord) -> Option<&Node> {
        if coord.x >= 0
            && coord.y >= 0
            && (coord.x as usize) < self.width
            && (coord.y as usize) < self.height
        {
            let idx = (coord.x as usize) + (coord.y as usize) * self.width;
            Some(&self.data[idx])
        } else {
            None
        }
    }

    // Assumes the only valid moves are to or from the empty node, and all non-empty nodes are
    // interchangeable
    fn get_shortest_path(&self) -> Option<Vec<(Coord, Coord)>> {
        let goal = Coord::new((self.width - 1) as i64, 0);
        let empty_idx = self.data.iter().position(|node| node.used_tb == 0)?;
        let empty = Coord::new(
            (empty_idx % self.width) as i64,
            (empty_idx / self.width) as i64,
        );

        let mut queue = VecDeque::from([(goal, empty)]);
        let mut parents = HashMap::from([((goal, empty), (goal, empty))]);

        while let Some((goal, empty)) = queue.pop_front() {
            let empty_node = self
                .get(empty)
                .expect("Only valid coordinates entered in queue");
            for new_empty in empty.neighbors_iter().filter(|to| {
                self.get(*to)
                    .map(|to| to.used_tb < empty_node.size_tb)
                    .unwrap_or(false)
            }) {
                let new_goal = if new_empty == goal { empty } else { goal };

                if new_goal == Coord::new(0, 0) {
                    let mut path = vec![(goal, empty)];
                    while let Some(state) = parents.get(path.last().unwrap())
                        && state != path.last().unwrap()
                    {
                        path.push(*state);
                    }
                    path.reverse();

                    return Some(path);
                }

                // reuse parents map as visited set
                match parents.entry((new_goal, new_empty)) {
                    Entry::Occupied(_) => {
                        continue;
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert((goal, empty));
                    }
                }

                queue.push_back((new_goal, new_empty))
            }
        }

        None
    }
}

fn parse_input(input: impl BufRead) -> Result<Grid> {
    let mut lines = input.lines().enumerate().map(|(no, line)| (no + 1, line));
    let first_line = lines
        .next()
        .ok_or_else(|| anyhow!("Input is empty"))?
        .1
        .context("Failed to read line 1")?;
    let first_line = first_line.trim();
    if first_line != "root@ebhq-gridcenter# df -h" {
        bail!("First line `{}` is invalid", first_line);
    }

    let second_line = lines
        .next()
        .ok_or_else(|| anyhow!("Input does not contain second line"))?
        .1
        .context("Failed to rad line 2")?;
    if second_line
        .split_whitespace()
        .ne(["Filesystem", "Size", "Used", "Avail", "Use%"])
    {
        bail!(
            "Second line `{}` does not match expected headings",
            second_line
        );
    }

    let map = lines
        .map(|(no, line)| {
            let line = line.with_context(|| format!("Failed to read line {}", no))?;

            let mut split = line.split_whitespace();
            let (x, y) = split
                .next()
                .ok_or_else(|| anyhow!("Expected filesystem on line {}", no))?
                .strip_prefix("/dev/grid/node-x")
                .ok_or_else(|| anyhow!("Filesystem on line {} not in /dev/grid/...", no))?
                .split_once("-y")
                .ok_or_else(|| {
                    anyhow!("Filesystem on line {} does not contain y coordinate", no)
                })?;
            let x = x
                .parse()
                .with_context(|| format!("Failed to parse x on line {}", no))?;
            let y = y
                .parse()
                .with_context(|| format!("Failed to parse y on line {}", no))?;
            let coord = Coord::new(x, y);

            let size_tb = split
                .next()
                .ok_or_else(|| anyhow!("Expected size on line {}", no))?
                .strip_suffix('T')
                .ok_or_else(|| anyhow!("Expected size in TB on line {}", no))?
                .parse()
                .with_context(|| format!("Failed to parse size on line {}", no))?;
            let used_tb = split
                .next()
                .ok_or_else(|| anyhow!("Expected used on line {}", no))?
                .strip_suffix('T')
                .ok_or_else(|| anyhow!("Expected used in TB on line {}", no))?
                .parse()
                .with_context(|| format!("Failed to parse used on line {}", no))?;
            let avail_tb: u16 = split
                .next()
                .ok_or_else(|| anyhow!("Expected avail on line {}", no))?
                .strip_suffix('T')
                .ok_or_else(|| anyhow!("Expected avail in TB on line {}", no))?
                .parse()
                .with_context(|| format!("Failed to parse avail on line {}", no))?;
            let used_percent: u16 = split
                .next()
                .ok_or_else(|| anyhow!("Expected used % on line {}", no))?
                .strip_suffix('%')
                .ok_or_else(|| anyhow!("Expected used % in percent on line {}", no))?
                .parse()
                .with_context(|| format!("Failed to parse used % on line {}", no))?;

            if avail_tb != size_tb - used_tb {
                bail!("Bad avail on line {}", no);
            }

            if used_percent != 100 * used_tb / size_tb {
                bail!("Bad use % on line {}", no);
            }

            let node = Node::new(size_tb, used_tb);

            Ok((coord, node))
        })
        .collect::<Result<_>>()?;

    Grid::new(map).context("Failed to construct grid")
}

fn read_input(path: impl AsRef<Path>) -> Result<Grid> {
    let path = path.as_ref();
    let input = BufReader::new(
        File::open(path)
            .with_context(|| format!("Failed to open `{}` for reading", path.display()))?,
    );
    parse_input(input).with_context(|| format!("Failed to parse file `{}`", path.display()))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::parse_input;

    const SAMPLE_INPUT: &str = "\
root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%";

    #[test]
    fn sample_input_p2() {
        let grid = parse_input(Cursor::new(SAMPLE_INPUT)).expect("Sample input parses");
        let minimum_path = grid
            .get_shortest_path()
            .expect("Sample input has shortest path");
        assert_eq!(minimum_path.len(), 7);
    }
}
