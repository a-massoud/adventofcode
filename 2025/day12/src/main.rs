// I'm much more happy with this solution. Still mildly annoyed that I didn't need to do this,
// but I'm happy with a solution that works on the sample input.
// The complicated part is largely modified from https://github.com/nickponline/aoc-2025/blob/main/12/main.py

use anyhow::{Context, anyhow, bail};
use regex::Regex;
use std::cell::LazyCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::{env, fs, mem};
use z3::ast::Bool;
use z3::{SatResult, Solver};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let (blocks, bins) = parse_input(&input)?;

    let p1 = bins
        .iter()
        .enumerate()
        .inspect(|(i, _)| println!("{}/{}", i, bins.len()))
        .map(|(i, bin)| (i, blocks_fit_in_bin((bin.0, &bin.1), &blocks)))
        .try_fold(0usize, |acc, (i, r)| {
            r.map(|r| acc + r.then_some(1).unwrap_or(0))
                .with_context(|| format!("in block {}", i))
        })?;

    println!("{} bins are possible", p1);

    Ok(())
}

fn get_dihedral_orbit(set: &HashSet<(i64, i64)>) -> Vec<HashSet<(i64, i64)>> {
    let mut s: Vec<_> = (0..4)
        .flat_map(|r| {
            let rset: BTreeSet<_> = set
                .iter()
                .map(|&(mut x, mut y)| {
                    for _ in 0..r {
                        y = -y;
                        mem::swap(&mut x, &mut y);
                    }
                    (x, y)
                })
                .collect();
            let fset: BTreeSet<_> = rset.iter().map(|&(x, y)| (-x, y)).collect();

            [rset, fset].into_iter()
        })
        .collect();
    s.sort_unstable();
    s.dedup();
    s.into_iter().map(|i| i.into_iter().collect()).collect()
}

fn parse_input(
    input: &str,
) -> anyhow::Result<(
    Vec<Vec<HashSet<(i64, i64)>>>,
    Vec<((usize, usize), Vec<usize>)>,
)> {
    let lines: Vec<_> = input.lines().collect();

    let split: Vec<_> = lines.split(|s| s.trim().is_empty()).collect();

    if split.len() < 2 {
        bail!("no double newlines");
    }

    let blocks: Vec<_> = split[..split.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, &chunk)| {
            if chunk.len() != 4 {
                bail!("chunk must be 3x3");
            }

            if chunk[0].trim() != format!("{}:", i) {
                bail!("first line of chunk {} malformatted", i);
            }

            let set = chunk[1..]
                .iter()
                .enumerate()
                .flat_map(|(y, &line)| {
                    line.bytes().enumerate().flat_map(move |(x, ch)| match ch {
                        b'#' => Some(Ok((x as i64 - 1, y as i64 - 1))),
                        b'.' => None,
                        _ => Some(Err(anyhow!("bad char `{}`", ch))),
                    })
                })
                .collect::<anyhow::Result<_>>()?;

            Ok(get_dihedral_orbit(&set))
        })
        .collect::<anyhow::Result<_>>()?;

    let last = split[split.len() - 1];
    const RE: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"^([0-9]+)x([0-9]+):((?:\s[0-9]+)+)$").unwrap());

    let grids = last
        .iter()
        .map(|s| -> anyhow::Result<_> {
            let (_, [a, b, c]) = RE.captures(s).ok_or(anyhow!("bad line `{}`", s))?.extract();

            let dim: usize = a.parse()?;
            let dim2: usize = b.parse()?;

            let v = c
                .split_whitespace()
                .map(str::parse::<usize>)
                .collect::<Result<Vec<usize>, _>>()?;

            if v.len() != (&blocks).len() {
                bail!("line `{}` had incorrect number of constraints", s);
            }

            Ok(((dim, dim2), v))
        })
        .collect::<anyhow::Result<_>>()?;

    Ok((blocks, grids))
}

fn get_placements(
    block: &HashSet<(i64, i64)>,
    binwidth: usize,
    binheight: usize,
) -> Vec<(usize, usize)> {
    if block.len() == 0 {
        return Vec::new();
    }

    let (xmin, xmax, ymin, ymax) = block.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |(xmin, xmax, ymin, ymax), &(x, y)| (xmin.min(x), xmax.max(x), ymin.min(y), ymax.max(y)),
    );

    let mut r = Vec::new();
    for dx in 0..binwidth {
        for dy in 0..binheight {
            if dx as i64 + xmin >= 0
                && dx as i64 + xmax < binwidth as i64
                && dy as i64 + ymin >= 0
                && dy as i64 + ymax < binheight as i64
            {
                r.push((dx, dy))
            }
        }
    }

    r
}

fn blocks_fit_in_bin(
    bin: ((usize, usize), &[usize]),
    blocks: &[impl AsRef<[HashSet<(i64, i64)>]>],
) -> anyhow::Result<bool> {
    let ((binwidth, binheight), requirements) = bin;
    if binwidth * binheight
        < blocks
            .iter()
            .enumerate()
            .flat_map(|(i, x)| {
                let x = x.as_ref();
                x.first().map(|set| set.len() * requirements[i])
            })
            .sum()
    {
        // physically not enough space for blocks
        return Ok(false);
    }

    let (maxwidth, maxheight) = blocks
        .iter()
        .flat_map(|block| {
            block.as_ref().first().map(|var| {
                var.iter().fold(
                    (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
                    |(xmin, xmax, ymin, ymax), &(x, y)| {
                        (xmin.min(x), xmax.max(x), ymin.min(y), ymax.max(y))
                    },
                )
            })
        })
        .fold(
            (i64::MIN, i64::MIN),
            |(maxwidth, maxheight), (xmin, xmax, ymin, ymax)| {
                (maxwidth.max(xmax - xmin + 1), maxheight.max(ymax - ymin + 1))
            },
        );
    if (binwidth as i64 / maxwidth) * (binheight as i64 / maxheight) > blocks.len() as i64 {
        // naïve packing works
        return Ok(true);
    }

    let solver = Solver::new();

    let mut placements = Vec::new();
    let mut cell_to_placements: HashMap<(i64, i64), Vec<(usize, usize)>> = HashMap::new();

    for (i_block, vars) in blocks.iter().enumerate() {
        let vars = vars.as_ref();
        let mut shape_placements = Vec::new();

        for (i_var, shape) in vars.iter().enumerate() {
            let positions = get_placements(shape, binwidth, binheight);

            for (dx, dy) in positions {
                let var = Bool::fresh_const(&format!("p_{}_{}_{}_{}", i_block, i_var, dx, dy));
                let covered: HashSet<_> = shape
                    .iter()
                    .map(|(x, y)| (dx as i64 + x, dy as i64 + y))
                    .collect();
                shape_placements.push(var);

                for cell in covered {
                    cell_to_placements
                        .entry(cell)
                        .and_modify(|v| v.push((placements.len(), shape_placements.len() - 1)))
                        .or_insert(vec![(placements.len(), shape_placements.len() - 1)]);
                }
            }
        }

        placements.push(shape_placements);
    }

    // constraint 1: exactly requirements[i] placements for each shape type
    for (i, &count) in requirements.iter().enumerate() {
        let vars_for_shape = &placements[i];
        if count > 0 {
            if vars_for_shape.len() < count {
                return Ok(false);
            }
            solver.assert(Bool::pb_eq(
                &vars_for_shape.iter().map(|b| (b, 1)).collect::<Vec<_>>(),
                count as i32,
            ))
        } else {
            for var in vars_for_shape {
                solver.assert(var.eq(Bool::from_bool(false)));
            }
        }
    }

    // constraint 2: each cell has at most one placement
    for (_, vars) in &cell_to_placements {
        let vars: Vec<_> = vars.iter().map(|&(i, j)| (&placements[i][j], 1)).collect();
        solver.assert(Bool::pb_le(&vars, 1));
    }

    println!("Solving...");
    match solver.check() {
        SatResult::Unsat => Ok(false),
        SatResult::Unknown => bail!("failed to determine system"),
        SatResult::Sat => Ok(true),
    }
}
