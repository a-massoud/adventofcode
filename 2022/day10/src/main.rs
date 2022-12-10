use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

// I dunno why I did today in Rust, just kinda felt like it. Overall not bad at all, this worked
// out pretty well. If this becomes the new intcode, I'm gonna need to re-implement it in C++ as an
// actual API but for now we good.

fn main() {
    let lines = match read_input(&match env::args().nth(1) {
        Some(fname) => fname,
        None => {
            eprintln!("i need input");
            process::exit(1);
        }
    }) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("Err: {}", err);
            process::exit(1);
        }
    };

    let part1 = part1_results(&lines);
    println!("Part 1: {}", part1);

    let part2 = part2_results(&lines);
    println!("Part 2:");
    for line in part2 {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn read_input(fname: &String) -> Result<Vec<String>, io::Error> {
    let input_file = match File::open(fname) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut lines = Vec::new();
    for line in BufReader::new(input_file).lines() {
        match line {
            Ok(str) => lines.push(str),
            Err(err) => return Err(err),
        }
    }

    Ok(lines)
}

fn part1_results(lines: &Vec<String>) -> i32 {
    let mut vals = HashSet::<i32>::new();
    let mut clock = 0i32;
    let mut rx = 1i32;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "addx" {
            clock += 1;
            if (clock - 20) % 40 == 0 {
                eprintln!("clock: {} val: {}", clock, rx);
                vals.insert(rx * clock);
            }
            clock += 1;
            if (clock - 20) % 40 == 0 {
                eprintln!("clock: {} val: {}", clock, rx);
                vals.insert(rx * clock);
            }
            rx += parts[1].parse::<i32>().unwrap();
        } else {
            clock += 1;
            if (clock - 20) % 40 == 0 {
                eprintln!("clock: {} val: {}", clock, rx);
                vals.insert(rx * clock);
            }
        }
    }

    let mut tot = 0i32;
    for val in vals {
        tot += val;
    }
    return tot;
}

fn part2_results(lines: &Vec<String>) -> [[char; 40]; 6] {
    let mut screen = [[' '; 40]; 6];
    let mut clock = 0u64;
    let mut rx = 1i64;
    let mut iptr = 0usize;

    let should_draw_pixel = |x: i64, clock: u64| -> bool {
        let x_pos = (clock % 40) as i64;
        x_pos - 1 <= x && x <= x_pos + 1
    };

    let mut in_add = false;
    while iptr < lines.len() {
        let parts: Vec<&str> = lines[iptr].split(" ").collect();
        if should_draw_pixel(rx, clock) {
            screen[(clock / 40) as usize][(clock % 40) as usize] = '#';
        }

        iptr += 1;
        if parts[0] == "addx" {
            if in_add {
                in_add = false;
                rx += parts[1].parse::<i64>().unwrap();
            } else {
                in_add = true;
                iptr -= 1;
            }
        }
        clock += 1;
    }

    return screen;
}
