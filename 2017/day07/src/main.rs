// Just truly, dramatic overkill with way more parsing than necessary but it was fun.

use std::{env, fs::File, io::BufReader, process::ExitCode};

use crate::day7::read_tower;

mod day7;

fn main() -> ExitCode {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "<unknown>".to_owned());

    let Some(input_path) = args.next() else {
        eprintln!("Usage: {} <input file>", program_name);
        return ExitCode::FAILURE;
    };

    let input = BufReader::new(match File::open(&input_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open `{}`: {}", input_path, e);
            return ExitCode::FAILURE;
        }
    });

    let tower = match read_tower(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read `{}`: {}", input_path, e);
            return ExitCode::FAILURE;
        }
    };

    println!("=== Part 1 ===");

    let root = tower.root();

    println!("Root: {}", root);

    println!("\n=== Part 2 ===");

    if let Some((name, weight)) = tower.get_balancing_correction() {
        println!("Correction: {} needs to weigh {}", name, weight);
    } else {
        println!("The tower was already balanced");
    };

    ExitCode::SUCCESS
}
