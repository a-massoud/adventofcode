use std::{env, fs::File, io::BufReader, process::ExitCode};

mod day8;

fn main() -> ExitCode {
    let mut args = env::args();

    let program_name = args.next().unwrap_or("<unknown>".to_owned());

    let Some(input_path) = args.next() else {
        eprintln!("Usage: {} <input file>", program_name);
        return ExitCode::FAILURE;
    };

    let input = BufReader::new(match File::open(&input_path) {
        Ok(v) => {v},
        Err(e) => {
            eprintln!("Failed to open `{}`: {}", input_path, e);
            return ExitCode::FAILURE;
        },
    });

    let program = match day8::read_program(input) {
        Ok(v) => {v},
        Err(e) => {
            eprintln!("Failed to read from `{}`: {}", input_path, e);
            return ExitCode::FAILURE;
        },
    };

    let (final_mem, max_mem) = program.run();

    println!("=== Part 1 ===");

    let Some((max_reg, max_val)) = final_mem.into_iter().max_by_key(|(_, v)| *v) else {
        eprintln!("No registers were set in program");
        return ExitCode::FAILURE;
    };

    println!("Maximum: register {} has value {}", max_reg, max_val);

    println!("\n=== Part 2 ===");

    let Some((max_reg, max_val)) = max_mem.into_iter().max_by_key(|(_, v)| *v) else {
        eprintln!("No registers were set in program");
        return ExitCode::FAILURE;
    };

    println!("Maximum: register {} had value {}", max_reg, max_val);

    ExitCode::SUCCESS
}
