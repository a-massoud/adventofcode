// There absolutely are closed forms but this is easy

use std::{collections::VecDeque, env};

use anyhow::{Context, Result, anyhow};

fn main() -> Result<()> {
    let n_elves = env::args()
        .nth(1)
        .ok_or(anyhow!("no input provided"))?
        .parse()
        .context("input must be a number")?;

    println!("===Part 1===");
    let last_elf = find_last_elf_stealing_next(n_elves);
    println!("Last elf: {}", last_elf);
    println!();

    println!("===Part 2===");
    let last_elf = find_last_elf_stealing_opposite(n_elves);
    println!("Last elf: {}", last_elf);

    Ok(())
}

fn find_last_elf_stealing_next(n_elves: usize) -> usize {
    if n_elves < 2 {
        return n_elves;
    }

    // since each elf steals everything, we only need to keep track of who's next
    let mut elves: Vec<_> = (1..=n_elves).collect();
    elves[n_elves - 1] = 0;

    let mut current_elf = 0;
    while elves[current_elf] != current_elf {
        let steal_elf = elves[current_elf];
        elves[current_elf] = elves[steal_elf];
        current_elf = elves[current_elf];
    }

    // problem indexing is 1-based
    current_elf + 1
}

fn find_last_elf_stealing_opposite(n_elves: usize) -> usize {
    if n_elves < 2 {
        return n_elves;
    }

    let half_elf = n_elves / 2;
    let mut right_half: VecDeque<_> = (1..=half_elf).collect();
    let mut left_half: VecDeque<_> = ((half_elf + 1)..=n_elves).collect();

    while left_half.len() + right_half.len() > 1 {
        let current_elf = right_half
            .pop_front()
            .expect("halves are balanced and there are at least two elements");
        left_half
            .pop_front()
            .expect("halves are balanced and there are at least two elements");
        left_half.push_back(current_elf);
        while right_half.len() < (left_half.len() + right_half.len()) / 2 {
            if let Some(x) = left_half.pop_front() {
                right_half.push_back(x);
            } else {
                break;
            }
        }
    }

    right_half
        .pop_front()
        .or_else(|| left_half.pop_front())
        .expect("loop only terminates when there is one elf")
}

#[cfg(test)]
mod test {
    use crate::{find_last_elf_stealing_next, find_last_elf_stealing_opposite};

    #[test]
    fn sample_input() {
        assert_eq!(find_last_elf_stealing_next(5), 3);
        assert_eq!(find_last_elf_stealing_opposite(5), 2);
    }
}
