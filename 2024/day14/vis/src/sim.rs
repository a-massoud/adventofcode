use std::iter;

use anyhow::{bail, Context};
use nalgebra::{vector, Vector2, Vector3};
use rand::Rng;
use regex::Regex;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot {
    pub p: Vector2<i64>,
    pub v: Vector2<i64>,
    pub color: Vector3<u8>,
}

impl Robot {
    pub fn new(p: Vector2<i64>, v: Vector2<i64>, color: Vector3<u8>) -> Self {
        Self { p, v, color }
    }

    pub fn step(&self, t: i64, room_size: Vector2<i64>) -> Self {
        Self::new(
            vector![
                (self.p.x + (self.v.x % room_size.x) * (t % room_size.x)).rem_euclid(room_size.x),
                (self.p.y + (self.v.y % room_size.y) * (t % room_size.y)).rem_euclid(room_size.y)
            ],
            self.v,
            self.color,
        )
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Robot>> {
    let robot_pattern = Regex::new(r"p=(.+),(.+) v=(.+),(.+)").expect("const regex");
    let mut rng = rand::thread_rng();
    let mut robots = Vec::new();

    for line in input.lines() {
        let Some(caps) = robot_pattern.captures(line) else {
            bail!("bad line: {}", line)
        };
        let (_, [px, py, vx, vy]) = caps.extract();
        let p = vector![
            px.parse()
                .with_context(|| format!("in line: `{}`", String::from(line)))?,
            py.parse()
                .with_context(|| format!("in line: `{}`", String::from(line)))?,
        ];
        let v = vector![
            vx.parse()
                .with_context(|| format!("in line: `{}`", String::from(line)))?,
            vy.parse()
                .with_context(|| format!("in line: `{}`", String::from(line)))?,
        ];
        robots.push(Robot::new(p, v, vector![rng.gen(), rng.gen(), rng.gen()]));
    }

    Ok(robots)
}

pub fn step_state(state: &[Robot], t: i64, room_size: Vector2<i64>) -> Vec<Robot> {
    state.iter().map(|robot| robot.step(t, room_size)).collect()
}

pub fn get_all_states(input: &[Robot], room_size: Vector2<i64>) -> Vec<Vec<Robot>> {
    iter::repeat_n(input, (room_size.x * room_size.y) as usize)
        .enumerate()
        .map(|(i, state)| step_state(state, i as i64, room_size))
        .collect()
}
