use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result, anyhow, bail};

fn main() -> Result<()> {
    let input_path = match env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!(
                "Usage: {} <input file>",
                env::args()
                    .next()
                    .expect("There is always at least one argument")
            );
            return Ok(());
        }
    };
    let program = read_input(input_path).context("Failed to read input")?;

    println!("===Part 1===");
    let text = program.run("abcdefgh").context("Failed to run program")?;
    println!("Output: {}", text);
    println!();

    println!("===Part 2===");
    let text = program
        .reverse("fbgdceah")
        .context("Failed to run program")?;
    println!("Output: {}", text);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SwapPos { x: usize, y: usize },
    SwapChar { x: u8, y: u8 },
    RotateLeft { n: usize },
    RotateRight { n: usize },
    RotateChar { x: u8 },
    RevPos { x: usize, y: usize },
    MovePos { x: usize, y: usize },
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        Self { instructions }
    }

    fn run(&self, text: &str) -> Result<String> {
        let mut text = Vec::from(text.as_bytes());

        for instr in &self.instructions {
            match instr {
                Instruction::SwapPos { x, y } => {
                    if *x >= text.len() {
                        bail!("Position {} is out of bounds", x);
                    }
                    if *y >= text.len() {
                        bail!("Position {} is out of bounds", y);
                    }

                    text.swap(*x, *y);
                }
                Instruction::SwapChar { x, y } => {
                    let Some(x) = text.iter().position(|v| x == v) else {
                        bail!("Character `{}` not in string", *x as char);
                    };
                    let Some(y) = text.iter().position(|v| y == v) else {
                        bail!("Character `{}` not in string", *y as char);
                    };

                    text.swap(x, y);
                }
                Instruction::RotateLeft { n } => {
                    let n = *n % text.len();
                    text = (0..text.len())
                        .map(|i| text[(i + n) % text.len()])
                        .collect();
                }
                Instruction::RotateRight { n } => {
                    let n = *n % text.len();
                    text = (0..text.len())
                        .map(|i| text[(i + text.len() - n) % text.len()])
                        .collect();
                }
                Instruction::RotateChar { x } => {
                    let Some(mut n) = text.iter().position(|v| x == v) else {
                        bail!("Character `{}` not in string", *x as char);
                    };

                    if n >= 4 {
                        n += 2;
                    } else {
                        n += 1;
                    }
                    let n = n % text.len();

                    text = (0..text.len())
                        .map(|i| text[(i + text.len() - n) % text.len()])
                        .collect();
                }
                Instruction::RevPos { x, y } => {
                    if *x >= text.len() {
                        bail!("Position {} is out of bounds", x);
                    }
                    if *y >= text.len() {
                        bail!("Position {} is out of bounds", y);
                    }

                    let mut i = (*x).min(*y);
                    let mut j = (*x).max(*y);
                    while i < j {
                        text.swap(i, j);
                        i += 1;
                        j -= 1;
                    }
                }
                Instruction::MovePos { x, y } => {
                    if *x >= text.len() {
                        bail!("Position {} is out of bounds", x);
                    }
                    if *y > text.len() {
                        bail!("Position {} is out of bounds", y);
                    }

                    let ch = text.remove(*x);
                    text.insert(*y, ch);
                }
            }
        }

        String::from_utf8(text).context("Final string is not UTF-8")
    }

    // note: this is only the inverse of run if each character is unique
    fn reverse(&self, text: &str) -> Result<String> {
        let mut text = Vec::from(text.as_bytes());

        for instr in self.instructions.iter().rev() {
            match instr {
                Instruction::SwapPos { x, y } => {
                    if *x >= text.len() {
                        bail!("Position {} is out of bounds", x);
                    }
                    if *y >= text.len() {
                        bail!("Position {} is out of bounds", y);
                    }

                    text.swap(*x, *y);
                }
                Instruction::SwapChar { x, y } => {
                    let Some(x) = text.iter().position(|v| x == v) else {
                        bail!("Character `{}` not in string", *x as char);
                    };
                    let Some(y) = text.iter().position(|v| y == v) else {
                        bail!("Character `{}` not in string", *y as char);
                    };

                    text.swap(x, y);
                }
                Instruction::RotateLeft { n } => {
                    let n = *n % text.len();
                    text = (0..text.len())
                        .map(|i| text[(i + text.len() - n) % text.len()])
                        .collect();
                }
                Instruction::RotateRight { n } => {
                    let n = *n % text.len();
                    text = (0..text.len())
                        .map(|i| text[(i + n) % text.len()])
                        .collect();
                }
                Instruction::RotateChar { x } => {
                    let mut found = false;
                    // there's a very good math way to do this but it relies on Z/text.len()Z being
                    // a field, i.e., text.len() being prime
                    for r in 0..text.len() {
                        let new_text: Vec<_> = (0..text.len())
                            .map(|i| text[(i + r) % text.len()])
                            .collect();

                        let Some(mut n) = new_text.iter().position(|v| x == v) else {
                            bail!("Character `{}` not in string", *x as char);
                        };

                        if n >= 4 {
                            n += 2;
                        } else {
                            n += 1;
                        }
                        let n = n % new_text.len();

                        if (0..new_text.len())
                            .map(|i| new_text[(i + new_text.len() - n) % new_text.len()])
                            .eq(text.iter().copied())
                        {
                            found = true;
                            text = new_text;
                            break;
                        }
                    }

                    if !found {
                        bail!("No inverse rotation by `{}` found", *x as char);
                    }
                }
                Instruction::RevPos { x, y } => {
                    if *x >= text.len() {
                        bail!("Position {} is out of bounds", x);
                    }
                    if *y >= text.len() {
                        bail!("Position {} is out of bounds", y);
                    }

                    let mut i = (*x).min(*y);
                    let mut j = (*x).max(*y);
                    while i < j {
                        text.swap(i, j);
                        i += 1;
                        j -= 1;
                    }
                }
                Instruction::MovePos { x, y } => {
                    if *x >= text.len() {
                        bail!("Position {} is out of bounds", x);
                    }
                    if *y > text.len() {
                        bail!("Position {} is out of bounds", y);
                    }

                    let ch = text.remove(*y);
                    text.insert(*x, ch);
                }
            }
        }

        String::from_utf8(text).context("Final string is not UTF-8")
    }
}

fn parse_instruction(instruction: &str) -> Result<Instruction> {
    if let Some(instr) = instruction.strip_prefix("swap position ") {
        let (x, y) = instr
            .split_once(" with position ")
            .ok_or_else(|| anyhow!("Bad instruction `{}`", instruction))?;
        let x = x
            .parse()
            .with_context(|| format!("Failed to parse `{}`", x))?;
        let y = y
            .parse()
            .with_context(|| format!("Failed to parse `{}`", y))?;

        return Ok(Instruction::SwapPos { x, y });
    }

    if let Some(instr) = instruction.strip_prefix("swap letter ") {
        let (x, y) = instr
            .split_once(" with letter ")
            .ok_or_else(|| anyhow!("Bad instruction `{}`", instruction))?;
        if x.len() != 1 || y.len() != 1 {
            bail!("Bad instruction `{}`", instruction);
        }

        return Ok(Instruction::SwapChar {
            x: x.as_bytes()[0],
            y: y.as_bytes()[0],
        });
    }

    if let Some(instr) = instruction.strip_prefix("rotate left ") {
        let n = instr
            .strip_suffix(" steps")
            .or_else(|| instr.strip_suffix(" step"))
            .ok_or_else(|| anyhow!("Bad instruction `{}`", instruction))?;
        let n = n
            .parse()
            .with_context(|| format!("Failed to parse `{}`", n))?;

        return Ok(Instruction::RotateLeft { n });
    }

    if let Some(instr) = instruction.strip_prefix("rotate right ") {
        let n = instr
            .strip_suffix(" steps")
            .or_else(|| instr.strip_suffix(" step"))
            .ok_or_else(|| anyhow!("Bad instruction `{}`", instruction))?;
        let n = n
            .parse()
            .with_context(|| format!("Failed to parse `{}`", n))?;

        return Ok(Instruction::RotateRight { n });
    }

    if let Some(instr) = instruction.strip_prefix("rotate based on position of letter ") {
        if instr.len() != 1 {
            bail!("Bad instruction `{}`", instruction);
        }

        return Ok(Instruction::RotateChar {
            x: instr.as_bytes()[0],
        });
    }

    if let Some(instr) = instruction.strip_prefix("reverse positions ") {
        let (x, y) = instr
            .split_once(" through ")
            .ok_or_else(|| anyhow!("Bad instruction `{}`", instruction))?;
        let x = x
            .parse()
            .with_context(|| format!("Failed to parse `{}`", x))?;
        let y = y
            .parse()
            .with_context(|| format!("Failed to parse `{}`", y))?;

        return Ok(Instruction::RevPos { x, y });
    }

    if let Some(instr) = instruction.strip_prefix("move position ") {
        let (x, y) = instr
            .split_once(" to position ")
            .ok_or_else(|| anyhow!("Bad instruction `{}`", instruction))?;
        let x = x
            .parse()
            .with_context(|| format!("Failed to parse `{}`", x))?;
        let y = y
            .parse()
            .with_context(|| format!("Failed to parse `{}`", y))?;

        return Ok(Instruction::MovePos { x, y });
    }

    Err(anyhow!("Instruction `{}` not recognized", instruction))
}

fn parse_input(input: impl BufRead) -> Result<Program> {
    let instructions: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(no, line)| {
            let no = no + 1;
            let line = line.with_context(|| format!("Failed to read line {}", no))?;
            let line = line.trim();

            parse_instruction(line).with_context(|| format!("Failed to parse line {}", no))
        })
        .collect::<Result<_>>()?;

    Ok(Program::new(instructions))
}

fn read_input(path: impl AsRef<Path>) -> Result<Program> {
    let path = path.as_ref();
    let input = BufReader::new(
        File::open(path).with_context(|| format!("Failed to open file `{}`", path.display()))?,
    );
    parse_input(input).with_context(|| format!("Failed to read file `{}`", path.display()))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::parse_input;

    const SAMPLE_INPUT: &str = "\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    #[test]
    fn sample_input() {
        let program = parse_input(Cursor::new(SAMPLE_INPUT)).expect("Sample input parses");
        let output = program.run("abcde").expect("Sample input runs");
        assert_eq!(output, "decab", "Sample input generates correct output");
    }

    #[test]
    fn reverse_sample_input() {
        let program = parse_input(Cursor::new(SAMPLE_INPUT)).expect("Sample input parses");
        let output = program.reverse("decab").expect("Sample input runs");
        assert_eq!(output, "abcde", "Sample input generates correct output");
    }
}
