use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::{Add, Mul, Sub};
use std::{env, fs};

use anyhow::bail;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Mul<i32, Output = i32> + Clone> Mul<T> for Point {
    type Output = Point;

    fn mul(self, rhs: T) -> Self::Output {
        Point {
            x: rhs.clone() * self.x,
            y: rhs * self.y,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Map {
    antenna: HashMap<char, BTreeSet<Point>>,
    width: usize,
    height: usize,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Usage: {} <input file>", args[0]);
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input);

    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));

    Ok(())
}

fn parse_input(input: &str) -> Map {
    let mut antenna: HashMap<char, BTreeSet<Point>> = HashMap::new();
    let mut width = 0usize;
    let mut height = 0usize;

    for (y, line) in input.lines().enumerate() {
        if y > height {
            height = y + 1;
        }
        for (x, ch) in line.chars().enumerate() {
            if x > width {
                width = x + 1;
            }
            match ch {
                '.' => (),
                _ => match antenna.get_mut(&ch) {
                    Some(v) => {
                        v.insert(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    None => {
                        let mut v = BTreeSet::new();
                        v.insert(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                        antenna.insert(ch, v);
                    }
                },
            }
        }
    }

    Map {
        antenna,
        width,
        height,
    }
}

fn part1(input: &Map) -> i32 {
    input
        .antenna
        .iter()
        .flat_map(|(_, pts)| {
            pts.iter()
                .enumerate()
                .flat_map(|(i, &x)| {
                    pts.iter()
                        .skip(i + 1)
                        .flat_map(move |&y| [x + (x - y), (y - (x - y))].into_iter())
                })
                .filter(|pt| {
                    pt.x >= 0
                        && pt.y >= 0
                        && (pt.x as usize) < input.width
                        && (pt.y as usize) < input.height
                })
        })
        .collect::<HashSet<Point>>()
        .len() as i32
}

fn part2(input: &Map) -> i32 {
    let n_harmonics = isize::max(input.width as isize, input.height as isize);

    let hs = input
        .antenna
        .iter()
        .flat_map(|(_, pts)| {
            pts.iter()
                .enumerate()
                .flat_map(|(i, &x)| {
                    pts.iter().skip(i + 1).flat_map(move |&y| {
                        let diff = y - x;
                        ((-n_harmonics)..(n_harmonics + 1)).map(move |n| x + diff * (n as i32))
                    })
                })
                .filter(|pt| {
                    pt.x >= 0
                        && pt.y >= 0
                        && (pt.x as usize) < input.width
                        && (pt.y as usize) < input.height
                })
        })
        .collect::<HashSet<Point>>();

    hs.len() as i32
}
