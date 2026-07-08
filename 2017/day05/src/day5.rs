use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result};

#[derive(Debug, Default, Clone)]
pub struct Offsets(Vec<isize>);

impl Offsets {
    fn count_steps_helper<const USE_DECREASE_RULE: bool>(&self) -> usize {
        let mut n = 0;
        let mut offsets = self.clone();
        let mut ip = 0isize;

        while 0 <= ip && ip < offsets.0.len() as isize {
            let new_ip = ip + offsets.0[ip as usize];

            if !USE_DECREASE_RULE || offsets.0[ip as usize] < 3 {
                offsets.0[ip as usize] += 1;
            } else {
                offsets.0[ip as usize] -= 1;
            }

            ip = new_ip;
            n += 1;
        }

        n
    }

    #[inline(always)]
    pub fn count_steps(&self) -> usize {
        self.count_steps_helper::<false>()
    }

    #[inline(always)]
    pub fn count_steps_with_decrease_rule(&self) -> usize {
        self.count_steps_helper::<true>()
    }
}

pub fn parse_offsets(input: impl BufRead) -> Result<Offsets> {
    let offsets = input
        .lines()
        .enumerate()
        .map(|(it, line)| {
            let no = it + 1;
            let line = line.with_context(|| format!("Failed to read line {}", no))?;

            line.trim()
                .parse()
                .with_context(|| format!("Failed to parse line {}", no))
        })
        .collect::<Result<_>>()?;

    Ok(Offsets(offsets))
}

pub fn read_offsets(path: impl AsRef<Path>) -> Result<Offsets> {
    let path = path.as_ref();
    let input = BufReader::new(
        File::open(path)
            .with_context(|| format!("Failed to open `{}` for reading", path.display()))?,
    );
    parse_offsets(input).with_context(|| format!("Failed to parse `{}`", path.display()))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::day5::parse_offsets;

    #[test]
    fn sample_input_works() {
        let offsets = parse_offsets(Cursor::new("0\n3\n0\n1\n-3")).expect("Sample input parses");
        assert_eq!(
            offsets.count_steps(),
            5,
            "Sample input matches without decrease rule"
        );
        assert_eq!(
            offsets.count_steps_with_decrease_rule(),
            10,
            "Sample input matches with decrease rule"
        );
    }
}
