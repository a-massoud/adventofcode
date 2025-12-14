use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use std::{
    array, env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use z3::{Optimize, SatResult, ast::Int};

fn main() -> anyhow::Result<()> {
    let ingredients = read_input(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;

    let (p1, p2) = optimize_cookies(&ingredients, 100);
    println!("Part 1: {}", p1);

    println!("Part 2: {}", p2);

    Ok(())
}

type Ingredient = [i64; 5];

fn read_input(input: impl AsRef<Path>) -> anyhow::Result<Vec<Ingredient>> {
    let re = Regex::new(r"^\S+: capacity ([-+]?\d+), durability ([-+]?\d+), flavor ([-+]?\d+), texture ([-+]?\d+), calories ([-+]?\d+)$").unwrap();

    let reader = BufReader::new(File::open(input)?);

    reader
        .lines()
        .map(|line| {
            let line = line?;
            let (_, v): (_, [_; 5]) = re
                .captures(&line)
                .ok_or(anyhow!("bad line `{}`", line))?
                .extract();

            let ingredient: Ingredient = v
                .iter()
                .map(|i| i.parse())
                .process_results(|it| it.collect_array().expect("regex matches five elements"))?;

            Ok(ingredient)
        })
        .collect()
}

fn optimize_cookies(ingredients: &[Ingredient], budget: i64) -> (i64, i64) {
    if ingredients.len() == 0 {
        return (0, 0);
    }

    let solver = Optimize::new();

    let amts: Vec<_> = (0..ingredients.len())
        .map(|i| Int::fresh_const(&format!("i_{}", i)))
        .collect();

    solver.assert(&amts.iter().sum::<Int>().eq(budget));

    amts.iter().for_each(|v| solver.assert(&v.ge(0)));
    solver.maximize(
        &amts
            .iter()
            .zip(ingredients.iter())
            .map(|(amt, vals)| vals.iter().map(move |&i| amt * i))
            .fold(
                array::from_fn::<Int, 4, _>(|_| Int::from_i64(0)),
                |acc, v| {
                    acc.iter()
                        .zip(v)
                        .map(|(a, b)| a + b)
                        .collect_array()
                        .unwrap()
                },
            )
            .into_iter()
            .fold(Int::from_i64(1), |acc, b| {
                let prod = &acc * &b;
                b.gt(0).ite(&prod, &Int::from_i64(0))
            }),
    );

    solver.push();
    let p1 = if solver.check(&[]) == SatResult::Sat {
        let model = solver.get_model().unwrap();
        let amts: Vec<_> = amts
            .iter()
            .map(|i| model.eval(i, false).unwrap().as_i64().unwrap())
            .collect();

        amts.iter()
            .zip(ingredients.iter())
            .map(|(amt, ingredient)| ingredient.iter().map(move |i| amt * i))
            .fold([0; 4], |acc, v| {
                acc.iter()
                    .zip(v)
                    .map(|(a, b)| a + b)
                    .collect_array()
                    .unwrap()
            })
            .iter()
            .fold(1, |acc, &v| acc * v.max(0))
    } else {
        0
    };
    solver.pop();

    solver.push();
    solver.assert(
        &amts
            .iter()
            .zip(ingredients.iter())
            .map(|(amt, ing)| amt * ing[4])
            .sum::<Int>()
            .eq(500),
    );
    let p2 = if solver.check(&[]) == SatResult::Sat {
        let model = solver.get_model().unwrap();
        let amts: Vec<_> = amts
            .iter()
            .map(|i| model.eval(i, false).unwrap().as_i64().unwrap())
            .collect();

        amts.iter()
            .zip(ingredients.iter())
            .map(|(amt, ingredient)| ingredient.iter().map(move |i| amt * i))
            .fold([0; 4], |acc, v| {
                acc.iter()
                    .zip(v)
                    .map(|(a, b)| a + b)
                    .collect_array()
                    .unwrap()
            })
            .iter()
            .fold(1, |acc, &v| acc * v.max(0))
    } else {
        0
    };
    solver.pop();

    (p1, p2)
}
