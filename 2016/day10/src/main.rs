// Fun quick thing

use std::{
    collections::{HashMap, hash_map::Entry},
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result, anyhow, bail};
use regex::Regex;

fn main() -> Result<()> {
    let input = env::args().nth(1).ok_or(anyhow!("no argument provided"))?;
    let (initial_state, instructions) = read_input(&input).context("reading input")?;

    println!("=== Part 1 ===");

    let comparison_bot = initial_state
        .find_comparison_bot(&instructions, Value(17), Value(61))
        .context("failed to find comparison bot")?
        .ok_or(anyhow!("no comparison bot found"))?;
    println!("Bot {} compares 17 and 61", comparison_bot.0);
    println!();

    println!("=== Part 2 ===");
    let outputs = initial_state
        .get_final_outputs(&instructions)
        .context("failed to run to completion")?;
    let value_0 = outputs
        .get(&Output(0))
        .ok_or(anyhow!("output 0 doesn't exist"))?
        .first()
        .ok_or(anyhow!("output 0 is empty"))?;
    let value_1 = outputs
        .get(&Output(1))
        .ok_or(anyhow!("output 1 doesn't exist"))?
        .first()
        .ok_or(anyhow!("output 1 is empty"))?;
    let value_2 = outputs
        .get(&Output(2))
        .ok_or(anyhow!("output 2 doesn't exist"))?
        .first()
        .ok_or(anyhow!("output 2 is empty"))?;
    let product = value_0.0 * value_1.0 * value_2.0;
    println!("Product: {}", product);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BotId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Value(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Output(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Bot(BotId),
    Output(Output),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bot {
    Single(Value),
    Pair { low: Value, high: Value },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    from: BotId,
    low: Target,
    high: Target,
}

#[derive(Debug, Default, Clone)]
struct State {
    bots: HashMap<BotId, Bot>,
    outputs: HashMap<Output, Vec<Value>>,
}

impl State {
    fn step(&self, instructions: &[Instruction]) -> Result<Option<(Self, BotId)>> {
        for instruction in instructions {
            if let Some(Bot::Pair { low, high }) = self.bots.get(&instruction.from) {
                let mut new_state = self.clone();

                new_state.bots.remove(&instruction.from);

                match instruction.low {
                    Target::Bot(bot_id) => match new_state.bots.entry(bot_id) {
                        Entry::Occupied(mut entry) => match entry.get() {
                            Bot::Single(value) => {
                                entry.insert(Bot::Pair {
                                    low: (*value).min(*low),
                                    high: (*value).max(*low),
                                });
                            }
                            Bot::Pair { .. } => {
                                bail!("cannot send low to bot {}", bot_id.0);
                            }
                        },
                        Entry::Vacant(entry) => {
                            entry.insert(Bot::Single(*low));
                        }
                    },
                    Target::Output(output) => {
                        new_state
                            .outputs
                            .entry(output)
                            .and_modify(|output| output.push(*low))
                            .or_insert_with(|| vec![*low]);
                    }
                }

                match instruction.high {
                    Target::Bot(bot_id) => match new_state.bots.entry(bot_id) {
                        Entry::Occupied(mut entry) => match entry.get() {
                            Bot::Single(value) => {
                                entry.insert(Bot::Pair {
                                    low: (*value).min(*high),
                                    high: (*value).max(*high),
                                });
                            }
                            Bot::Pair { .. } => {
                                bail!("cannot send low to bot {}", bot_id.0);
                            }
                        },
                        Entry::Vacant(entry) => {
                            entry.insert(Bot::Single(*high));
                        }
                    },
                    Target::Output(output) => {
                        new_state
                            .outputs
                            .entry(output)
                            .and_modify(|output| output.push(*high))
                            .or_insert_with(|| vec![*high]);
                    }
                }

                return Ok(Some((new_state, instruction.from)));
            }
        }

        Ok(None)
    }

    fn get_final_outputs(
        &self,
        instructions: &[Instruction],
    ) -> Result<HashMap<Output, Vec<Value>>> {
        let mut state = self.clone();

        while !state.bots.is_empty() {
            state = match state.step(instructions)? {
                Some((v, _)) => v,
                None => {
                    return Ok(state.outputs);
                }
            }
        }

        Ok(state.outputs)
    }

    fn find_comparison_bot(
        &self,
        instructions: &[Instruction],
        low: Value,
        high: Value,
    ) -> Result<Option<BotId>> {
        let mut state = self.clone();

        while !state.bots.is_empty() {
            state = match state.step(instructions)? {
                Some((v, bot)) => {
                    if let Some(Bot::Pair {
                        low: low_val,
                        high: high_val,
                    }) = state.bots.get(&bot)
                        && *low_val == low
                        && *high_val == high
                    {
                        return Ok(Some(bot));
                    }
                    v
                }
                None => {
                    return Ok(None);
                }
            };
        }

        Ok(None)
    }
}

fn parse_input(input: impl BufRead) -> Result<(State, Vec<Instruction>)> {
    let initial = Regex::new("^value ([0-9]+) goes to bot ([0-9]+)$").unwrap();
    let instruction = Regex::new(
        "^bot ([0-9]+) gives low to (bot|output) ([0-9]+) and high to (bot|output) ([0-9]+)$",
    )
    .unwrap();

    let mut initial_state = State::default();
    let mut instructions = Vec::new();

    for line in input.lines() {
        let line = line?;
        let line = line.trim();

        if let Some((_, [value, bot_id])) = initial.captures(line).map(|cap| cap.extract()) {
            let value = Value(
                value
                    .parse()
                    .with_context(|| format!("parsing value in line `{}`", &line))?,
            );
            let bot_id = BotId(
                bot_id
                    .parse()
                    .with_context(|| format!("parsing bot in line `{}`", &line))?,
            );

            match initial_state.bots.entry(bot_id) {
                Entry::Occupied(mut entry) => match entry.get() {
                    Bot::Single(v) => {
                        entry.insert(Bot::Pair {
                            low: value.min(*v),
                            high: value.max(*v),
                        });
                    }
                    Bot::Pair { .. } => {
                        bail!("bot {} already occupied, cannot give third value", bot_id.0);
                    }
                },
                Entry::Vacant(entry) => {
                    entry.insert(Bot::Single(value));
                }
            };
        } else if let Some((_, [bot_id, low_type, low_id, high_type, high_id])) =
            instruction.captures(line).map(|cap| cap.extract())
        {
            let bot_id = BotId(
                bot_id
                    .parse()
                    .with_context(|| format!("parsing bot in line `{}`", line))?,
            );
            let low = match low_type {
                "bot" => Target::Bot(BotId(
                    low_id
                        .parse()
                        .with_context(|| format!("parsing low bot in line `{}`", line))?,
                )),
                "output" => Target::Output(Output(
                    low_id
                        .parse()
                        .with_context(|| format!("parsing low output in line `{}`", line))?,
                )),
                _ => unreachable!(),
            };
            let high = match high_type {
                "bot" => Target::Bot(BotId(
                    high_id
                        .parse()
                        .with_context(|| format!("parsing high bot in line `{}`", line))?,
                )),
                "output" => Target::Output(Output(
                    high_id
                        .parse()
                        .with_context(|| format!("parsing high output in line `{}`", line))?,
                )),
                _ => unreachable!(),
            };

            instructions.push(Instruction {
                from: bot_id,
                low,
                high,
            });
        } else {
            bail!("bad line `{}`", line);
        }
    }

    Ok((initial_state, instructions))
}

fn read_input(path: impl AsRef<Path>) -> Result<(State, Vec<Instruction>)> {
    let input = BufReader::new(
        File::open(&path)
            .with_context(|| format!("opening `{}` for reading", &path.as_ref().display()))?,
    );
    parse_input(input)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    const SAMPLE_INPUT: &str = "\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn sample_input() {
        let (initial_state, instructions) =
            parse_input(Cursor::new(SAMPLE_INPUT)).expect("failed to parse input");
        let comparison_bot = initial_state
            .find_comparison_bot(&instructions, Value(2), Value(5))
            .expect("failed to find comparison bot");
        assert_eq!(comparison_bot, Some(BotId(2)));
    }
}
