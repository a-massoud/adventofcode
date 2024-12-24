// learning about the Bron-Kerbosch algorithm was fun

use anyhow::{bail, Context};
use regex::Regex;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env, fs,
    sync::LazyLock,
};

type Graph = HashMap<String, HashSet<String>>;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("no input file provided");
    }

    let input = fs::read_to_string(&args[1]).context("failed to read input")?;
    let input = parse_input(&input).context("failed to parse input")?;

    println!("Part 1: {}", count_loops(&input, 3));
    print!("Part 2: ");
    let mut max_clique: Vec<String> = find_max_clique(&input).into_iter().collect();
    max_clique.sort();
    for x in &max_clique[..max_clique.len() - 1] {
        print!("{},", x);
    }
    println!("{}", max_clique[max_clique.len() - 1]);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Graph> {
    static LINE_PATTERN: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(..)-(..)$").expect("static regex failed to compile"));
    let _ = &*LINE_PATTERN; // initialize

    let mut graph = Graph::new();

    for line in input.lines() {
        let Some((_, [a, b])) = LINE_PATTERN.captures(line).map(|x| x.extract()) else {
            bail!("line `{}` does not match pattern", line);
        };

        let a = String::from(a);
        let b = String::from(b);
        if let Some(e) = graph.get_mut(&a) {
            e.insert(b.clone());
        } else {
            graph.insert(a.clone(), HashSet::from([b.clone()]));
        }
        if let Some(e) = graph.get_mut(&b) {
            e.insert(a.clone());
        } else {
            graph.insert(b.clone(), HashSet::from([a.clone()]));
        }
    }

    Ok(graph)
}

fn count_loops(graph: &Graph, size: usize) -> usize {
    fn recurse(graph: &Graph, n: usize, c: &str, target: &str) -> HashSet<Vec<String>> {
        if n == 0 {
            return if graph[c].contains(target) {
                HashSet::from([vec![String::from(c)]])
            } else {
                HashSet::new()
            };
        }

        graph[c]
            .iter()
            .filter(|&x| x != target)
            .flat_map(|x| {
                recurse(graph, n - 1, x, target)
                    .into_iter()
                    .map(|x| x.into_iter().chain([String::from(c)]).collect())
            })
            .collect()
    }

    let a = graph
        .keys()
        .filter(|x| x.as_bytes()[0] == b't')
        .flat_map(|x| recurse(graph, size - 1, x, x).into_iter())
        .map(|x| x.into_iter().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    let mut b = Vec::new();
    for set in a {
        let mut should_add = true;
        for comp in &b {
            if &set == comp {
                should_add = false;
                break;
            }
        }

        if should_add {
            b.push(set.clone());
        }
    }

    b.len()
}

fn find_max_clique(graph: &Graph) -> HashSet<String> {
    fn recurse(
        graph: &Graph,
        r: HashSet<String>,
        mut p: HashSet<String>,
        mut x: HashSet<String>,
    ) -> Vec<HashSet<String>> {
        let Some(u) = p.union(&x).next().cloned() else {
            return vec![r];
        };

        let mut ret = Vec::new();
        for v in p.clone().into_iter().filter(|v| !graph[&u].contains(v)) {
            ret.append(&mut recurse(
                graph,
                r.union(&HashSet::from([v.clone()])).cloned().collect(),
                p.intersection(&graph[&v]).cloned().collect(),
                x.intersection(&graph[&v]).cloned().collect(),
            ));
            p.remove(&v);
            x.insert(v.clone());
        }

        ret
    }

    let mut r = recurse(
        graph,
        HashSet::new(),
        graph.keys().cloned().collect(),
        HashSet::new(),
    );
    r.sort_by_key(|y| Reverse(y.len()));

    r[0].clone()
}
