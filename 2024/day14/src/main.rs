use anyhow::{bail, Context};
use core::str;
use regex::Regex;
use std::collections::HashSet;
use std::io::{BufRead, Write};
use std::{env, fs, io, iter};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    p: Vec2,
    v: Vec2,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        bail!("usage: {} <input file> <room width> <room height>", args[0]);
    }

    let room_width: u64 = args[2]
        .parse()
        .context("room must have positive integer width")?;
    let room_height: u64 = args[3]
        .parse()
        .context("room must have positive integer height")?;
    let room_size = Vec2 {
        x: room_width as i64,
        y: room_height as i64,
    };

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    println!("Part 1: {}", part1(&input, room_size));
    println!("Part 2: {}", part2(&input, room_size));

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Robot>> {
    let robot_pattern = Regex::new(r"p=(.+),(.+) v=(.+),(.+)").expect("const regex");
    let mut robots = Vec::new();

    for line in input.lines() {
        let Some(caps) = robot_pattern.captures(line) else {
            bail!("bad line: {}", line)
        };
        let (_, [px, py, vx, vy]) = caps.extract();
        let p: Vec2 = Vec2 {
            x: px
                .parse()
                .with_context(|| format!("in line: `{}`", String::from(line)))?,
            y: py
                .parse()
                .with_context(|| format!("in line: `{}`", String::from(line)))?,
        };
        let v: Vec2 = Vec2 {
            x: vx
                .parse()
                .with_context(|| format!("in line: `{}`", String::from(line)))?,
            y: vy
                .parse()
                .with_context(|| format!("in line: `{}`", String::from(line)))?,
        };
        robots.push(Robot { p, v });
    }

    Ok(robots)
}

fn state_after(state: &Vec<Robot>, t: i64, room_size: Vec2) -> Vec<Robot> {
    state
        .iter()
        .map(|robot| Robot {
            p: Vec2 {
                x: (robot.p.x + ((robot.v.x % room_size.x) * (t % room_size.x)))
                    .rem_euclid(room_size.x),
                y: (robot.p.y + ((robot.v.y % room_size.y) * (t % room_size.y)))
                    .rem_euclid(room_size.y),
            },
            v: robot.v,
        })
        .collect()
}

fn part1(input: &Vec<Robot>, room_size: Vec2) -> i64 {
    let p1 =
        state_after(&input, 100, room_size)
            .iter()
            .fold((0i64, 0i64, 0i64, 0i64), |acc, robot| {
                if robot.p.x < room_size.x / 2 && robot.p.y < room_size.y / 2 {
                    (acc.0 + 1, acc.1, acc.2, acc.3)
                } else if robot.p.x >= (room_size.x / 2) + 1 && robot.p.y < room_size.y / 2 {
                    (acc.0, acc.1 + 1, acc.2, acc.3)
                } else if robot.p.x < room_size.x / 2 && robot.p.y >= (room_size.y / 2) + 1 {
                    (acc.0, acc.1, acc.2 + 1, acc.3)
                } else if robot.p.x >= (room_size.x / 2) + 1 && robot.p.y >= (room_size.y / 2) + 1 {
                    (acc.0, acc.1, acc.2, acc.3 + 1)
                } else {
                    acc
                }
            });
    p1.0 * p1.1 * p1.2 * p1.3
}

fn print_state(state: &Vec<Robot>, room_size: Vec2) {
    let mut v: Vec<Vec<u8>> = iter::repeat_n(
        iter::repeat_n(b'.', room_size.x as usize).collect(),
        room_size.y as usize,
    )
    .collect();
    for r in state {
        v[r.p.y as usize][r.p.x as usize] = b'#';
    }

    for l in v {
        println!("{}", str::from_utf8(&l).expect("this is utf8"));
    }
}

fn contains_doubled_robots(state: &Vec<Robot>) -> bool {
    let mut locs = HashSet::new();
    for r in state {
        if !locs.insert(r.p) {
            return true;
        }
    }

    false
}

fn part2(input: &Vec<Robot>, room_size: Vec2) -> i64 {
    let mut state = input.clone();
    let mut t = 0;

    loop {
        while contains_doubled_robots(&state) {
            state = state_after(&state, 1, room_size);
            t += 1;
        }

        println!();
        print_state(&state, room_size);
        print!("Continue (y/n): ");
        io::stdout().flush().expect("failed to flush stdout");
        let mut line = String::new();
        io::stdin()
            .lock()
            .read_line(&mut line)
            .expect("failed to read line");
        if line.trim().bytes().next().map(|ch| ch.to_ascii_lowercase()) != Some(b'y') {
            break;
        }
        state = state_after(&state, 1, room_size);
        t += 1;
    }

    t
}
