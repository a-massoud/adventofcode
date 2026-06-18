// Nice and simple

use std::{
    collections::{HashMap, HashSet, VecDeque, hash_map},
    env,
};

use anyhow::{Context, Result, anyhow};

fn main() -> Result<()> {
    let favorite_num = env::args()
        .nth(1)
        .ok_or(anyhow!("no argument provided"))?
        .parse()
        .context("failed to parse argument as i64")?;

    println!("===Part 1===");
    let shortest_path = find_shortest_path(Point(1, 1), Point(31, 39), favorite_num)
        .ok_or(anyhow!("path not found"))?;
    println!("Number of steps: {}", shortest_path.len() - 1);
    println!();

    println!("===Part 2===");
    let n_locations = count_reachable_locations(Point(1, 1), 50, favorite_num);
    println!("Number of locations: {}", n_locations);

    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i64, i64);

impl Point {
    fn is_wall(&self, favorite_num: i64) -> bool {
        let v = self.0 * self.0
            + 3 * self.0
            + 2 * self.0 * self.1
            + self.1
            + self.1 * self.1
            + favorite_num;
        let count = v.count_ones();
        count % 2 == 1
    }
}

fn find_shortest_path(start: Point, end: Point, favorite_num: i64) -> Option<Vec<Point>> {
    if start.is_wall(favorite_num) || end.is_wall(favorite_num) {
        return None;
    }

    if start == end {
        return Some(vec![start]);
    }

    let mut queue = VecDeque::from([start]);
    let mut parents = HashMap::from([(start, start)]);

    while let Some(r) = queue.pop_front() {
        for Point(dx, dy) in [Point(0, -1), Point(0, 1), Point(1, 0), Point(-1, 0)] {
            let nr = Point(r.0 + dx, r.1 + dy);

            if nr.0 < 0 || nr.1 < 0 || nr.is_wall(favorite_num) {
                continue;
            }

            match parents.entry(nr) {
                hash_map::Entry::Occupied(_) => {
                    continue;
                }
                hash_map::Entry::Vacant(v) => {
                    v.insert(r);
                }
            }

            if nr == end {
                let mut ret = vec![nr];

                while let Some(&prev) = ret.last()
                    && let Some(&v) = parents.get(&prev)
                    && v != prev
                {
                    ret.push(v);
                }
                ret.reverse();

                return Some(ret);
            }

            queue.push_back(nr);
        }
    }

    None
}

fn count_reachable_locations(start: Point, n_steps: usize, favorite_num: i64) -> usize {
    if start.is_wall(favorite_num) {
        return 0;
    }

    let mut queue = VecDeque::from([(0, start)]);
    let mut visited = HashSet::from([start]);

    while let Some((n, Point(x, y))) = queue.pop_front() {
        for Point(dx, dy) in [Point(0, -1), Point(0, 1), Point(1, 0), Point(-1, 0)] {
            let nr = Point(x + dx, y + dy);

            if nr.0 < 0 || nr.1 < 0 || nr.is_wall(favorite_num) || n >= n_steps {
                continue;
            }

            if !visited.insert(nr) {
                continue;
            }

            queue.push_back((n + 1, nr));
        }
    }

    visited.len()
}

#[cfg(test)]
mod test {
    use crate::{Point, count_reachable_locations, find_shortest_path};

    #[test]
    fn sample_input_p1() {
        let path = find_shortest_path(Point(1, 1), Point(7, 4), 10);
        assert_eq!(
            path,
            Some(vec![
                Point(1, 1),
                Point(1, 2),
                Point(2, 2),
                Point(3, 2),
                Point(3, 3),
                Point(3, 4),
                Point(4, 4),
                Point(4, 5),
                Point(5, 5),
                Point(6, 5),
                Point(6, 4),
                Point(7, 4)
            ])
        );
    }

    #[test]
    fn sample_input_p2() {
        assert_eq!(count_reachable_locations(Point(1, 1), 50, 10), 151);
    }
}
