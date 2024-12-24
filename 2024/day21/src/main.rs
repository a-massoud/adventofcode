// This one was really hard. Needed to look online for hints that BFS was not the way, recursive
// solution is the way. I was still storing full strings at the end of part 1, so I needed to fix
// that for part 2. I'm honestly not 100% sure why this works, but I am happy with it.

use anyhow::{anyhow, bail, Context};
use core::str;
use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

fn main() -> anyhow::Result<()> {
    let numeric_graph = graph_from_grid(vec![
        "789".as_bytes().to_vec(),
        "456".as_bytes().to_vec(),
        "123".as_bytes().to_vec(),
        "X0A".as_bytes().to_vec(),
    ]);
    let keypad_graph = graph_from_grid(vec!["X^A".as_bytes().to_vec(), "<v>".as_bytes().to_vec()]);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0]);
    }

    let input = fs::read_to_string(&args[1]).context("reading input file")?;
    let input: Vec<String> = input.lines().map(String::from).collect();

    println!(
        "Part 1: {}",
        input
            .iter()
            .map(|x| (
                get_movement_len(x, &keypad_graph, &numeric_graph, 2),
                x[..x.len() - 1].parse::<usize>().unwrap()
            ))
            .fold(0usize, |acc, (code, n)| code * n + acc)
    );
    println!(
        "Part 2: {}",
        input
            .iter()
            .map(|x| (
                get_movement_len(x, &keypad_graph, &numeric_graph, 25),
                x[..x.len() - 1].parse::<usize>().unwrap()
            ))
            .fold(0usize, |acc, (code, n)| code * n + acc)
    );

    Ok(())
}

fn graph_from_grid(grid: Vec<Vec<u8>>) -> HashMap<u8, [Option<u8>; 4]> {
    let mut graph = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == b'X' {
                continue;
            }

            let mut neighbors = [None, None, None, None];
            if j < row.len() - 1 && grid[i][j + 1] != b'X' {
                neighbors[0] = Some(grid[i][j + 1]);
            }
            if i > 0 && grid[i - 1][j] != b'X' {
                neighbors[1] = Some(grid[i - 1][j]);
            }
            if j > 0 && grid[i][j - 1] != b'X' {
                neighbors[2] = Some(grid[i][j - 1]);
            }
            if i < grid.len() - 1 && grid[i + 1][j] != b'X' {
                neighbors[3] = Some(grid[i + 1][j]);
            }

            graph.insert(ch, neighbors);
        }
    }

    graph
}

fn get_all_paths(
    c: u8,
    n: u8,
    graph: &HashMap<u8, [Option<u8>; 4]>,
) -> anyhow::Result<Vec<String>> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(c);
    visited.insert(c, (0, Vec::new()));

    while let Some(p) = queue.pop_front() {
        let d = visited.get(&p).expect("in queue but not visited").0;
        for (i, &q) in graph
            .get(&p)
            .ok_or(anyhow!("edges to nonexistent nodes"))?
            .iter()
            .enumerate()
        {
            let Some(q) = q else {
                continue;
            };
            if visited.contains_key(&q) && visited[&q].0 < d + 1 {
                continue;
            }

            if visited.contains_key(&q) && visited[&q].0 == d + 1 {
                visited.get_mut(&q).unwrap().1.push((
                    match i {
                        0 => b'>',
                        1 => b'^',
                        2 => b'<',
                        3 => b'v',
                        _ => unreachable!(),
                    },
                    p,
                ));
            } else {
                visited.insert(
                    q,
                    (
                        d + 1,
                        vec![(
                            match i {
                                0 => b'>',
                                1 => b'^',
                                2 => b'<',
                                3 => b'v',
                                _ => unreachable!(),
                            },
                            p,
                        )],
                    ),
                );
            }
            queue.push_back(q);
        }
    }

    let mut stack = vec![(n, String::new())];
    let mut routes = Vec::new();
    while let Some((p, r)) = stack.pop() {
        if p == c {
            routes.push(r);
            continue;
        }
        let parents = &mut visited.get_mut(&p).ok_or(anyhow!("malformed graph"))?.1;
        if parents.is_empty() {
            continue;
        }
        for par in parents {
            stack.push((par.1, r.clone() + str::from_utf8(&[par.0]).unwrap()));
        }
    }

    Ok(routes.iter().map(|x| x.chars().rev().collect()).collect())
}

fn get_movement_len(
    code: &str,
    keypad_graph: &HashMap<u8, [Option<u8>; 4]>,
    numeric_graph: &HashMap<u8, [Option<u8>; 4]>,
    n_keypads: usize,
) -> usize {
    fn recurse(
        code: &str,
        keypad_graph: &HashMap<u8, [Option<u8>; 4]>,
        numeric_graph: &HashMap<u8, [Option<u8>; 4]>,
        n_keypads: usize,
        n: usize,
        cache: &mut HashMap<(String, usize), usize>
    ) -> usize {
        if let Some(a) = cache.get(&(code.to_owned(), n)) {
            return a.to_owned();
        }
        if n == n_keypads + 1 {
            return code.len();
        }
        let graph = if n == 0 { numeric_graph } else { keypad_graph };
        let ncode = String::from('A') + code;
        let mut s = 0;
        for seg in ncode.as_bytes().windows(2) {
            s += &get_all_paths(seg[0], seg[1], graph)
                .unwrap()
                .iter()
                .map(|x| {
                    recurse(
                        &(x.to_owned() + "A"),
                        keypad_graph,
                        numeric_graph,
                        n_keypads,
                        n + 1,
                        cache
                    )
                })
                .min_by(|x, y| x.cmp(y))
                .unwrap();
        }

        cache.insert((code.to_owned(), n), s);
        s
    }

    let mut cache = HashMap::new();
    recurse(code, keypad_graph, numeric_graph, n_keypads, 0, &mut cache)
}
