use std::{collections::HashMap, io::BufRead};

use anyhow::{Context, Result, anyhow, bail};
use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, value},
    multi::many0,
    sequence::{preceded, terminated},
};

use crate::solver::{Floor, Generator, Item, Microchip};

enum NamedItem {
    Microchip(String),
    Generator(String),
}

fn parse_ordinal(input: &str) -> IResult<&str, u8> {
    alt((
        value(0, tag("first")),
        value(1, tag("second")),
        value(2, tag("third")),
        value(3, tag("fourth")),
    ))
    .parse(input)
}

fn parse_item(input: &str) -> IResult<&str, NamedItem> {
    let (input, _) = alt((tag("a "), tag("an "))).parse(input)?;
    let (input, element) = take_while1(|c: char| c.is_lowercase()).parse(input)?;
    alt((
        tag(" generator").map(|_| NamedItem::Generator(element.to_owned())),
        tag("-compatible microchip").map(|_| NamedItem::Microchip(element.to_owned())),
    ))
    .parse(input)
}

fn parse_item_list(input: &str) -> IResult<&str, Vec<NamedItem>> {
    let (input, first) = parse_item(input)?;

    if let Ok((input, second)) = preceded(tag(" and "), parse_item).parse(input) {
        return Ok((input, vec![first, second]));
    }

    let (input, mut items) = many0(preceded(tag(", "), parse_item)).parse(input)?;
    if items.is_empty() {
        return Ok((input, vec![first]));
    }
    let (input, last) = preceded(tag(", and "), parse_item).parse(input)?;

    items.reserve_exact(2);
    items.insert(0, first);
    items.push(last);

    Ok((input, items))
}

fn parse_line(input: &str) -> IResult<&str, (u8, Vec<NamedItem>)> {
    let (input, _) = tag("The ")(input)?;
    let (input, idx) = parse_ordinal(input)?;
    let (input, _) = tag(" floor contains ")(input)?;

    let (input, items) = all_consuming(alt((
        tag("nothing relevant.").map(|_| Vec::new()),
        terminated(parse_item_list, tag(".")),
    )))
    .parse(input)?;

    Ok((input, (idx, items)))
}

pub fn parse_input(input: impl BufRead) -> Result<([Floor; 4], HashMap<String, u32>)> {
    let mut floors = [const { None }; 4];
    let mut ids = HashMap::new();

    for (no, line) in input.lines().enumerate() {
        let line = line?;
        let (_, (idx, named_items)) =
            parse_line(&line).map_err(|e| anyhow!("parse error in line {}: {:?}", no, e))?;
        if floors[idx as usize].is_some() {
            bail!("floor {} specified twice", idx);
        }

        let mut items = Vec::new();
        for item in named_items {
            match item {
                NamedItem::Microchip(name) => {
                    if !ids.contains_key(&name) {
                        ids.insert(name.clone(), ids.len() as u32);
                    }
                    items.push(Item::Microchip(Microchip {
                        element: ids[&name],
                    }));
                }
                NamedItem::Generator(name) => {
                    if !ids.contains_key(&name) {
                        ids.insert(name.clone(), ids.len() as u32);
                    }
                    items.push(Item::Generator(Generator {
                        element: ids[&name],
                    }));
                }
            }
        }

        floors[idx as usize] =
            Some(Floor::new(items).with_context(|| format!("on floor {}", idx))?);
    }

    floors
        .into_iter()
        .enumerate()
        .map(|(i, floor)| floor.ok_or(anyhow!("no specification for floor {}", i)))
        .process_results(|iter| {
            iter.collect_array()
                .expect("floors declared with 4 elements")
        })
        .map(move |floors| (floors, ids))
}
