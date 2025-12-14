use anyhow::{anyhow, bail};
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, hash_map},
    env, fs, iter,
};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let (people, happiness) = parse_input(&input)?;

    let p1 = maximize_happiness(&people, &happiness);
    println!("Part 1: {}", p1);

    let newname = (0..)
        .find_map(|i| {
            let i = i.to_string();
            (!people.contains(&i)).then(|| i)
        })
        .unwrap(); // always will hit because people is finite
    let p2 = maximize_happiness(
        &(people.union(&[newname].into()).cloned().collect()),
        &happiness,
    );
    println!("Part 2: {}", p2);

    Ok(())
}

type PeopleSet = HashSet<String>;
type HappinessMap = HashMap<String, HashMap<String, i32>>;

fn parse_input(input: &str) -> anyhow::Result<(PeopleSet, HappinessMap)> {
    let re =
        Regex::new(r"^(\S+) would (gain|lose) (\d+) happiness units by sitting next to (\S+).$")
            .unwrap();

    let mut happiness = HappinessMap::new();
    for line in input.lines() {
        let (_, [name, gl, n, other]) = re
            .captures(line)
            .ok_or(anyhow!("bad line `{}`", line))?
            .extract();
        let v = n.parse::<i32>().unwrap() * if gl == "lose" { -1 } else { 1 };
        match happiness.entry(name.to_owned().to_lowercase()) {
            hash_map::Entry::Occupied(mut entry) => {
                if entry
                    .get_mut()
                    .insert(other.to_owned().to_lowercase(), v)
                    .is_some()
                {
                    bail!("line `{}` repeats a condition", line);
                }
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(HashMap::from([(other.to_owned().to_lowercase(), v)]));
            }
        }
    }

    let people: PeopleSet = happiness
        .iter()
        .flat_map(|(k, v)| iter::chain(iter::once(k.to_owned()), v.keys().cloned()))
        .collect();

    Ok((people, happiness))
}

fn maximize_happiness(people: &PeopleSet, happiness: &HappinessMap) -> i32 {
    if people.len() <= 1 {
        return 0;
    }

    if people.len() == 2 {
        let (p1, p2) = people.iter().collect_tuple().unwrap();
        return happiness.get(p1).and_then(|map| map.get(p2)).unwrap_or(&0)
            + happiness.get(p2).and_then(|map| map.get(p1)).unwrap_or(&0);
    }

    // we fix the first element of the permutation because it is cyclic
    let first = people.iter().next().unwrap();

    people
        .iter()
        .filter(|&x| x != first)
        .permutations(people.len() - 1)
        .filter(|perm| {
            perm.iter()
                .zip(perm.iter().rev())
                .find_map(|(a, b)| (a < b).then(|| true).or((a > b).then(|| false)))
                .unwrap_or(true)
        }) // filter out only one mirror image along the axis through first (lexicographic ordering without allocation)
        .map(|perm| {
            let mut s = 0;
            let len = perm.len() + 1;
            let idx = |i: usize| {if i == 0 {&first} else {perm[i - 1]}};

            for i in 0..len {
                let Some(person) = happiness.get(idx(i)) else {
                    continue;
                };

                let l = (i + len - 1) % len;
                let r = (i + 1) % len;

                s += person.get(idx(l)).unwrap_or(&0);
                s += person.get(idx(r)).unwrap_or(&0);
            }

            s
        })
        .max()
        .unwrap_or(0)
}
