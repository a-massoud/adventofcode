// Very suboptimal, but it works!

mod parser;
mod solver;

use std::{collections::HashMap, env, fs::File, io::BufReader, path::Path};

use anyhow::{Context, Result, anyhow, bail};

use crate::solver::{Floor, Generator, Item, Microchip};

fn main() -> Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("no input provided"))?;
    let (floors, ids) = read_input(input_path).context("reading input")?;

    println!("=== Part 1 ===");
    let min_steps = solver::get_min_steps(&floors).ok_or(anyhow!("no solution found"))?;
    println!("Minimum steps: {}", min_steps);
    println!();

    println!("=== Part 2 ===");
    let (floors, _ids) = add_new_elements(floors, ids).context(anyhow!("adding new elements"))?;
    let min_steps = solver::get_min_steps(&floors).ok_or(anyhow!("no solution found"))?;
    println!("Minimum steps: {}", min_steps);

    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> Result<([Floor; 4], HashMap<String, u32>)> {
    let input = BufReader::new(
        File::open(&path).with_context(|| format!("opening file `{}`", path.as_ref().display()))?,
    );
    parser::parse_input(input)
}

fn add_new_elements(
    mut floors: [Floor; 4],
    mut ids: HashMap<String, u32>,
) -> Result<([Floor; 4], HashMap<String, u32>)> {
    if ids.insert("elerium".to_owned(), ids.len() as u32).is_some()
        || ids
            .insert("dilithium".to_owned(), ids.len() as u32)
            .is_some()
    {
        bail!("elerium or dilithium already are elements");
    }
    let mut floor0_items = floors[0].items();
    floor0_items.push(Item::Generator(Generator {
        element: ids["elerium"],
    }));
    floor0_items.push(Item::Microchip(Microchip {
        element: ids["elerium"],
    }));
    floor0_items.push(Item::Generator(Generator {
        element: ids["dilithium"],
    }));
    floor0_items.push(Item::Microchip(Microchip {
        element: ids["dilithium"],
    }));
    floors[0] = Floor::new(floor0_items).context("creating floor with new items")?;

    Ok((floors, ids))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{parser, solver};

    const SAMPLE_INPUT: &str = "\
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test]
    fn sample_input() {
        let (input, _ids) =
            parser::parse_input(Cursor::new(SAMPLE_INPUT)).expect("failed to parse input");
        let min_steps = solver::get_min_steps(&input).expect("failed to find minimum steps");
        assert_eq!(min_steps, 11);
    }
}
