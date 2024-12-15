use anyhow::{anyhow, bail};
use std::collections::{HashSet, VecDeque};
use std::{env, fs};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0]);
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input)?;

    let (part1, part2) = score_map(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut grid = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for ch in line.chars() {
            row.push(
                ch.to_digit(10)
                    .ok_or(anyhow!("non-digit in input"))?
                    .try_into()
                    .expect("single-digit base-10 number larger than u8"),
            );
        }

        grid.push(row);
    }

    Ok(grid)
}

fn score_trailhead(input: &Vec<Vec<u8>>, trailhead: &Point) -> (i64, i64) {
    const DIRS: [Point; 4] = [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
    ];

    if input.get(trailhead.y as usize).is_none()
        || input[trailhead.y as usize].get(trailhead.x as usize) != Some(&0)
    {
        return (0, 0);
    }

    let mut reachable_ends = HashSet::new();
    let mut rating = 0;
    let mut queue = VecDeque::new();
    queue.push_back(*trailhead);
    while !queue.is_empty() {
        let pt = queue.pop_front().expect("nonempty queue has no elements");

        if input[pt.y as usize][pt.x as usize] == 9 {
            reachable_ends.insert(pt);
            rating += 1;
            continue;
        }

        for dir in DIRS {
            if input.get((pt.y + dir.y) as usize).is_some_and(|row| {
                row.get((pt.x + dir.x) as usize)
                    .is_some_and(|&x| x == input[pt.y as usize][pt.x as usize] + 1)
            }) {
                queue.push_back(Point {
                    x: pt.x + dir.x,
                    y: pt.y + dir.y,
                });
            }
        }
    }

    (reachable_ends.len() as i64, rating)
}

fn score_map(input: &Vec<Vec<u8>>) -> (i64, i64) {
    input
        .iter()
        .enumerate()
        .fold((0i64, 0i64), |map_acc, (y, row)| {
            let row_score = row
                .iter()
                .enumerate()
                .fold((0i64, 0i64), |row_acc, (x, _)| {
                    let tr_score = score_trailhead(
                        input,
                        &Point {
                            x: x as isize,
                            y: y as isize,
                        },
                    );
                    (row_acc.0 + tr_score.0, row_acc.1 + tr_score.1)
                });
            (map_acc.0 + row_score.0, map_acc.1 + row_score.1)
        })
}
