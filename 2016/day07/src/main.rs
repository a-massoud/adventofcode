// Also nice and simple

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("no input provided"))?;

    let input = read_input(input_path)?;

    let n_supporting_tls = input.iter().filter(|&v| v.supports_tls()).count();
    println!("TLS: {}", n_supporting_tls);

    let n_supporting_ssl = input.iter().filter(|&v| v.supports_ssl()).count();
    println!("SSL: {}", n_supporting_ssl);

    Ok(())
}

#[derive(Debug, Default)]
struct Address {
    non_hypernet: Vec<Vec<u8>>,
    hypernet: Vec<Vec<u8>>,
}

impl Address {
    fn new(address: &[u8]) -> anyhow::Result<Self> {
        let mut non_hypernet = vec![Vec::new()];
        let mut hypernet = vec![Vec::new()];
        let mut hyper_depth = 0usize;

        for &ch in address {
            if hyper_depth == 0 {
                if ch == b'[' {
                    hyper_depth = 1;
                    hypernet.push(Vec::new());
                    continue;
                }
                if ch == b']' {
                    bail!("unmatched close bracket in address `{:?}`", address);
                }
                non_hypernet
                    .last_mut()
                    .expect("non_hypernet should be nonempty")
                    .push(ch);
            } else {
                if ch == b']' {
                    hyper_depth -= 1;
                    if hyper_depth == 0 {
                        non_hypernet.push(Vec::new());
                        continue;
                    }
                } else if ch == b'[' {
                    hyper_depth += 1;
                }
                hypernet
                    .last_mut()
                    .expect("hypernet should be nonempty")
                    .push(ch);
            }
        }

        if hyper_depth != 0 {
            bail!("unmatched open bracket in address `{:?}`", address);
        }

        Ok(Self {
            non_hypernet,
            hypernet,
        })
    }

    fn supports_tls(&self) -> bool {
        let is_abba = |v: &[u8]| v.len() == 4 && v[0] != v[1] && v[0] == v[3] && v[1] == v[2];

        self.non_hypernet
            .iter()
            .any(|sec| sec.windows(4).any(is_abba))
            && self
                .hypernet
                .iter()
                .all(|sec| sec.windows(4).all(|v| !is_abba(v)))
    }

    fn supports_ssl(&self) -> bool {
        let abas: Vec<_> = self
            .non_hypernet
            .iter()
            .flat_map(|sec| {
                sec.windows(3).filter_map(|v| {
                    if v[0] != v[1] && v[0] == v[2] {
                        Some((v[0], v[1]))
                    } else {
                        None
                    }
                })
            })
            .collect();
        if abas.is_empty() {
            return false;
        }

        self.hypernet.iter().any(|sec| {
            sec.windows(3)
                .any(|v| v[0] == v[2] && abas.contains(&(v[1], v[0])))
        })
    }
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<Address>> {
    let input = BufReader::new(File::open(path)?);

    input
        .lines()
        .map(|line| Address::new(line?.as_bytes()))
        .collect()
}
