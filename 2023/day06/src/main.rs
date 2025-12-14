use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// I got confused by how to deal with integer results to the quadratic formula, and then I realized
// I could just round weird and it was fine

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return;
    }

    let input = read_input(&args[1]);

    // the formula is weird because it has to account for if the intersection is whole numbers
    println!(
        "Part 1 results: {}",
        input.iter().fold(1i64, |acc, (time, dist)| {
            let time = *time as f64;
            let dist = *dist as f64;
            acc * (((time + (time.powf(2.0) - 4.0 * dist).sqrt()) / 2.0).ceil() as i64
                - ((time - (time.powf(2.0) - 4.0 * dist).sqrt()) / 2.0).floor() as i64
                - 1)
        })
    );

    println!("Part 2 results: {}", {
        let (time, dist): (Vec<i64>, Vec<i64>) = input.clone().into_iter().unzip();
        let time: f64 = time
            .iter()
            .fold(String::new(), |acc, val| acc + &val.to_string())
            .parse()
            .unwrap();
        let dist: f64 = dist
            .iter()
            .fold(String::new(), |acc, val| acc + &val.to_string())
            .parse()
            .unwrap();
        ((time + (time.powf(2.0) - 4.0 * dist).sqrt()) / 2.0).ceil() as i64
            - ((time - (time.powf(2.0) - 4.0 * dist).sqrt()) / 2.0).floor() as i64
            - 1
    });
}

fn read_input(fname: &str) -> Vec<(i64, i64)> {
    let mut lines = BufReader::new(File::open(fname).unwrap()).lines();

    let times = lines.next().unwrap().unwrap();
    let distances = lines.next().unwrap().unwrap();

    // it'll just automatically skip the label because it won't parse
    let times = times
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok());
    let distances = distances
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok());

    times.zip(distances).collect()
}
