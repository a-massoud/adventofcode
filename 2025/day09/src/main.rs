use anyhow::{Context, anyhow};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let input = parse_input(&input)?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    Ok(())
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
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

fn part1(input: &[Point]) -> i64 {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| input.iter().take(i).map(|b| get_area(a, b)))
        .max()
        .unwrap_or(0)
}
