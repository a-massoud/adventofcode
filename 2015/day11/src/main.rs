use std::env;

use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    let input = env::args().nth(1).ok_or(anyhow!("no argument"))?;
    if input.len() != 8 || !input.chars().all(|ch| ch.is_ascii_lowercase()) {
        bail!("input should be eight lowercase ascii characters")
    }

    let p1 = get_next_pass(&input);
    println!("Part 1: {}", p1);

    let p2 = get_next_pass(&p1);
    println!("Part 2: {}", p2);

    Ok(())
}

fn increment_password(pass: &str) -> String {
    let mut r: Vec<_> = pass
        .chars()
        .rev()
        .scan(true, |should_inc, ch| {
            if *should_inc && ch.is_ascii_lowercase() {
                const WRAPPER: u8 = b'z' - b'a' + 1;
                let next_ch = ch as u8 - b'a' + 1;
                *should_inc = next_ch >= WRAPPER;
                Some(((next_ch % WRAPPER) + b'a') as char)
            } else {
                Some(ch)
            }
        })
        .collect();
    r.reverse();

    r.iter().collect()
}

fn check_password(pass: &str) -> bool {
    let pass = pass.as_bytes();
    let mut triplet_found = false;
    let mut double_indices = Vec::new();

    if !pass.len() == 8 {
        return false;
    }

    for (i, &ch) in pass.iter().enumerate() {
        if ch < b'a' || ch > b'z' || ch == b'i' || ch == b'o' || ch == b'l' {
            return false;
        }

        if !triplet_found && i >= 2 {
            triplet_found = pass[i - 1] == ch - 1 && pass[i - 2] == ch - 2;
        }

        if i >= 1 && pass[i - 1] == ch {
            double_indices.push(i);
        }
    }

    if !triplet_found
        || double_indices.len() <= 1
        || (double_indices.len() == 2 && double_indices[1] == double_indices[0] + 1)
    {
        return false;
    }

    true
}

fn get_next_pass(pass: &str) -> String {
    let mut pass = pass.to_owned();
    let mut max_reached;

    loop {
        max_reached = !pass.is_empty() && pass.as_bytes()[0] == b'z';

        pass = increment_password(&pass);

        if max_reached && !pass.is_empty() && pass.as_bytes()[0] == b'a' {
            return String::new();
        }

        if check_password(&pass) {
            break;
        }
    }

    pass
}
