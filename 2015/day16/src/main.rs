use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{anyhow, bail};

const STANDARD: Sue = Sue {
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

fn main() -> anyhow::Result<()> {
    let sues = read_input(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;

    println!(
        "Part 1: {}",
        sues.iter()
            .position(Sue::matches_std)
            .ok_or(anyhow!("no matching sue found"))?
            + 1
    );

    println!(
        "Part 2: {}",
        sues.iter()
            .position(Sue::matches_std_with_ranges)
            .ok_or(anyhow!("no matching sue found"))?
            + 1
    );

    Ok(())
}

#[derive(Debug, Default, Clone, Copy)]
struct Sue {
    children: Option<i64>,
    cats: Option<i64>,
    samoyeds: Option<i64>,
    pomeranians: Option<i64>,
    akitas: Option<i64>,
    vizslas: Option<i64>,
    goldfish: Option<i64>,
    trees: Option<i64>,
    cars: Option<i64>,
    perfumes: Option<i64>,
}

impl Sue {
    pub fn matches_std(&self) -> bool {
        self.children
            .map_or(true, |v| v == STANDARD.children.unwrap())
            && self.cats.map_or(true, |v| v == STANDARD.cats.unwrap())
            && self
                .samoyeds
                .map_or(true, |v| v == STANDARD.samoyeds.unwrap())
            && self
                .pomeranians
                .map_or(true, |v| v == STANDARD.pomeranians.unwrap())
            && self.akitas.map_or(true, |v| v == STANDARD.akitas.unwrap())
            && self
                .vizslas
                .map_or(true, |v| v == STANDARD.vizslas.unwrap())
            && self
                .goldfish
                .map_or(true, |v| v == STANDARD.goldfish.unwrap())
            && self.trees.map_or(true, |v| v == STANDARD.trees.unwrap())
            && self.cars.map_or(true, |v| v == STANDARD.cars.unwrap())
            && self
                .perfumes
                .map_or(true, |v| v == STANDARD.perfumes.unwrap())
    }

    pub fn matches_std_with_ranges(&self) -> bool {
        self.children
            .map_or(true, |v| v == STANDARD.children.unwrap())
            && self.cats.map_or(true, |v| v > STANDARD.cats.unwrap())
            && self
                .samoyeds
                .map_or(true, |v| v == STANDARD.samoyeds.unwrap())
            && self
                .pomeranians
                .map_or(true, |v| v < STANDARD.pomeranians.unwrap())
            && self.akitas.map_or(true, |v| v == STANDARD.akitas.unwrap())
            && self
                .vizslas
                .map_or(true, |v| v == STANDARD.vizslas.unwrap())
            && self
                .goldfish
                .map_or(true, |v| v < STANDARD.goldfish.unwrap())
            && self.trees.map_or(true, |v| v > STANDARD.trees.unwrap())
            && self.cars.map_or(true, |v| v == STANDARD.cars.unwrap())
            && self
                .perfumes
                .map_or(true, |v| v == STANDARD.perfumes.unwrap())
    }
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<Sue>> {
    let reader = BufReader::new(File::open(path)?);

    reader
        .lines()
        .map(|line| {
            let line = line?;

            let (_, rline) = line
                .split_once(": ")
                .ok_or(anyhow!("no colons in line `{}`", line))?;
            let mut r = Sue::default();

            for prop in rline.split(", ") {
                let (name, v) = prop
                    .split_once(": ")
                    .ok_or(anyhow!("unsplittable property `{}`", prop))?;

                match name {
                    "children" => r.children = Some(v.parse()?),
                    "cats" => r.cats = Some(v.parse()?),
                    "samoyeds" => r.samoyeds = Some(v.parse()?),
                    "pomeranians" => r.pomeranians = Some(v.parse()?),
                    "akitas" => r.akitas = Some(v.parse()?),
                    "vizslas" => r.vizslas = Some(v.parse()?),
                    "goldfish" => r.goldfish = Some(v.parse()?),
                    "trees" => r.trees = Some(v.parse()?),
                    "cars" => r.cars = Some(v.parse()?),
                    "perfumes" => r.perfumes = Some(v.parse()?),
                    _ => bail!("unknown property `{}`", name),
                }
            }

            Ok(r)
        })
        .collect()
}
