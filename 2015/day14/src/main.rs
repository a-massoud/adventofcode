use anyhow::anyhow;
use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    let input = read_input(&env::args().nth(1).ok_or(anyhow!("no argument"))?)?;

    let (names, dist) = winners_at(&input, 2503).ok_or(anyhow!("no winner"))?;
    println!("{} wins at {} km.", names[0], dist);

    let (name, points) = points_winner_at(&input, 2503).ok_or(anyhow!("no winner"))?;
    println!("{} wins at {} points.", name, points);

    Ok(())
}

#[derive(Debug, Default, Clone, Copy)]
struct Reindeer {
    pub speed: i32,
    pub run_time: i32,
    pub rest_time: i32,
}

fn read_input(path: &impl AsRef<Path>) -> anyhow::Result<HashMap<String, Reindeer>> {
    let re = Regex::new(
        r"^(\S+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$",
    )
    .unwrap();

    let reader = BufReader::new(File::open(path)?);
    reader
        .lines()
        .map(|line| {
            let line = line?;

            let (_, [name, speed, run_time, rest_time]) = re
                .captures(&line)
                .ok_or(anyhow!("bad line `{}`", line))?
                .extract();

            Ok((
                name.to_owned(),
                Reindeer {
                    speed: speed.parse()?,
                    run_time: run_time.parse()?,
                    rest_time: rest_time.parse()?,
                },
            ))
        })
        .collect()
}

fn winners_at(reindeer: &HashMap<String, Reindeer>, time: i32) -> Option<(Vec<&str>, i32)> {
    reindeer
        .iter()
        .map(|(name, stats)| {
            let n_cycles = time / (stats.rest_time + stats.run_time);
            let c_cycle = time % (stats.rest_time + stats.run_time);
            let d_per_run = stats.run_time * stats.speed;

            if c_cycle > stats.run_time {
                // resting
                return (name.as_str(), d_per_run * (n_cycles + 1));
            } else {
                // running
                return (name.as_str(), d_per_run * n_cycles + stats.speed * c_cycle);
            }
        })
        .fold(None, |max, (name, dist)| match max {
            Some((names, v)) => {
                if dist > v {
                    Some((vec![name], dist))
                } else if dist == v {
                    Some((names.into_iter().chain([name].into_iter()).collect(), dist))
                } else {
                    Some((names, v))
                }
            }
            None => Some((vec![name], dist)),
        })
}

fn points_winner_at(reindeer: &HashMap<String, Reindeer>, time: i32) -> Option<(&str, i32)> {
    let mut points: HashMap<&str, i32> = reindeer.keys().map(|k| (k.as_str(), 0)).collect();

    for t in 1..=time {
        if let Some((names, _)) = winners_at(reindeer, t) {
            for name in names {
                points.get_mut(name).map(|score| *score += 1);
            }
        }
    }

    points
        .into_iter()
        .fold(None, |max, (name, points)| match max {
            Some((_, v, _)) => {
                if points > v {
                    Some((name, points, true))
                } else if points == v {
                    Some((name, points, false))
                } else {
                    max
                }
            }
            None => Some((name, points, true)),
        })
        .and_then(|(name, points, unique)| unique.then(|| (name, points)))
}
