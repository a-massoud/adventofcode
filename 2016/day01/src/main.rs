// Nice and simple

use std::{collections::HashSet, env, fs, path::Path};

use anyhow::{Context, anyhow};

fn main() -> anyhow::Result<()> {
    let input = env::args().nth(1).ok_or(anyhow!("No argument provided"))?;
    let instructions = read_input(input)?;

    let final_pos = follow_directions(&instructions);
    println!("Part 1: {}", final_pos.norm());

    let first_twice_pos = first_twice(&instructions).ok_or(anyhow!("No position visited twice"))?;
    println!("Part 2: {}", first_twice_pos.norm());

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn(&self, turn: Turn) -> Self {
        match (self, turn) {
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::North, Turn::Right) => Direction::East,
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::East, Turn::Left) => Direction::North,
            (Direction::East, Turn::Right) => Direction::South,
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::West, Turn::Right) => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    pub turn: Turn,
    pub dist: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn norm(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<Instruction>> {
    let input = fs::read_to_string(&path)
        .with_context(|| format!("Could not open `{:?}` for reading", path.as_ref()))?;

    input
        .split(',')
        .map(|i| {
            let i = i.trim().as_bytes();
            if i.len() < 2 {
                return Err(anyhow!("Instruction {} too short", str::from_utf8(i)?));
            }
            match i[0] {
                b'L' => Ok(Instruction {
                    turn: Turn::Left,
                    dist: str::from_utf8(&i[1..])?.parse()?,
                }),
                b'R' => Ok(Instruction {
                    turn: Turn::Right,
                    dist: str::from_utf8(&i[1..])?.parse()?,
                }),
                _ => Err(anyhow!("Invalid instruction {}", str::from_utf8(i)?)),
            }
        })
        .collect()
}

fn follow_directions(input: &[Instruction]) -> Point {
    let mut pos = Point { x: 0, y: 0 };
    let mut facing = Direction::North;

    for i in input {
        facing = facing.turn(i.turn);
        match facing {
            Direction::North => pos.y += i.dist,
            Direction::South => pos.y -= i.dist,
            Direction::East => pos.x += i.dist,
            Direction::West => pos.x -= i.dist,
        }
    }

    pos
}

fn first_twice(input: &[Instruction]) -> Option<Point> {
    let mut pos = Point { x: 0, y: 0 };
    let mut visited = HashSet::from([pos]);
    let mut facing = Direction::North;

    for i in input {
        facing = facing.turn(i.turn);

        for _ in 1..=i.dist {
            match facing {
                Direction::North => pos.y += 1,
                Direction::South => pos.y -= 1,
                Direction::East => pos.x += 1,
                Direction::West => pos.x -= 1,
            }

            if visited.contains(&pos) {
                return Some(pos);
            }

            visited.insert(pos);
        }
    }

    None
}
