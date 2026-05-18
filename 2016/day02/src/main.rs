// Fun little problem. I don't know why I decided to encode the entire graph by hand.

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, anyhow};

fn main() -> anyhow::Result<()> {
    let input = env::args_os()
        .nth(1)
        .ok_or(anyhow!("No argument provided"))?;
    let input = read_input(input)?;

    let code = get_code(&input, imagined_keypad);
    println!("Part 1: {}", code.iter().collect::<String>());

    let code = get_code(&input, real_keypad);
    println!("Part 2: {}", code.iter().collect::<String>());

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<Vec<Instruction>>> {
    let input = BufReader::new(File::open(&path).with_context(|| {
        format!(
            "Failed to open `{}` for reading",
            path.as_ref().to_string_lossy()
        )
    })?);

    input
        .lines()
        .map(|line| {
            let line = line.context("Failed to read line")?;

            line.bytes()
                .map(|b| match b {
                    b'U' => Ok(Instruction::Up),
                    b'D' => Ok(Instruction::Down),
                    b'L' => Ok(Instruction::Left),
                    b'R' => Ok(Instruction::Right),
                    _ => Err(anyhow!("bad instruction {}", char::from(b))),
                })
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect()
}

const fn imagined_keypad(current_value: char, instruction: Instruction) -> char {
    match current_value {
        '1' => match instruction {
            Instruction::Up => '1',
            Instruction::Right => '2',
            Instruction::Down => '4',
            Instruction::Left => '1',
        },
        '2' => match instruction {
            Instruction::Up => '2',
            Instruction::Right => '3',
            Instruction::Down => '5',
            Instruction::Left => '1',
        },
        '3' => match instruction {
            Instruction::Up => '3',
            Instruction::Right => '3',
            Instruction::Down => '6',
            Instruction::Left => '2',
        },
        '4' => match instruction {
            Instruction::Up => '1',
            Instruction::Right => '5',
            Instruction::Down => '7',
            Instruction::Left => '4',
        },
        '5' => match instruction {
            Instruction::Up => '2',
            Instruction::Right => '6',
            Instruction::Down => '8',
            Instruction::Left => '4',
        },
        '6' => match instruction {
            Instruction::Up => '3',
            Instruction::Right => '6',
            Instruction::Down => '9',
            Instruction::Left => '5',
        },
        '7' => match instruction {
            Instruction::Up => '4',
            Instruction::Right => '8',
            Instruction::Down => '7',
            Instruction::Left => '7',
        },
        '8' => match instruction {
            Instruction::Up => '5',
            Instruction::Right => '9',
            Instruction::Down => '8',
            Instruction::Left => '7',
        },
        '9' => match instruction {
            Instruction::Up => '6',
            Instruction::Right => '9',
            Instruction::Down => '9',
            Instruction::Left => '8',
        },
        _ => panic!("left the keypad"),
    }
}

const fn real_keypad(current_value: char, instruction: Instruction) -> char {
    match current_value {
        '1' => match instruction {
            Instruction::Up => '1',
            Instruction::Right => '1',
            Instruction::Down => '3',
            Instruction::Left => '1',
        },
        '2' => match instruction {
            Instruction::Up => '2',
            Instruction::Right => '3',
            Instruction::Down => '6',
            Instruction::Left => '2',
        },
        '3' => match instruction {
            Instruction::Up => '1',
            Instruction::Right => '4',
            Instruction::Down => '7',
            Instruction::Left => '2',
        },
        '4' => match instruction {
            Instruction::Up => '4',
            Instruction::Right => '4',
            Instruction::Down => '8',
            Instruction::Left => '3',
        },
        '5' => match instruction {
            Instruction::Up => '5',
            Instruction::Right => '6',
            Instruction::Down => '5',
            Instruction::Left => '5',
        },
        '6' => match instruction {
            Instruction::Up => '2',
            Instruction::Right => '7',
            Instruction::Down => 'A',
            Instruction::Left => '5',
        },
        '7' => match instruction {
            Instruction::Up => '3',
            Instruction::Right => '7',
            Instruction::Down => 'B',
            Instruction::Left => '6',
        },
        '8' => match instruction {
            Instruction::Up => '4',
            Instruction::Right => '9',
            Instruction::Down => 'C',
            Instruction::Left => '7',
        },
        '9' => match instruction {
            Instruction::Up => '9',
            Instruction::Right => '9',
            Instruction::Down => '9',
            Instruction::Left => '8',
        },
        'A' => match instruction {
            Instruction::Up => '6',
            Instruction::Right => 'B',
            Instruction::Down => 'A',
            Instruction::Left => 'A',
        },
        'B' => match instruction {
            Instruction::Up => '7',
            Instruction::Right => 'C',
            Instruction::Down => 'D',
            Instruction::Left => 'A',
        },
        'C' => match instruction {
            Instruction::Up => '8',
            Instruction::Right => 'C',
            Instruction::Down => 'C',
            Instruction::Left => 'B',
        },
        'D' => match instruction {
            Instruction::Up => 'B',
            Instruction::Right => 'D',
            Instruction::Down => 'D',
            Instruction::Left => 'D',
        },
        _ => panic!("left the keypad"),
    }
}

fn get_code(
    input: &[impl AsRef<[Instruction]>],
    next_value: impl Fn(char, Instruction) -> char,
) -> Vec<char> {
    let mut r = Vec::with_capacity(input.len());

    for line in input {
        r.push(
            line.as_ref()
                .iter()
                .fold(*r.last().unwrap_or(&'5'), |v, &i| next_value(v, i)),
        );
    }

    r
}
