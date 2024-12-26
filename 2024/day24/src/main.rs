use anyhow::{bail, Context};
use regex::Regex;
use std::{cmp::Reverse, collections::HashMap, env, fs, sync::LazyLock};

#[derive(Debug, Clone)]
enum Wire {
    Const(bool),
    And(String, String),
    Xor(String, String),
    Or(String, String),
}

impl Wire {
    fn evaluate(&self, wires: &HashMap<String, Wire>) -> bool {
        match self {
            Wire::Const(v) => *v,
            Wire::And(a, b) => {
                wires.get(a).map(|x| x.evaluate(wires)).unwrap_or(false)
                    && wires.get(b).map(|x| x.evaluate(wires)).unwrap_or(false)
            }
            Wire::Xor(a, b) => {
                wires.get(a).map(|x| x.evaluate(wires)).unwrap_or(false)
                    != wires.get(b).map(|x| x.evaluate(wires)).unwrap_or(false)
            }
            Wire::Or(a, b) => {
                wires.get(a).map(|x| x.evaluate(wires)).unwrap_or(false)
                    || wires.get(b).map(|x| x.evaluate(wires)).unwrap_or(false)
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("input file required");
    }

    let input = fs::read_to_string(&args[1]).context("failed to read file")?;
    let input = parse_input(&input).context("failed to parse input")?;

    println!("Part 1: {}", eval_zs(&input));

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<HashMap<String, Wire>> {
    static WIRE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(.{3}) (AND|XOR|OR) (.{3}) -> (.{3})$").unwrap());
    let _ = &*WIRE_RE;

    let mut wires = HashMap::new();

    let mut lines = input.lines();
    let mut line = lines.next();
    while line.is_some() && line != Some("") {
        let Some((name, value)) = line.unwrap().split_once(": ") else {
            bail!("invalid line: `{}`", line.unwrap());
        };

        if name.len() != 3 {
            bail!("name too long: `{}`", name);
        }

        let value = match value {
            "1" => true,
            "0" => false,
            _ => bail!("invalid value: `{}`", value),
        };

        wires.insert(String::from(name), Wire::Const(value));
        line = lines.next();
    }

    for line in lines {
        let Some(c) = WIRE_RE.captures(line) else {
            bail!("invalid line: `{}`", line);
        };

        let (_, [a, t, b, r]) = c.extract();

        wires.insert(
            String::from(r),
            match t {
                "AND" => Wire::And(String::from(a), String::from(b)),
                "XOR" => Wire::Xor(String::from(a), String::from(b)),
                "OR" => Wire::Or(String::from(a), String::from(b)),
                _ => bail!("invalid operator type: `{}`", t),
            },
        );
    }

    Ok(wires)
}

fn eval_zs(wires: &HashMap<String, Wire>) -> u64 {
    let mut v: Vec<_> = wires
        .iter()
        .filter(|(n, _)| n.bytes().next() == Some(b'z'))
        .map(|(n, v)| (n.to_owned(), v.evaluate(wires)))
        .collect();
    v.sort_by_key(|x| Reverse(x.0.clone()));

    v.iter()
        .fold(0u64, |acc, &(_, v)| (acc << 1) + if v { 1 } else { 0 })
}
