use anyhow::{bail, Context};
use std::{env, fs};

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0])
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    println!(
        "Part 1: {}",
        input
            .iter()
            .filter_map(|machine| get_presses(machine))
            .map(|(a, b)| 3 * a + b)
            .sum::<i64>()
    );
    println!(
        "Part 2: {}",
        input
            .clone()
            .iter_mut()
            .filter_map(|machine| {
                machine.prize.x += 10000000000000;
                machine.prize.y += 10000000000000;
                get_presses(machine)
            })
            .map(|(a, b)| 3 * a + b)
            .sum::<i64>()
    );

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<ClawMachine>> {
    let mut res = Vec::new();

    for machineblock in input.split("\n\n") {
        let lines: Vec<&str> = machineblock.lines().collect();
        if lines.len() != 3 {
            bail!("block too large: `{}`", machineblock);
        }

        let lines: Vec<(&str, &str)> = lines
            .iter()
            .map(|l| {
                let l = l
                    .split_once(':')
                    .unwrap_or(("", ""))
                    .1
                    .split_once(", ")
                    .unwrap_or(("", ""));
                if l.0.len() < 3 || l.1.len() < 3 {
                    bail!("bad line")
                }
                Ok((&l.0.trim()[2..], &l.1.trim()[2..]))
            })
            .collect::<Result<_, _>>()
            .with_context(|| format!("in block: `{}`", machineblock))?;

        res.push(
            (move || -> anyhow::Result<ClawMachine> {
                let a = Vec2 {
                    x: lines[0].0.parse()?,
                    y: lines[0].1.parse()?,
                };
                let b = Vec2 {
                    x: lines[1].0.parse()?,
                    y: lines[1].1.parse()?,
                };
                let prize = Vec2 {
                    x: lines[2].0.parse()?,
                    y: lines[2].1.parse()?,
                };
                Ok(ClawMachine { a, b, prize })
            })()
            .with_context(|| format!("in block: `{}`", machineblock))?,
        );
    }

    Ok(res)
}

fn get_presses(machine: &ClawMachine) -> Option<(i64, i64)> {
    let a = (machine.prize.x * machine.b.y - machine.prize.y * machine.b.x)
        / (machine.a.x * machine.b.y - machine.a.y * machine.b.x);
    let b = (-machine.prize.x * machine.a.y + machine.prize.y * machine.a.x)
        / (machine.a.x * machine.b.y - machine.a.y * machine.b.x);

    if machine.a.x * a + machine.b.x * b == machine.prize.x
        && machine.a.y * a + machine.b.y * b == machine.prize.y
    {
        Some((a, b))
    } else {
        None
    }
}
