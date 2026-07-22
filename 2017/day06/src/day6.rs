use std::{
    collections::{HashMap, hash_map::Entry},
    io::{self, Read},
    num::ParseIntError,
};

use thiserror::Error;

#[derive(Debug, Default, Clone)]
pub struct Memory {
    banks: Vec<u64>,
}

impl Memory {
    fn into_next(mut self) -> Self {
        let Some((idx, mut amt)) = self
            .banks
            .iter()
            .copied()
            .enumerate()
            .rev()
            .max_by_key(|(_, v)| *v)
        else {
            return self; // no change
        };

        self.banks[idx] = 0;
        let mut i = (idx + 1) % self.banks.len();
        while amt > 0 {
            self.banks[i] += 1;
            i += 1;
            i %= self.banks.len();
            amt -= 1;
        }

        self
    }

    pub fn find_cycle(&self) -> (usize, usize) {
        let mut current = self.clone();
        let mut count = 0;
        let mut visited = HashMap::new();
        loop {
            match visited.entry(current.banks.clone()) {
                Entry::Occupied(entry) => {
                    return (count, count - entry.get());
                }
                Entry::Vacant(entry) => {
                    entry.insert(count);
                    count += 1;
                    current = current.into_next();
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseMemoryError {
    #[error("Failed to read input: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to parse value `{word}`: {e}")]
    UnexpectedValue { word: String, e: ParseIntError },
}

pub fn read_memory(input: impl Read) -> Result<Memory, ParseMemoryError> {
    let input = io::read_to_string(input)?;
    Ok(Memory {
        banks: input
            .split_whitespace()
            .map(|tok| {
                tok.parse().map_err(|e| ParseMemoryError::UnexpectedValue {
                    word: tok.to_owned(),
                    e,
                })
            })
            .collect::<Result<_, _>>()?,
    })
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::day6;

    #[test]
    fn sample_input() {
        let memory = day6::read_memory(Cursor::new("0 2 7 0")).expect("Sample input parses");
        assert_eq!(
            memory.find_cycle(),
            (5, 4),
            "Sample input calculates correctly"
        );
    }
}
