// I'm actually happy with this one. It's ugly but I like it.

use anyhow::{bail, Ok};
use std::collections::HashSet;
use std::{env, fs};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: {} <input file>", args[0])
    }

    let input = fs::read_to_string(&args[1])?;
    let input = parse_input(&input);

    println!(
        "Part 1: {}",
        input.iter().map(|r| perim_fence_value(r)).sum::<i32>()
    );
    println!(
        "Part 2: {}",
        input.iter().map(|r| side_fence_value(r)).sum::<i32>()
    );

    Ok(())
}

fn parse_input(input: &str) -> Vec<HashSet<Point>> {
    const DIRS: [Point; 4] = [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
    ];
    let mut regions = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();

    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if visited.contains(&Point {
                x: j as i32,
                y: i as i32,
            }) {
                continue;
            }
            visited.insert(Point {
                x: j as i32,
                y: i as i32,
            });

            let mut region = HashSet::new();
            let mut queue = Vec::new();
            queue.push(Point {
                x: j as i32,
                y: i as i32,
            });
            while !queue.is_empty() {
                let pt = queue.pop().expect("nonempty queue contains no elements");
                region.insert(pt);

                for dir in DIRS {
                    let npt = Point {
                        x: pt.x + dir.x,
                        y: pt.y + dir.y,
                    };
                    let row = input.get(npt.y as usize);
                    let Some(row) = row else { continue };
                    if row.get(npt.x as usize) == Some(&ch) && !visited.contains(&npt) {
                        visited.insert(npt);
                        queue.push(Point {
                            x: pt.x + dir.x,
                            y: pt.y + dir.y,
                        });
                    }
                }
            }
            regions.push(region);
        }
    }

    regions
}

fn perim_fence_value(region: &HashSet<Point>) -> i32 {
    const DIRS: [Point; 4] = [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
    ];
    let mut perim = 0i32;

    for pt in region {
        for dir in DIRS {
            if !region.contains(&Point {
                x: pt.x + dir.x,
                y: pt.y + dir.y,
            }) {
                perim += 1;
            }
        }
    }

    perim * region.len() as i32
}

fn side_fence_value(region: &HashSet<Point>) -> i32 {
    const DIRS: [Point; 4] = [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
    ];
    let mut sides = 0i32;
    let mut visited = HashSet::new();

    for pt in region {
        for dir in DIRS {
            if !region.contains(&Point {
                x: pt.x + dir.x,
                y: pt.y + dir.y,
            }) && !visited.contains(&(*pt, dir))
            {
                visited.insert((*pt, dir));
                let rot90 = Point {
                    x: -dir.y,
                    y: dir.x,
                };
                let rotn90 = Point {
                    x: dir.y,
                    y: -dir.x,
                };
                for i in 1.. {
                    let prot90 = Point {
                        x: pt.x + rot90.x * i,
                        y: pt.y + rot90.y * i,
                    };
                    if !region.contains(&prot90)
                        || region.contains(&Point {
                            x: prot90.x + dir.x,
                            y: prot90.y + dir.y,
                        })
                        || visited.contains(&(prot90, dir))
                    {
                        break;
                    }
                    visited.insert((prot90, dir));
                }
                for i in 1.. {
                    let protn90 = Point {
                        x: pt.x + rotn90.x * i,
                        y: pt.y + rotn90.y * i,
                    };
                    if !region.contains(&protn90)
                        || region.contains(&Point {
                            x: protn90.x + dir.x,
                            y: protn90.y + dir.y,
                        })
                        || visited.contains(&(protn90, dir))
                    {
                        break;
                    }
                    visited.insert((protn90, dir));
                }
                sides += 1;
            }
        }
    }

    sides * region.len() as i32
}
