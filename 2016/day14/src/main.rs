// That was fun. Definitely not the *most* efficient solution, but not terrible.

use std::{collections::HashSet, env};

use anyhow::{Result, anyhow};
use itertools::Itertools;
use md5::{Digest, Md5};

fn main() -> Result<()> {
    let input = env::args().nth(1).ok_or(anyhow!("no argument provided"))?;

    println!("===Part 1===");
    let key = get_64th_key(&input, 0);
    println!("64th key: {}", key);
    println!();

    println!("===Part 2===");
    let key = get_64th_key(&input, 2016);
    println!("64th key: {}", key);

    Ok(())
}

fn get_64th_key(salt: &str, extra_hashes: usize) -> usize {
    let mut candidates = Vec::new();
    let mut keys = HashSet::new();

    let mut stop_at = None;

    let mut idx: usize = 0;
    while stop_at.is_none_or(|limit| idx <= limit) {
        let pass = salt.to_owned() + &idx.to_string();
        let mut hasher = Md5::new_with_prefix(pass);
        let mut hash = hasher.finalize_reset();
        for _ in 0..extra_hashes {
            let pass = hex::encode(hash);
            hasher.update(pass);
            hash = hasher.finalize_reset();
        }

        candidates.retain(|(_, jdx)| idx <= jdx + 1000);

        for jdx in hash
            .iter()
            .flat_map(|b| [(b >> 4) & 0x0F, b & 0x0F])
            .tuple_windows()
            .filter_map(|(a, b, c, d, e)| (a == b && a == c && a == d && a == e).then_some(a))
            .flat_map(|v| {
                candidates
                    .iter()
                    .filter_map(move |(u, jdx)| (*u == v).then_some(*jdx))
            })
        {
            keys.insert(jdx);
        }

        if let Some(v) = hash
            .iter()
            .flat_map(|b| [(b >> 4) & 0x0F, b & 0x0F])
            .tuple_windows()
            .filter_map(|(a, b, c)| (a == b && a == c).then_some(a))
            .next()
        {
            candidates.push((v, idx));
        }

        if keys.len() >= 64 && stop_at.is_none() {
            stop_at = Some(idx + 1000);
        }

        idx += 1;
    }

    let mut keys: Vec<_> = keys.into_iter().collect();
    keys.sort();
    keys[63]
}

#[cfg(test)]
mod test {
    use crate::get_64th_key;

    #[test]
    fn sample_input_p1() {
        assert_eq!(get_64th_key("abc", 0), 22728);
    }

    #[test]
    fn sample_input_p2() {
        assert_eq!(get_64th_key("abc", 2016), 22551);
    }
}
