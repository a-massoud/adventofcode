// Really easy again... I do like how the part 1 and part 2 solutions are almost identical though!

use anyhow::{anyhow, bail};
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let (source, splitters, end) = parse_input(&input)?;

    let p1 = sim_tachyon(source, &splitters, end);
    println!("Part 1: {}", p1);

    let p2 = sim_quantum_tachyon(source, &splitters, end);
    println!("Part 2: {}", p2);

    Ok(())
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

fn parse_input(input: &str) -> anyhow::Result<(Point, HashSet<Point>, i64)> {
    let mut source = None;
    let mut splitters = HashSet::new();
    let mut len = 0;

    for (y, line) in input.lines().enumerate() {
        let line = line.trim();
        len += 1;
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    source = Some(Point::new(x as i64, y as i64));
                }
                '^' => {
                    splitters.insert(Point::new(x as i64, y as i64));
                }
                '.' => (),
                _ => {
                    bail!("bad character `{}`", ch);
                }
            }
        }
    }

    let source = source.ok_or(anyhow!("no source found"))?;

    Ok((source, splitters, len))
}

fn sim_tachyon(source: Point, splitters: &HashSet<Point>, end: i64) -> i64 {
    let mut tachyons: HashSet<_> = [source].into();
    let mut splits = 0;

    for _ in 0..(end - source.y) {
        let mut next_tachyons = HashSet::new();

        for tach in tachyons {
            if splitters.contains(&(tach + Point::new(0, 1))) {
                splits += 1;
                if !splitters.contains(&(tach + Point::new(1, 1))) {
                    next_tachyons.insert(tach + Point::new(1, 1));
                }
                if !splitters.contains(&(tach + Point::new(-1, 1))) {
                    next_tachyons.insert(tach + Point::new(-1, 1));
                }
            } else {
                next_tachyons.insert(tach + Point::new(0, 1));
            }
        }

        tachyons = next_tachyons;
    }

    splits
}

fn sim_quantum_tachyon(source: Point, splitters: &HashSet<Point>, end: i64) -> i64 {
    let mut tachyons = HashMap::new();
    tachyons.insert(source, 1i64);

    for _ in 0..(end - source.y) {
        let mut next_tachyons = HashMap::new();

        for (tach, mult) in tachyons {
            if splitters.contains(&(tach + Point::new(0, 1))) {
                if !splitters.contains(&(tach + Point::new(1, 1))) {
                    next_tachyons.entry(tach + Point::new(1, 1)).and_modify(|x| *x += mult).or_insert(mult);
                }
                if !splitters.contains(&(tach + Point::new(-1, 1))) {
                    next_tachyons.entry(tach + Point::new(-1, 1)).and_modify(|x| *x += mult).or_insert(mult);
                }
            } else {
                next_tachyons.entry(tach + Point::new(0, 1)).and_modify(|x| *x += mult).or_insert(mult);
            }
        }

        tachyons = next_tachyons;
    }

    tachyons.values().sum()
}
