mod day5;

use std::{env, process::ExitCode};

use crate::day5::read_offsets;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return ExitCode::FAILURE;
    }

    let offsets = match read_offsets(&args[1]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read input: {:#}", e);
            return ExitCode::FAILURE;
        }
    };

    let n_steps = offsets.count_steps();
    let n_steps_with_decrease_rule = offsets.count_steps_with_decrease_rule();

    println!(
        "\
Part 1
======
Steps: {}

Part 2
======
Steps: {}",
        n_steps, n_steps_with_decrease_rule
    );

    ExitCode::SUCCESS
}
