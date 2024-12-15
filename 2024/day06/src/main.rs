// inefficient solution: meet parallelization

use anyhow::{anyhow, bail};
use rayon::prelude::*;
use std::collections::HashSet;
use std::ops::{Add, AddAssign};
use std::{env, fs, mem};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn in_box(&self, upper_left: Point, lower_right: Point) -> bool {
        self.x >= upper_left.x
            && self.y >= upper_left.y
            && self.x <= lower_right.x
            && self.y <= lower_right.y
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    loc: Point,
    dir: Point,
}

#[derive(Debug, Default, Clone)]
struct Map {
    map: HashSet<Point>,
    width: i32,
    height: i32,
    guard: Guard,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Usage: {} <input file>", args[0])
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    println!("Part 1 result: {}", part1(&input)?);
    println!("Part 2 result: {}", part2(&input)?);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Map> {
    let mut map = HashSet::new();
    let mut width = 0i32;
    let mut height = 0i32;
    let mut guard = Guard::default();

    for (y, line) in input.lines().enumerate() {
        if line.len() > (width as usize) {
            width = line
                .len()
                .try_into()
                .map_err(|_| anyhow!("input too wide"))?;
        }
        height += 1;
        for (x, char) in line.chars().enumerate() {
            let pt = Point {
                x: x.try_into().map_err(|_| anyhow!("input too wide"))?,
                y: y.try_into().map_err(|_| anyhow!("input too tall"))?,
            };
            match char {
                '#' => {
                    map.insert(pt);
                }
                '^' => {
                    guard = Guard {
                        loc: pt,
                        dir: Point { x: 0, y: -1 },
                    }
                }
                _ => (),
            }
        }
    }

    Ok(Map {
        map,
        width,
        height,
        guard,
    })
}

fn get_visited_locs(input: &Map) -> (HashSet<Point>, bool) {
    let mut visited = HashSet::new();
    let mut guard = input.guard;

    while guard.loc.in_box(
        Point { x: 0, y: 0 },
        Point {
            x: input.width - 1,
            y: input.height - 1,
        },
    ) && !visited.contains(&guard)
    {
        visited.insert(guard);

        if input.map.contains(&(guard.loc + guard.dir)) {
            mem::swap(&mut guard.dir.x, &mut guard.dir.y);
            guard.dir.x *= -1;
        } else {
            guard.loc += guard.dir;
        }
    }

    let in_loop = visited.contains(&guard);
    let mut locs = HashSet::new();
    for loc in visited {
        locs.insert(loc.loc);
    }
    (locs, in_loop)
}

fn part1(input: &Map) -> anyhow::Result<i32> {
    get_visited_locs(input)
        .0
        .len()
        .try_into()
        .map_err(|_| anyhow!("path too long"))
}

fn part2(input: &Map) -> anyhow::Result<i32> {
    let possible_locs = get_visited_locs(input).0;

    std::iter::repeat(input.clone())
        .take(possible_locs.len())
        .zip(possible_locs.iter())
        .par_bridge()
        .map(|(mut map, loc)| {
            map.map.insert(*loc);
            get_visited_locs(&map).1
        })
        .filter(|&x| x)
        .count()
        .try_into()
        .map_err(|_| anyhow!("too many options"))
}
