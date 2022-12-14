use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// I did today in rust for some reason. Part II solution is... hacked together from the part I one, but it's fine and it works.

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PointVal {
    Empty,
    Wall,
    Sand,
}

#[derive(Debug, Clone)]
struct Field {
    min_x: i64,
    field: Vec<Vec<PointVal>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("I need some input");
        return;
    }

    let mut field = read_input(&args[1]);

    let print_field = |field: &Field| {
        for row in &field.field {
            for col in row {
                print!(
                    "{}",
                    match col {
                        PointVal::Empty => '.',
                        PointVal::Wall => '#',
                        PointVal::Sand => 'o',
                    }
                );
            }
            println!();
        }
    };

    print_field(&field);

    let part1_results = simulate_part1(field.clone());
    println!();
    print_field(&part1_results.1);
    println!("Part 1 results: {}", part1_results.0);

    let gen_row = |len: usize, val: PointVal| -> Vec<PointVal> {
        let mut row = Vec::<PointVal>::new();

        for _ in 0..len {
            row.push(val);
        }

        row
    };

    field
        .field
        .push(gen_row(field.field[0].len(), PointVal::Empty));
    field
        .field
        .push(gen_row(field.field[0].len(), PointVal::Empty));

    let part2_results = simulate_part2(field.clone());
    println!();
    print_field(&part2_results.1);
    println!("Part 2 results: {}", part2_results.0);
}

fn read_input(fname: &str) -> Field {
    let input_file_path = Path::new(fname);

    let input_file = File::open(input_file_path).unwrap();

    let mut paths = Vec::<Vec<Point>>::new();

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    for line in io::BufReader::new(input_file).lines().map(|x| x.unwrap()) {
        let mut curve = Vec::<Point>::new();
        for seg in line.split(" -> ") {
            let pnt: Vec<&str> = seg.split(',').collect();
            let p = Point {
                x: pnt[0].parse().unwrap(),
                y: pnt[1].parse().unwrap(),
            };
            if p.x < min_x {
                min_x = p.x;
            }
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y > max_y {
                max_y = p.y;
            }
            curve.push(p);
        }
        paths.push(curve);
    }

    let mut field = Field {
        min_x,
        field: Vec::<Vec<PointVal>>::new(),
    };

    for _ in 0..(max_y + 1) {
        let mut row = Vec::<PointVal>::new();
        for _ in min_x..(max_x + 1) {
            row.push(PointVal::Empty);
        }
        field.field.push(row);
    }
    for path in paths {
        for i in 0..(path.len() - 1) {
            if path[i].x - path[i + 1].x == 0 {
                // vertical line
                let from = if path[i].y < path[i + 1].y {
                    path[i].y
                } else {
                    path[i + 1].y
                };
                let to = if path[i].y >= path[i + 1].y {
                    path[i].y
                } else {
                    path[i + 1].y
                };
                for j in from..(to + 1) {
                    field.field[j as usize][(path[i].x - field.min_x) as usize] = PointVal::Wall;
                }
            } else if path[i].y - path[i + 1].y == 0 {
                // horizontal line
                let from = if path[i].x < path[i + 1].x {
                    path[i].x
                } else {
                    path[i + 1].x
                };
                let to = if path[i].x >= path[i + 1].x {
                    path[i].x
                } else {
                    path[i + 1].x
                };
                for j in from..(to + 1) {
                    field.field[path[i].y as usize][(j - field.min_x) as usize] = PointVal::Wall;
                }
            } else {
                eprintln!("This should not happen.");
            }
        }
    }
    return field;
}

fn simulate_part1(mut field: Field) -> (i64, Field) {
    let mut total_grains = 0i64;

    let is_out_of_bounds = |grain: Point, bounds_x: (i64, i64), bounds_y: (i64, i64)| {
        grain.x < bounds_x.0 || grain.x > bounds_x.1 || grain.y < bounds_y.0 || grain.y > bounds_y.1
    };

    let bounds_x = (field.min_x, field.min_x + (field.field[0].len() as i64) - 1);
    let bounds_y = (0, (field.field.len() as i64) - 1);

    'simulating: loop {
        let mut grain = Point { x: 500, y: 0 };

        loop {
            if is_out_of_bounds(grain, bounds_x, (bounds_y.0, bounds_y.1 - 1)) {
                break 'simulating;
            }

            // down
            if field.field[(grain.y + 1) as usize][(grain.x - field.min_x) as usize]
                == PointVal::Empty
            {
                grain.y += 1;
                continue;
            }

            // down and left
            if grain.x <= bounds_x.0
                || field.field[(grain.y + 1) as usize][(grain.x - field.min_x - 1) as usize]
                    == PointVal::Empty
            {
                grain.y += 1;
                grain.x -= 1;
                continue;
            }

            // down and right
            if grain.x >= bounds_x.1
                || field.field[(grain.y + 1) as usize][(grain.x - field.min_x + 1) as usize]
                    == PointVal::Empty
            {
                grain.y += 1;
                grain.x += 1;
                continue;
            }

            // stop moving
            break;
        }

        field.field[grain.y as usize][(grain.x - field.min_x) as usize] = PointVal::Sand;
        total_grains += 1;
    }

    (total_grains, field)
}

fn simulate_part2(mut field: Field) -> (i64, Field) {
    let mut total_grains = 0i64;

    let mut bounds_x = (field.min_x, field.min_x + (field.field[0].len() as i64) - 1);
    let bounds_y = (0, (field.field.len() as i64) - 1);

    'simulating: loop {
        let mut grain = Point { x: 500, y: 0 };

        loop {
            if field.field[0][(500 - field.min_x) as usize] == PointVal::Sand {
                break 'simulating;
            }

            // down
            if grain.y + 1 < bounds_y.1
                && field.field[(grain.y + 1) as usize][(grain.x - field.min_x) as usize]
                    == PointVal::Empty
            {
                grain.y += 1;
                continue;
            }

            // down and left
            if grain.y + 1 < bounds_y.1
                && (grain.x - 1 <= bounds_x.0
                    || field.field[(grain.y + 1) as usize][(grain.x - field.min_x - 1) as usize]
                        == PointVal::Empty)
            {
                if grain.x - 1 <= bounds_x.0 {
                    for row in &mut field.field {
                        row.insert(0, PointVal::Empty);
                    }
                    field.min_x -= 1;
                    bounds_x.0 -= 1;
                }
                grain.y += 1;
                grain.x -= 1;
                continue;
            }

            // down and right
            if grain.y + 1 < bounds_y.1
                && (grain.x + 1 >= bounds_x.1
                    || field.field[(grain.y + 1) as usize][(grain.x - field.min_x + 1) as usize]
                        == PointVal::Empty)
            {
                if grain.x + 1 >= bounds_x.1 {
                    for row in &mut field.field {
                        row.push(PointVal::Empty);
                    }
                    bounds_x.1 += 1;
                }
                grain.y += 1;
                grain.x += 1;
                continue;
            }

            // stop moving
            break;
        }

        field.field[grain.y as usize][(grain.x - field.min_x) as usize] = PointVal::Sand;
        total_grains += 1;
    }

    (total_grains, field)
}
