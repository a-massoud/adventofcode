use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    let (rules, input) = read_input(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;

    let p1 = find_n_possible(&rules, &input);
    println!("Part 1: {}", p1);

    let p2 = find_steps_to(&rules, &input).ok_or(anyhow!("no route to molecule"))?;
    println!("Part 2: {}", p2);

    Ok(())
}

fn read_input(
    path: impl AsRef<Path>,
) -> anyhow::Result<(HashMap<String, HashSet<String>>, String)> {
    let reader = BufReader::new(File::open(path)?);
    let lines: Vec<_> = reader
        .lines()
        .map(|line| line.map(|line| line.trim().to_owned()))
        .collect::<Result<_, _>>()?;

    let mut split = lines.split(|line| line.is_empty());
    let replacements = split.next().ok_or(anyhow!("no lines"))?;
    let mut text = split.flatten();
    let first = text.next().ok_or(anyhow!("no text"))?.to_owned();
    let text = text.fold(first, |acc, line| acc + "\n" + line);

    let replacements = replacements
        .iter()
        .map(|s| {
            let (from, arrow, to) = s
                .split_whitespace()
                .collect_tuple()
                .ok_or(anyhow!("line `{}` not splittable in 3", s))?;
            if arrow != "=>" {
                bail!("middle not arrow in line `{}`", s);
            }

            Ok((from.to_owned(), to.to_owned()))
        })
        .try_fold(
            HashMap::<String, HashSet<String>>::new(),
            |mut acc, rule| -> anyhow::Result<_> {
                let (from, to) = rule?;
                acc.entry(from)
                    .and_modify(|set| {
                        set.insert(to.clone());
                    })
                    .or_insert_with(|| HashSet::from([to]));
                Ok(acc)
            },
        )?;

    Ok((replacements, text))
}

fn find_n_possible(rules: &HashMap<String, HashSet<String>>, text: &str) -> usize {
    let mut replaced = HashSet::new();

    for (from, to_set) in rules {
        for (idx, _) in text.match_indices(from) {
            for to in to_set {
                let mut t = text.as_bytes()[..idx].to_owned();
                t.extend_from_slice(to.as_bytes());
                t.extend_from_slice(&text.as_bytes()[(idx + from.len())..]);
                replaced.insert(t);
            }
        }
    }

    replaced.len()
}

#[derive(Debug, PartialEq, Eq)]
struct OrdMolecule(pub String, usize);

impl PartialOrd for OrdMolecule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdMolecule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .0
            .len()
            .cmp(&self.0.len())
            .then_with(|| other.1.cmp(&self.1))
            .then_with(|| self.0.cmp(&other.0))
    }
}

fn find_steps_to(rules: &HashMap<String, HashSet<String>>, target: &str) -> Option<usize> {
    let rules: Vec<(String, String)> = rules
        .iter()
        .flat_map(|(k, v)| v.iter().map(|v| (v.to_owned(), k.to_owned())))
        .collect();

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(OrdMolecule(target.to_owned(), 0));

    while let Some(OrdMolecule(s, steps)) = queue.pop() {
        if s == "e" {
            return Some(steps);
        }

        let mut to_push = Vec::new();
        for (from, to) in &rules {
            for (idx, _) in s.match_indices(from) {
                let mut t = s.as_bytes()[..idx].to_owned();
                t.extend_from_slice(to.as_bytes());
                t.extend_from_slice(&s.as_bytes()[(idx + from.len())..]);
                if visited.insert(t.clone()) {
                    to_push.push(String::from_utf8(t).unwrap());
                }
            }
        }

        for v in to_push {
            queue.push(OrdMolecule(v, steps + 1));
        }
    }

    None
}
