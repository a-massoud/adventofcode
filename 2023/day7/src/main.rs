use core::str;
use std::{
    cmp::{Ordering, Reverse},
    env, fs,
};

use anyhow::{anyhow, bail, Context};
use regex::Regex;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return;
    }

    let Ok(input) = fs::read_to_string(&args[1]) else {
        eprintln!("Failed to read from input file `{}`.", args[1]);
        return;
    };

    let input = match parse_input(&input) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to parse input: {}.", e);
            return;
        }
    };

    println!("Part 1: {}", calculate_winnings(&input, false));
    println!("Part 2: {}", calculate_winnings(&input, true));
}

fn parse_input(input: &str) -> anyhow::Result<Vec<([u8; 5], i64)>> {
    let line_pattern = Regex::new(r"^([AKQJT2-9]{5}) (\d+)$").expect("Constant regex");
    let mut data = Vec::new();

    for line in input.lines() {
        let (_, [card, bet]) = line_pattern
            .captures(line)
            .ok_or(anyhow!("bad line `{}`", line))?
            .extract();

        let bet: i64 = bet.parse().with_context(|| format!("bad bet `{}`", bet))?;

        data.push((card.bytes().collect::<Vec<_>>().try_into().unwrap(), bet));
    }

    Ok(data)
}

fn calculate_winnings(input: &[([u8; 5], i64)], wild_jokers: bool) -> i64 {
    let mut input = Vec::from(input);

    input.sort_unstable_by(|(a, _), (b, _)| compare_cards(a, b, wild_jokers).unwrap());

    input
        .iter()
        .enumerate()
        .fold(0i64, |acc, (i, (_, bet))| acc + bet * (i as i64 + 1))
}

fn compare_cards(a: &[u8], b: &[u8], wild_jokers: bool) -> anyhow::Result<Ordering> {
    let high_cards = if !wild_jokers {
        [
            b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
        ]
    } else {
        [
            b'J', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A',
        ]
    };

    let a_type;
    let b_type;
    if wild_jokers {
        let mut err = Ok(());
        a_type = high_cards
            .iter()
            .map(|ch| {
                get_card_type(
                    &a.iter()
                        .cloned()
                        .map(|x| if x == b'J' { *ch } else { x })
                        .collect::<Vec<_>>(),
                )
            })
            .scan((), |_, x| x.map_err(|e| err = Err(e)).ok())
            .max()
            .unwrap_or(HandType::High);
        err?;
        let mut err = Ok(());
        b_type = high_cards
            .iter()
            .map(|ch| {
                get_card_type(
                    &b.iter()
                        .cloned()
                        .map(|x| if x == b'J' { *ch } else { x })
                        .collect::<Vec<_>>(),
                )
            })
            .scan((), |_, x| x.map_err(|e| err = Err(e)).ok())
            .max()
            .unwrap_or(HandType::High);
        err?;
    } else {
        a_type = get_card_type(a)?;
        b_type = get_card_type(b)?;
    }

    if a_type != b_type {
        return Ok(a_type.cmp(&b_type));
    }

    let Some((a, b)) = a
        .iter()
        .cloned()
        .zip(b.iter().cloned())
        .find(|(x, y)| x != y)
    else {
        return Ok(Ordering::Equal);
    };
    let a_score = high_cards
        .iter()
        .position(|x| *x == a)
        .ok_or(anyhow!("bad card"))?;
    let b_score = high_cards
        .iter()
        .position(|x| *x == b)
        .ok_or(anyhow!("bad card"))?;

    Ok(a_score.cmp(&b_score))
}

fn get_card_type(c: &[u8]) -> anyhow::Result<HandType> {
    let mut scores = [0i8; 13];
    for ch in c {
        match ch {
            b'2'..=b'9' => scores[(ch - b'2') as usize] += 1,
            b'A' => scores[8] += 1,
            b'K' => scores[9] += 1,
            b'Q' => scores[10] += 1,
            b'J' => scores[11] += 1,
            b'T' => scores[12] += 1,
            _ => bail!("bad card"),
        }
    }

    scores.sort_unstable_by_key(|x| Reverse(*x));

    Ok(match scores[0..2] {
        [5, _] => HandType::FiveKind,
        [4, _] => HandType::FourKind,
        [3, 2] => HandType::FullHouse,
        [3, _] => HandType::ThreeKind,
        [2, 2] => HandType::TwoPair,
        [2, _] => HandType::OnePair,
        _ => HandType::High,
    })
}
