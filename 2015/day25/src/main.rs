use std::env;

const INITIAL: u64 = 20151125;
const MULT: u64 =  252533;
const MOD: u64 = 33554393;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <row> <column>", args[0]);
        return;
    }

    let row: u64 = match args[1].parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse row: {}", e);
            return;
        }
    };

    let col: u64 = match args[2].parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse col: {}", e);
            return;
        }
    };

    if row == 0 || col == 0 {
        eprintln!("row and col must be nonzero");
        return;
    }

    let row = row - 1;
    let col = col - 1;

    println!("Part 1: {}", get_code(row, col));
}

fn get_code(row: u64, col: u64) -> u64 {
    let k = row + col;
    let n_before = k * (k + 1) / 2; // number of elements before the diagonal we are on
    let n = n_before + col;

    let mut v = INITIAL;
    for _ in 0..n {
        v = (v * MULT) % MOD;
    }

    v
}
