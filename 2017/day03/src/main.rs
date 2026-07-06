// Fun math puzzle.

use std::{env, fs, num::NonZeroU64, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return ExitCode::FAILURE;
    }

    let input = match fs::read_to_string(&args[1]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let input: u64 = match input.trim().parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let address = match Address::new(input) {
        Some(v) => v,
        None => {
            eprintln!("Address must be nonnegative");
            return ExitCode::FAILURE;
        }
    };

    let dist = address.position().manhattan_norm();

    println!("Part 1");
    println!("======");
    println!("Distance: {}", dist);
    println!();

    let larger_value = find_larger_value(input);

    println!("Part 2");
    println!("======");
    println!("Larger value: {}", larger_value);

    ExitCode::SUCCESS
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    const fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }

    const fn manhattan_norm(self) -> u64 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }

    fn neighbors_iter(self) -> impl Iterator<Item = Self> {
        const DIRECTIONS: [(i64, i64); 8] = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        DIRECTIONS
            .into_iter()
            .map(move |(dx, dy)| Coord::new(self.x + dx, self.y + dy))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Address(NonZeroU64);

impl Address {
    const fn new(value: u64) -> Option<Self> {
        // can't use ? in const context
        if let Some(v) = NonZeroU64::new(value) {
            Some(Address(v))
        } else {
            None
        }
    }

    const fn position(self) -> Coord {
        let v = self.0.get();
        if v == 1 {
            return Coord::new(0, 0);
        }
        let square = ((v - 1).isqrt() - 1) / 2 + 1;

        let min_on_square = (2 * square - 1).pow(2) + 1;
        let n_per_side = 2 * square;

        if v < min_on_square + n_per_side {
            Coord::new(
                square as i64,
                v as i64 - (min_on_square as i64 + n_per_side as i64 / 2 - 1),
            )
        } else if v < min_on_square + 2 * n_per_side {
            Coord::new(
                ((min_on_square + n_per_side) as i64 + n_per_side as i64 / 2 - 1) - v as i64,
                square as i64,
            )
        } else if v < min_on_square + 3 * n_per_side {
            Coord::new(
                -(square as i64),
                ((min_on_square + 2 * n_per_side) as i64 + n_per_side as i64 / 2 - 1) - v as i64,
            )
        } else {
            Coord::new(
                v as i64 - ((min_on_square + 3 * n_per_side) as i64 + n_per_side as i64 / 2 - 1),
                -(square as i64),
            )
        }
    }

    const fn from_coord(coord: Coord) -> Self {
        if coord.x == 0 && coord.y == 0 {
            return Address(NonZeroU64::new(1).unwrap());
        }

        // .max() is not const
        let x = coord.x.unsigned_abs();
        let y = coord.y.unsigned_abs();
        let square = if x > y { x } else { y };

        let min_on_square = (2 * square - 1).pow(2) + 1;
        let n_per_side = 2 * square;

        if coord.x == square as i64 && coord.y > -(square as i64) {
            // second check is just to eliminate bottom right corner
            Address::new(min_on_square + (coord.y + (square as i64 - 1)) as u64)
                .expect("Positive-definiteness guaranteed by math")
        } else if coord.y == square as i64 {
            Address::new(
                min_on_square + n_per_side + (coord.x - (square as i64 - 1)).unsigned_abs(),
            )
            .expect("Positive-definiteness guaranteed by math")
        } else if coord.x == -(square as i64) {
            Address::new(
                min_on_square + 2 * n_per_side + (coord.y - (square as i64 - 1)).unsigned_abs(),
            )
            .expect("Positive-definiteness guaranteed by math")
        } else {
            Address::new(min_on_square + 3 * n_per_side + (coord.x + (square as i64 - 1)) as u64)
                .expect("Positive-definiteness guaranteed by math")
        }
    }
}

fn find_larger_value(target: u64) -> u64 {
    if target < 1 {
        return 1;
    }

    let mut values = vec![1];

    for address in (2..).map(|v| Address::new(v).unwrap()) {
        let v = address
            .position()
            .neighbors_iter()
            .filter_map(|coord| values.get((Address::from_coord(coord).0.get() - 1) as usize))
            .sum();
        if v > target {
            return v;
        }
        values.push(v);
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::{Address, Coord, find_larger_value};

    #[test]
    fn position() {
        assert_eq!(Address::new(1).unwrap().position(), Coord::new(0, 0));
        assert_eq!(Address::new(9).unwrap().position(), Coord::new(1, -1));
        assert_eq!(Address::new(12).unwrap().position(), Coord::new(2, 1));
        assert_eq!(Address::new(23).unwrap().position(), Coord::new(0, -2));
        assert_eq!(Address::new(1024).unwrap().position().manhattan_norm(), 31);
    }

    #[test]
    fn from_coord() {
        assert_eq!(
            Address::from_coord(Coord::new(0, 0)),
            Address::new(1).unwrap()
        );
        assert_eq!(
            Address::from_coord(Coord::new(2, 1)),
            Address::new(12).unwrap()
        );
        assert_eq!(
            Address::from_coord(Coord::new(0, -2)),
            Address::new(23).unwrap()
        );
    }

    #[test]
    fn find_larger_value_works() {
        assert_eq!(find_larger_value(132), 133);
        assert_eq!(find_larger_value(4), 5);
        assert_eq!(find_larger_value(747), 806);
    }

    #[test]
    fn round_trip() {
        for n in 1..1_000 {
            let a = Address::new(n).unwrap();
            assert_eq!(Address::from_coord(a.position()), a);
        }
    }
}
