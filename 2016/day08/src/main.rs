// I love accidentally solving part 2 in part 1.

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, IndexMut},
    path::Path,
};

use anyhow::{anyhow, bail};
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("no argument provided"))?;
    let input = read_input(input_path)?;

    let mut screen = Screen::new(50, 6);
    screen.simulate(&input)?;
    println!("{}", screen.display());
    println!("Enabled pixels: {}", screen.count_pixels());

    Ok(())
}

#[derive(Debug, Clone)]
struct Screen {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![false; width * height],
            width,
            height,
        }
    }

    fn count_pixels(&self) -> usize {
        self.data.iter().filter(|&&x| x).count()
    }

    fn step(&mut self, instruction: &Instruction) -> anyhow::Result<()> {
        match *instruction {
            Instruction::Rect { width, height } => {
                if width > self.width || height > self.height {
                    bail!("width/height out of range");
                }

                for x in 0..width {
                    for y in 0..height {
                        self[(x, y)] = true;
                    }
                }
            }
            Instruction::Row { y, n } => {
                if y >= self.height {
                    bail!("height out of range");
                }

                let mut new_row = vec![false; self.width];
                for x in 0..self.width {
                    new_row[(x + n) % self.width] = self[(x, y)];
                }
                self.data[y * self.width..(y + 1) * self.width].copy_from_slice(&new_row);
            }
            Instruction::Col { x, n } => {
                if x >= self.width {
                    bail!("width out of range");
                }

                let mut new_col = vec![false; self.height];
                for y in 0..self.height {
                    new_col[(y + n) % self.height] = self[(x, y)];
                }

                for (y, v) in new_col.into_iter().enumerate() {
                    self[(x, y)] = v;
                }
            }
        }

        Ok(())
    }

    fn simulate(&mut self, instructions: &[Instruction]) -> anyhow::Result<()> {
        for instruction in instructions {
            self.step(instruction)?;
        }

        Ok(())
    }

    fn display(&self) -> String {
        let mut r = String::with_capacity(self.width * self.height + self.height - 1);

        for y in 0..self.height {
            for x in 0..self.width {
                r.push(if self[(x, y)] { '#' } else { ' ' });
            }

            if y + 1 != self.height {
                r.push('\n');
            }
        }

        r
    }
}

impl Index<(usize, usize)> for Screen {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Screen {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x + y * self.width]
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Rect { width: usize, height: usize },
    Row { y: usize, n: usize },
    Col { x: usize, n: usize },
}

fn parse_input(input: impl BufRead) -> anyhow::Result<Vec<Instruction>> {
    let rect_regex = Regex::new(r"^rect ([0-9]+)x([0-9]+)$").unwrap();
    let row_regex = Regex::new(r"^rotate row y=([0-9]+) by ([0-9]+)$").unwrap();
    let col_regex = Regex::new(r"^rotate column x=([0-9]+) by ([0-9]+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let line = line?;

            if let Some((_, [width, height])) = rect_regex.captures(&line).map(|c| c.extract()) {
                Ok(Instruction::Rect {
                    width: width.parse()?,
                    height: height.parse()?,
                })
            } else if let Some((_, [y, n])) = row_regex.captures(&line).map(|c| c.extract()) {
                Ok(Instruction::Row {
                    y: y.parse()?,
                    n: n.parse()?,
                })
            } else if let Some((_, [x, n])) = col_regex.captures(&line).map(|c| c.extract()) {
                Ok(Instruction::Col {
                    x: x.parse()?,
                    n: n.parse()?,
                })
            } else {
                Err(anyhow!("malformed line `{}`", line))
            }
        })
        .collect()
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<Instruction>> {
    let input = BufReader::new(File::open(path)?);
    parse_input(input)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    const SAMPLE_INPUT: &str = "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    #[test]
    fn sample_input() {
        let input = parse_input(Cursor::new(SAMPLE_INPUT)).expect("failed to parse input");
        let mut screen = Screen::new(7, 3);
        screen.simulate(&input).expect("failed to simulate");
        assert_eq!(screen.count_pixels(), 6, "state:\n{}", screen.display());
    }
}
