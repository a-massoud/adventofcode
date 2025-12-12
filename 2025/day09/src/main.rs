// Definitely not the most pretty solution, but I did have to fight against my instinct
// to brute force it; that did not work.

use anyhow::{Context, anyhow};
use std::collections::HashMap;
use std::hash::Hash;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let input = parse_input(&input)?;

    let p1 = find_max_rectangle(&input);
    println!("Part 1: {}", p1);

    let p2 = find_max_enclosed_rectangle(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

fn get_area(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once(',')
                .ok_or_else(|| anyhow!("bad line `{}`", line))?;
            let x = a
                .parse()
                .with_context(|| format!("failed to parse `{}`", a))?;
            let y = b
                .parse()
                .with_context(|| format!("failed to parse `{}`", b))?;
            Ok(Point::new(x, y))
        })
        .collect()
}

fn find_max_rectangle(input: &[Point]) -> i64 {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| input.iter().take(i).map(|b| get_area(a, b)))
        .max()
        .unwrap_or(0)
}

fn get_shape_interior(shape: &[Point]) -> HashMap<i64, Vec<(i64, i64)>> {
    if shape.is_empty() {
        return HashMap::new();
    }

    let edges: Vec<_> = shape
        .windows(2)
        .map(|point| (&point[0], &point[1]))
        .chain(
            shape
                .len()
                .ge(&2)
                .then(|| (&shape[shape.len() - 1], &shape[0]))
                .into_iter(),
        )
        .collect();

    let (ymin, ymax) = shape
        .iter()
        .skip(1)
        .fold((shape[0].y, shape[0].y), |acc, p| {
            (acc.0.min(p.y), acc.1.max(p.y))
        });

    let mut r: HashMap<_, _> = (ymin..=ymax)
        .map(|y| {
            let mut intercepts: Vec<_> = edges
                .iter()
                .filter(|(a, b)| (a.y > y) != (b.y > y) && a.y != b.y)
                .map(|(a, b)| ((y - a.y) * (b.x - a.x)) / (b.y - a.y) + a.x)
                .collect();
            intercepts.sort_unstable();
            let intercepts: Vec<_> = intercepts.chunks_exact(2).map(|c| (c[0], c[1])).collect();
            (y, intercepts)
        })
        .collect();

    for (a, b) in edges {
        if a.y == b.y {
            let range = (a.x.min(b.x), a.x.max(b.x));
            r.entry(a.y)
                .and_modify(|v| {
                    if v.is_empty() {
                        v.push(range);
                        return;
                    }

                    let mut min = range.0;
                    while min <= range.1 {
                        let n = v.iter().position(|rng| rng.0 > min);

                        if let Some(i) = n {
                            // i is now the index of the range that will be after our range
                            if i > 0 && v[i - 1].1 >= min {
                                min = v[i - 1].1 + 1;
                            } else if v[i].0 <= range.1 {
                                v.insert(i, (min, v[i].0 - 1));
                                min = v[i + 1].1 + 1;
                            } else {
                                v.insert(i, (min, range.1));
                                min = range.1 + 1;
                            }
                        } else {
                            let prev_min = v.last().unwrap().1;
                            if min <= prev_min {
                                min = prev_min + 1;
                            } else {
                                v.push((min, range.1));
                                min = range.1 + 1;
                            }
                        }
                    }
                })
                .or_insert(vec![range]);
        }
    }

    r
}

fn find_max_enclosed_rectangle(input: &[Point]) -> i64 {
    let interior = get_shape_interior(input);
    println!("Computed interior\nChecking rectangles...");

    input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            input.iter().take(i).flat_map(|b| {
                let alpha = Point::new(a.x.min(b.x), a.y.min(b.y));
                let beta = Point::new(a.x.max(b.x), a.y.max(b.y));

                (alpha.y..=beta.y)
                    .all(|y| {
                        let Some(v) = interior.get(&y) else {
                            return false;
                        };
                        let Some(i) = v.iter().position(|min| min.0 <= alpha.x) else {
                            return false;
                        };
                        let Some(j) = v[i..].iter().position(|max| max.1 >= beta.x) else {
                            return false;
                        };
                        v[i..=j].windows(2).all(|win| {
                            let (_, a) = win[0];
                            let (b, _) = win[1];
                            a + 1 == b
                        })
                    })
                    .then(|| get_area(a, b))
            })
        })
        .max()
        .unwrap_or(0)
}
