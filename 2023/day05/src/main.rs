use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use seedmap::{SeedFunctionMap, SeedFunctionRange, SeedMap};

mod seedmap;

// This was pretty fun. Had some trouble with my own brain parsing the input, but once I got over
// those hurdles writing the code wasn't hard, I just had to understand what I was doing.
// It is *highly* recommended that to run this to do so in release mode. It is not exactly fast in
// debug mode. Release takes ~6sec total on my pretty powerful machine. It also just... sometimes
// produces the wrong answer? I wish I knew what that was about. Running it a few times should
// yield 2 different answers for part 2, one of them is correct.

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return;
    }

    let input = read_input(&args[1]);

    println!(
        "Part 1 result: {}",
        input.seeds.iter().map(|x| input.run(*x)).min().unwrap()
    );

    println!(
        "Part 2 result: {}",
        input.run(
            (0..=i64::MAX)
                .map(|x| input.rev_run(x))
                .filter(|x| input.is_in_seed_range(*x))
                .next()
                .unwrap()
        )
    );
}

fn parse_block<I>(lines: &mut I) -> SeedFunctionMap
where
    I: Iterator<Item = String>,
{
    lines.next();

    let mut map = HashSet::new();
    let mut line = lines.next().unwrap();
    while line != "" {
        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        line = match lines.next() {
            Some(x) => x,
            None => String::new(),
        };

        if nums.len() != 3 {
            panic!("Parsing `{}` failed", line);
        }

        map.insert(SeedFunctionRange::new(nums[1], nums[0] - nums[1], nums[2]));
    }
    SeedFunctionMap::new(map)
}

fn read_input(file_name: &str) -> SeedMap {
    let input = BufReader::new(File::open(file_name).unwrap());

    let mut lines = input.lines().filter_map(|x| x.ok());

    let seed_line = lines.next().unwrap();
    let seeds: Vec<i64> = seed_line[seed_line.find(':').unwrap()..]
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    lines.next();

    let seed_soil_map = parse_block(&mut lines);
    let soil_fertilizer_map = parse_block(&mut lines);
    let fertilizer_water_map = parse_block(&mut lines);
    let water_light_map = parse_block(&mut lines);
    let light_temp_map = parse_block(&mut lines);
    let temp_humidity_map = parse_block(&mut lines);
    let humidity_loc_map = parse_block(&mut lines);

    SeedMap {
        seeds,
        seed_soil_map,
        soil_fertilizer_map,
        fertilizer_water_map,
        water_light_map,
        light_temp_map,
        temp_humidity_map,
        humidity_loc_map,
    }
}
