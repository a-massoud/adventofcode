// I was genuinely about to implement a diophantine equation solver by hand before realizing that
// z3 had bindings for rust. Thank god for that.

use anyhow::{anyhow, bail, Context};
use std::collections::{HashSet, VecDeque};
use std::{env, fs, iter};
use z3::ast::Int;
use z3::Optimize;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let input = parse_input(&input)?;

    let p1 = input
        .iter()
        .map(|machine| {
            get_min_buttons_indicator(machine)
                .ok_or(anyhow!("no route found for machine {:?}", machine))
        })
        .try_fold(0, |acc, x| x.map(|v| acc + v))?;
    println!("Part 1: {}", p1);

    let p2 = input
        .iter()
        .map(| machine| {
            get_min_buttons_joltage(machine)
                .ok_or(anyhow!("no route found for machine {:?}", machine))
        })
        .try_fold(0, |acc, x| x.map(|v| acc + v))?;
    println!("Part 2: {}", p2);

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct Machine {
    pub target: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub joltage: Vec<u64>,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Machine>> {
    input
        .lines()
        .map(|line| {
            let sections: Vec<_> = line.split_whitespace().collect();
            if sections.len() < 3 {
                bail!("line `{}` invalid", line);
            }
            let target = sections[0];
            let buttons = &sections[1..(sections.len() - 1)];
            let joltage = sections[sections.len() - 1];

            if target.len() < 2
                || target.as_bytes()[0] != b'['
                || target.as_bytes()[target.len() - 1] != b']'
            {
                bail!("line `{}` has invalid indicator", line);
            }

            let target: Vec<_> = target[1..(target.len() - 1)]
                .bytes()
                .map(|b| match b {
                    b'#' => Ok(true),
                    b'.' => Ok(false),
                    _ => Err(anyhow!("line `{}` has invalid indicator", line)),
                })
                .collect::<Result<_, _>>()?;

            let buttons: Vec<Vec<_>> = buttons
                .into_iter()
                .map(|button| {
                    if button.len() < 2
                        || button.as_bytes()[0] != b'('
                        || button.as_bytes()[button.len() - 1] != b')'
                    {
                        bail!("invalid button `{}`", button);
                    }

                    button[1..button.len() - 1]
                        .split(',')
                        .map(|v| v.parse::<usize>())
                        .collect::<Result<_, _>>()
                        .map_err(Into::into)
                })
                .collect::<Result<_, _>>()
                .with_context(|| format!("in button of line `{}`", line))?;
            if buttons.iter().flat_map(|i| i).any(|&i| i >= target.len()) {
                bail!("line `{}` has button with too high value", line);
            }

            if joltage.len() < 2
                || joltage.as_bytes()[0] != b'{'
                || joltage.as_bytes()[joltage.len() - 1] != b'}'
            {
                bail!("line `{}` has invalid joltage", line);
            }

            let joltage: Vec<_> = joltage[1..joltage.len() - 1]
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()?;

            if joltage.len() != target.len() {
                bail!(
                    "line `{}` has different number of joltages as indicators",
                    line
                );
            }

            Ok(Machine {
                target,
                buttons,
                joltage,
            })
        })
        .collect()
}

fn get_min_buttons_indicator(machine: &Machine) -> Option<usize> {
    let mut q = VecDeque::new();
    q.push_back((
        iter::repeat_n(false, machine.target.len()).collect::<Vec<_>>(),
        0,
    ));
    let mut visited = HashSet::new();

    while let Some((indicator, buttons_pressed)) = q.pop_front() {
        if indicator == machine.target {
            return Some(buttons_pressed);
        }

        if !visited.insert(indicator.clone()) {
            continue;
        }

        for button in &machine.buttons {
            let mut new_indicator = indicator.clone();
            for &j in button {
                new_indicator[j] = !new_indicator[j];
            }
            q.push_back((new_indicator, buttons_pressed + 1))
        }
    }

    None
}

fn get_min_buttons_joltage(machine: &Machine) -> Option<u64> {
    let buttons: Vec<_> = (0..machine.buttons.len())
        .map(|i| Int::fresh_const(&format!("b{}", i)))
        .collect();
    let solver = Optimize::new();

    for i in 0..machine.joltage.len() {
        solver.assert(
            &(buttons
                .iter()
                .enumerate()
                .flat_map(|(j, b)| {
                    if machine.buttons[j].contains(&i) {
                        Some(b)
                    } else {
                        None
                    }
                })
                .fold(Int::from_i64(0), |a, b| a + b)
                .eq(machine.joltage[i])),
        );
    }

    for button in &buttons {
        solver.assert(&button.ge(0));
    }

    solver.minimize(&buttons.iter().sum::<Int>());

    solver.check(&[]);

    solver
        .get_model()
        .and_then(|model| model.eval(&buttons.iter().sum::<Int>(), false))
        .and_then(|i| i.as_u64())
}
