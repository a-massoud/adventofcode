use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use game::{Game, Round};

mod game;

// This was pretty straightforward all around. Nothing really importantly difficult or particularly
// easy either.

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return;
    }

    let input = read_input(&args[1]);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn read_input(file_name: &str) -> Vec<Game> {
    let mut result = Vec::new();

    let mut input = BufReader::new(File::open(file_name).unwrap());
    let mut line = String::new();
    while {
        line.clear();
        input.read_line(&mut line).unwrap() != 0
    } {
        let mut game = Game::new();
        let rounds = (&line[line.find(": ").unwrap() + 2..]).trim().split("; ");
        for round in rounds {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let items = round.split(", ");
            for item in items {
                if item.ends_with("red") {
                    r = (&item[0..(item.find(' ').unwrap())]).parse().unwrap();
                } else if item.ends_with("green") {
                    g = (&item[0..(item.find(' ').unwrap())]).parse().unwrap();
                } else if item.ends_with("blue") {
                    b = (&item[0..(item.find(' ').unwrap())]).parse().unwrap();
                }
            }
            game.rounds.push(Round::new(r, g, b));
        }
        result.push(game);
    }

    result
}

fn part1(input: &Vec<Game>) -> i32 {
    input.iter().enumerate().fold(0, |acc, (i, game)| -> i32 {
        if game.fits(12, 13, 14) {
            acc + (i as i32) + 1
        } else {
            acc
        }
    })
}

fn part2(input: &Vec<Game>) -> i32 {
    input.iter().map(|x| x.min_power()).sum()
}
