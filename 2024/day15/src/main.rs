// A bit of jank but it works!

use anyhow::{anyhow, bail};
use core::str;
use std::collections::HashSet;
use std::ops::{Add, Mul, Sub};
use std::{env, fs};

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0])
    }

    let input = fs::read_to_string(&args[1])?;
    let (robot, walls, boxes, script) = parse_input(&input)?;

    println!(
        "Part 1: {}",
        simulate(robot, &walls, &boxes, &script)
            .1
            .iter()
            .fold(0i64, |acc, b| acc + b.y * 100 + b.x)
    );
    println!(
        "Part 2: {}",
        simulate_doubled(robot, &walls, &boxes, &script)
            .1
            .iter()
            .fold(0i64, |acc, b| acc + b.y * 100 + b.x)
    );

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<(Vec2, HashSet<Vec2>, HashSet<Vec2>, Vec<Dir>)> {
    let mut robot = Vec2::default();
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();

    let (map, movements) = input
        .split_once("\n\n")
        .ok_or(anyhow!("input not divided in two"))?;

    let line_length = map.lines().next().ok_or(anyhow!("empty map"))?.len();
    for (y, line) in map.lines().enumerate() {
        if line.len() != line_length {
            bail!("inconsistent map line lengths")
        }

        for (x, ch) in line.bytes().enumerate() {
            match ch {
                b'#' => {
                    walls.insert(Vec2 {
                        x: x as i64,
                        y: y as i64,
                    });
                }
                b'O' => {
                    boxes.insert(Vec2 {
                        x: x as i64,
                        y: y as i64,
                    });
                }
                b'@' => {
                    robot = Vec2 {
                        x: x as i64,
                        y: y as i64,
                    }
                }
                b'.' => (),
                _ => bail!("unexpected character in map"),
            }
        }
    }

    let script: Vec<Dir> = movements
        .bytes()
        .filter_map(|ch| match ch {
            b'^' => Some(Dir::Up),
            b'>' => Some(Dir::Right),
            b'v' => Some(Dir::Down),
            b'<' => Some(Dir::Left),
            _ => None,
        })
        .collect();

    Ok((robot, walls, boxes, script))
}

fn simulate(
    robot: Vec2,
    walls: &HashSet<Vec2>,
    boxes: &HashSet<Vec2>,
    script: &Vec<Dir>,
) -> (Vec2, HashSet<Vec2>) {
    let mut robot = robot;
    let mut boxes = boxes.clone();

    for dir in script {
        let dir = match dir {
            Dir::Up => Vec2 { x: 0, y: -1 },
            Dir::Down => Vec2 { x: 0, y: 1 },
            Dir::Left => Vec2 { x: -1, y: 0 },
            Dir::Right => Vec2 { x: 1, y: 0 },
        };

        let mut n = robot + dir;
        while boxes.contains(&n) {
            n = n + dir;
        }

        if walls.contains(&n) {
            continue;
        }

        if n == robot + dir {
            robot = n;
        } else {
            boxes.remove(&(robot + dir));
            boxes.insert(n);
            robot = robot + dir;
        }
    }

    (robot, boxes)
}

fn simulate_doubled(
    robot: Vec2,
    walls: &HashSet<Vec2>,
    boxes: &HashSet<Vec2>,
    script: &Vec<Dir>,
) -> (Vec2, HashSet<Vec2>) {
    let mut robot = Vec2 {
        x: robot.x * 2,
        y: robot.y,
    };
    let walls: HashSet<Vec2> = walls
        .iter()
        .flat_map(|w| {
            [
                Vec2 { x: w.x * 2, y: w.y },
                Vec2 {
                    x: w.x * 2 + 1,
                    y: w.y,
                },
            ]
        })
        .collect();
    let mut boxes: HashSet<Vec2> = boxes.iter().map(|b| Vec2 { x: b.x * 2, y: b.y }).collect();

    for dir in script {
        let dir = match dir {
            Dir::Up => Vec2 { x: 0, y: -1 },
            Dir::Down => Vec2 { x: 0, y: 1 },
            Dir::Left => Vec2 { x: -1, y: 0 },
            Dir::Right => Vec2 { x: 1, y: 0 },
        };

        let mut wall = false;
        let mut to_move = HashSet::new();
        let mut frontier = vec![robot + dir];
        while !frontier.is_empty() {
            let pt = frontier.pop().expect("nonempty queue has no elements");
            if walls.contains(&pt) {
                wall = true;
                break;
            }

            if boxes.contains(&pt) {
                to_move.insert(pt);
                frontier.push(pt + dir);
                if dir.y != 0 {
                    frontier.push(pt + dir + Vec2 { x: 1, y: 0 });
                }
            }
            let opt = pt - Vec2 { x: 1, y: 0 };
            if boxes.contains(&opt) {
                to_move.insert(opt);
                // this is jank
                if dir.x == 1 {
                    frontier.push(opt + dir + Vec2 { x: 1, y: 0 });
                } else {
                    frontier.push(opt + dir);
                }
                if dir.y != 0 {
                    frontier.push(opt + dir + Vec2 { x: 1, y: 0 });
                }
            }
        }

        if wall {
            continue;
        }

        for b in &to_move {
            boxes.remove(&b);
        }
        for b in &to_move {
            boxes.insert(*b + dir);
        }
        robot = robot + dir;
    }

    (robot, boxes)
}
