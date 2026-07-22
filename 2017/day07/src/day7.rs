use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    num::ParseIntError,
};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Tower {
    weights: HashMap<String, i64>,
    adjacencies: HashMap<String, Vec<String>>,
    root: String,
}

impl Tower {
    fn new(
        weights: HashMap<String, i64>,
        adjacencies: HashMap<String, Vec<String>>,
    ) -> Result<Self, TowerNewError> {
        for prog in adjacencies.values().flatten() {
            if !weights.contains_key(prog) {
                return Err(TowerNewError::UndescribedProgram {
                    name: prog.to_owned(),
                });
            }
        }

        let mut indegrees: HashMap<_, _> = weights.keys().map(|k| (k, 0usize)).collect();
        for neighbors in adjacencies.values() {
            for neighbor in neighbors {
                indegrees
                    .entry(neighbor)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }

        let mut root = None;
        for (node, degree) in indegrees {
            if degree == 0 {
                if root.is_some() {
                    return Err(TowerNewError::NoRoot);
                }
                root = Some(node.to_owned());
            } else if degree != 1 {
                return Err(TowerNewError::NonTree);
            }
        }
        let Some(root) = root else {
            return Err(TowerNewError::NoRoot);
        };

        let mut stack = vec![&root];
        let mut reachable = HashSet::from([&root]);
        while let Some(node) = stack.pop() {
            if let Some(children) = adjacencies.get(node) {
                for child in children {
                    if !reachable.insert(child) {
                        return Err(TowerNewError::NonTree);
                    }
                    stack.push(child);
                }
            }
        }
        for node in weights.keys() {
            if !reachable.contains(node) {
                return Err(TowerNewError::NonTree);
            }
        }

        Ok(Tower {
            weights,
            adjacencies,
            root,
        })
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn get_balancing_correction(&self) -> Option<(&str, i64)> {
        let mut total_weights = HashMap::new();
        let mut stack = vec![(&self.root, false)];

        while let Some((node, processed)) = stack.pop() {
            if processed {
                if let Some(children) = self.adjacencies.get(node) {
                    let mut counts = HashMap::new();
                    for child in children {
                        counts
                            .entry(total_weights[child])
                            .and_modify(|v| *v += 1)
                            .or_insert(1usize);
                    }
                    let expected_weight = counts
                        .into_iter()
                        .max_by_key(|(_, v)| *v)
                        .map(|(expected, _)| expected);
                    if let Some(expected_weight) = expected_weight
                        && let Some(uneven) = children
                            .iter()
                            .find(|v| total_weights[v] != expected_weight)
                    {
                        return Some((
                            uneven,
                            self.weights[uneven] - total_weights[uneven] + expected_weight,
                        ));
                    }

                    total_weights.insert(
                        node,
                        self.weights[node]
                            + children
                                .iter()
                                .map(|child| total_weights[child])
                                .sum::<i64>(),
                    );
                } else {
                    total_weights.insert(node, self.weights[node]);
                }
            } else {
                stack.push((node, true));

                if let Some(children) = self.adjacencies.get(node) {
                    for child in children {
                        stack.push((child, false));
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug, Error)]
pub enum TowerNewError {
    #[error("No weight provided for program `{name}`")]
    UndescribedProgram { name: String },
    #[error("No root found")]
    NoRoot,
    #[error("Tower is not a tree")]
    NonTree,
}

#[derive(Debug, Error)]
pub enum ReadTowerError {
    #[error("Failed to read line {no}: {e}")]
    ReadLine { no: usize, e: io::Error },
    #[error("No name on line {no}")]
    NoName { no: usize },
    #[error("Second definition of `{name}` on line {no}")]
    MultipleDefinitions { no: usize, name: String },
    #[error("No weight on line {no}")]
    NoWeight { no: usize },
    #[error("Invalid weight `{s}` on line {no}")]
    WeightFormat { no: usize, s: String },
    #[error("Failed to parse weight `{s}` on line {no}: {e}")]
    ParseWeight {
        no: usize,
        s: String,
        e: ParseIntError,
    },
    #[error("Negative weight {v} on line {no}")]
    NegativeWeight { no: usize, v: i64 },
    #[error("Invalid token `{tok}` on line {no}")]
    InvalidToken { no: usize, tok: String },
    #[error("Unexpected end of line {no}")]
    UnexpectedEol { no: usize },
    #[error("Expected end of line {no}, found `{tok}`")]
    ExpectedEol { no: usize, tok: String },
    #[error("Failed to construct tower: {0}")]
    TowerNew(#[from] TowerNewError),
}

pub fn read_tower(input: impl BufRead) -> Result<Tower, ReadTowerError> {
    let mut weights = HashMap::new();
    let mut adjacencies = HashMap::new();

    for (it, line) in input.lines().enumerate() {
        let no = it + 1;
        let line = line.map_err(|e| ReadTowerError::ReadLine { no, e })?;
        let mut split = line.split_whitespace();

        let name = split.next().ok_or(ReadTowerError::NoName { no })?;
        let weight_str = split.next().ok_or(ReadTowerError::NoWeight { no })?;
        let weight_stripped = weight_str
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .ok_or_else(|| ReadTowerError::WeightFormat {
                no,
                s: weight_str.to_owned(),
            })?;
        let weight = weight_stripped
            .parse()
            .map_err(|e| ReadTowerError::ParseWeight {
                no,
                s: weight_stripped.to_owned(),
                e,
            })?;
        if weight < 0 {
            return Err(ReadTowerError::NegativeWeight { no, v: weight });
        }

        if weights.insert(name.to_owned(), weight).is_some() {
            return Err(ReadTowerError::MultipleDefinitions {
                no,
                name: name.to_owned(),
            });
        }

        if let Some(arrow) = split.next() {
            if arrow != "->" {
                return Err(ReadTowerError::InvalidToken {
                    no,
                    tok: arrow.to_owned(),
                });
            }

            let mut adj = Vec::new();
            loop {
                let Some(tok) = split.next() else {
                    return Err(ReadTowerError::UnexpectedEol { no });
                };

                match tok.strip_suffix(',') {
                    Some(tok) => {
                        adj.push(tok.to_owned());
                    }
                    None => {
                        if let Some(v) = split.next() {
                            return Err(ReadTowerError::ExpectedEol {
                                no,
                                tok: v.to_owned(),
                            });
                        }
                        adj.push(tok.to_owned());
                        break;
                    }
                }
            }
            adjacencies.insert(name.to_owned(), adj);
        }
    }

    Tower::new(weights, adjacencies).map_err(ReadTowerError::from)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::day7;

    #[test]
    fn sample_input() {
        let tower = day7::read_tower(Cursor::new(
            "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
        ))
        .expect("Sample input parses");

        assert_eq!(tower.root(), "tknk", "Sample root matches output");
        assert_eq!(
            tower.get_balancing_correction(),
            Some(("ugml", 60)),
            "Sample correction matches output"
        );
    }
}
