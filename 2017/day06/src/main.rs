use std::{env, fs::File, process::ExitCode};

mod day6;

fn main() -> ExitCode {
    let mut args = env::args();
    let program_name = args.next().unwrap_or("<unknown>".to_owned());
    let Some(input_path) = args.next() else {
        eprintln!("Usage: {} <input file>", program_name);
        return ExitCode::FAILURE;
    };

    let input_file = match File::open(&input_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to open `{}`: {}", input_path, e);
            return ExitCode::FAILURE;
        }
    };
    let memory = match day6::read_memory(input_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read `{}`: {}", input_path, e);
            return ExitCode::FAILURE;
        }
    };

    let (step_count, cycle_len) = memory.find_cycle();

    println!(
        "\
=== Part 1 ===
Step count: {}

=== Part 2 ===
Cycle length: {}",
        step_count, cycle_len
    );

    ExitCode::SUCCESS
}
