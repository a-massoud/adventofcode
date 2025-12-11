// I took a while to figure out that the question meant 1000 pairs,
// regardless of whether they work or not; made it eventually though.
// Otherwise fun; brute force for the win!

use anyhow::{Context, anyhow, bail};
use nalgebra::allocator::Allocator;
use nalgebra::{
    DMatrix, DefaultAllocator, Dim, Matrix, Owned, Scalar, SquareMatrix, Storage, StorageMut,
    Vector3,
};
use num::Zero;
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{HashSet, VecDeque};
use std::ops::Mul;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let (points, adj) = parse_input(&input)?;

    let p1 = find_clique_sizes(&connect_first_n(&adj, 1000))
        .iter()
        .take(3)
        .fold(1usize, usize::mul);
    println!("Part 1: {}", p1);

    let p2 = get_critical_2(&adj);
    let p2 = points[p2.1].x * points[p2.0].x;
    print!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Vector3<i64>>, DMatrix<f64>)> {
    let points: Vec<_> = input
        .lines()
        .map(|line| {
            let s: Vec<_> = line.split(',').collect();
            if s.len() != 3 {
                bail!("bad line `{}`: wrong components", line);
            }

            let s: Vec<i64> = s
                .iter()
                .map(|i| {
                    i.parse()
                        .with_context(|| format!("`{}` in line `{}`", i, line))
                })
                .collect::<anyhow::Result<_>>()?;
            Ok(Vector3::new(s[0], s[1], s[2]))
        })
        .collect::<anyhow::Result<_>>()?;

    let mut adj = DMatrix::zeros(points.len(), points.len());
    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().enumerate() {
            adj[(i, j)] = (a - b).cast::<f64>().norm();
        }
    }

    Ok((points, adj))
}

fn are_points_connected<T, R, S>(i: usize, j: usize, adj: &SquareMatrix<T, R, S>) -> bool
where
    T: Zero + PartialEq + Scalar,
    R: Dim,
    S: Storage<T, R, R>,
{
    let mut visited = HashSet::new();
    let mut q = VecDeque::from([i]);

    while let Some(k) = q.pop_front() {
        if k == j {
            return true;
        }
        if !visited.insert(k) {
            continue;
        }
        for (l, len) in adj.row(k).iter().enumerate() {
            if !len.is_zero() {
                q.push_back(l);
            }
        }
    }

    false
}

fn connect_first_n<T, R, S>(
    dists: &SquareMatrix<T, R, S>,
    n: usize,
) -> SquareMatrix<i8, R, Owned<i8, R, R>>
where
    T: PartialOrd + Scalar,
    R: Dim,
    S: Storage<T, R, R>,
    DefaultAllocator: Allocator<R, R>,
    <DefaultAllocator as Allocator<R, R>>::Buffer<i8>: StorageMut<i8, R, R>,
{
    assert_eq!(dists.nrows(), dists.ncols());
    let dim = dists.nrows();
    let mut adj = Matrix::zeros_generic(R::from_usize(dim), R::from_usize(dim));
    let visited = RefCell::new(HashSet::new());

    let mut k = 0;

    let mut sorted_edges: Vec<_> = (0..dists.nrows())
        .flat_map(|i| (0..i).map(move |j| (i, j, &dists[(i, j)])))
        .collect();
    sorted_edges.sort_by(|(_, _, n1), (_, _, n2)| n1.partial_cmp(n2).unwrap_or(Ordering::Equal));

    for (i, j, _) in sorted_edges
        .into_iter()
        .filter(|(i, j, _)| !visited.borrow().contains(&(*i, *j)))
    {
        if k >= n {
            break;
        }
        k += 1;

        let mut visited = visited.borrow_mut();
        visited.insert((i, j));

        if !are_points_connected(i, j, &adj) {
            adj[(i, j)] = 1;
            adj[(j, i)] = 1;
        }
    }

    adj
}

fn find_clique_sizes<T, R, S>(adj: &SquareMatrix<T, R, S>) -> Vec<usize>
where
    T: Zero + Scalar,
    R: Dim,
    S: Storage<T, R, R>,
{
    assert_eq!(adj.nrows(), adj.ncols());
    let dim = adj.nrows();
    let mut r = Vec::new();
    let mut visited = HashSet::new();

    while let Some(i) = (0..dim).filter(|i| !visited.contains(i)).next() {
        let mut clique_size = 0;
        let mut q = VecDeque::new();
        q.push_back(i);
        while let Some(j) = q.pop_front() {
            if !visited.insert(j) {
                continue;
            }

            clique_size += 1;

            for k in (0..dim).filter(|&k| !adj[(j, k)].is_zero()) {
                q.push_back(k);
            }
        }

        r.push(clique_size);
    }

    r.sort_by_key(|&x| Reverse(x));
    r
}

fn get_critical_2<T, R, S>(dists: &SquareMatrix<T, R, S>) -> (usize, usize)
where
    T: PartialOrd + Scalar,
    R: Dim,
    S: Storage<T, R, R>,
    DefaultAllocator: Allocator<R, R>,
    <DefaultAllocator as Allocator<R, R>>::Buffer<i8>: StorageMut<i8, R, R>,
{
    assert_eq!(dists.nrows(), dists.ncols());
    let dim = dists.nrows();
    let mut adj = SquareMatrix::<i8, R, Owned<i8, R, R>>::zeros_generic(
        R::from_usize(dim),
        R::from_usize(dim),
    );
    let visited = RefCell::new(HashSet::new());

    let mut last_conn = (0, 0);

    let mut sorted_edges: Vec<_> = (0..dists.nrows())
        .flat_map(|i| (0..i).map(move |j| (i, j, &dists[(i, j)])))
        .collect();
    sorted_edges.sort_by(|(_, _, n1), (_, _, n2)| n1.partial_cmp(n2).unwrap_or(Ordering::Equal));

    for (i, j, _) in sorted_edges
        .into_iter()
        .filter(|(i, j, _)| !visited.borrow().contains(&(*i, *j)))
    {
        let mut visited = visited.borrow_mut();
        visited.insert((i, j));

        if is_graph_complete(&adj) {
            break;
        }

        if !are_points_connected(i, j, &adj) {
            adj[(i, j)] = 1;
            adj[(j, i)] = 1;
            last_conn = (i, j);
        }
    }

    last_conn
}

fn is_graph_complete<T, R, S>(adj: &SquareMatrix<T, R, S>) -> bool
where
    T: Zero + PartialEq + Scalar,
    R: Dim,
    S: Storage<T, R, R>,
{
    let mut visited = HashSet::new();
    let mut q = VecDeque::from([0]);

    while let Some(k) = q.pop_front() {
        if !visited.insert(k) {
            continue;
        }
        for (l, len) in adj.row(k).iter().enumerate() {
            if *len != T::zero() {
                q.push_back(l);
            }
        }
    }

    visited.len() == adj.nrows()
}
