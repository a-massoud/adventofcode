use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return Ok(());
    }

    let input = BufReader::new(File::open(&args[1])?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part 1 result: {}", get_number_nice(&input, is_nice_p1));
    println!("Part 2 result: {}", get_number_nice(&input, is_nice_p2));

    Ok(())
}

fn is_nice_p1(string: &str) -> bool {
    let mut double_letter = false;
    let mut vowel_count = 0;

    for (i, c) in string.chars().enumerate() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => (),
        };

        if string.chars().nth(i + 1).unwrap_or('\0') == c {
            double_letter = true;
        }

        if i != string.len() - 1 {
            match &string[i..i + 2] {
                "ab" | "cd" | "pq" | "xy" => return false,
                _ => (),
            }
        }
    }

    double_letter && vowel_count >= 3
}

fn is_nice_p2(string: &str) -> bool {
    let mut letter_pairs = HashSet::new();
    let mut next_pair = String::new();
    let mut found_letter_pair = false;
    let mut found_repeat = false;

    for i in 0..string.len() {
        if i < string.len() - 1 {
            if letter_pairs.contains(&string[i..i + 2]) {
                found_letter_pair = true;
            }
            letter_pairs.insert(next_pair.clone());
            next_pair = String::from(&string[i..i + 2]);
        }

        if i < string.len() - 2
            && string.chars().nth(i).unwrap() == string.chars().nth(i + 2).unwrap()
        {
            found_repeat = true;
        }
    }

    found_letter_pair && found_repeat
}

fn get_number_nice(input: &[String], is_nice: fn(&str) -> bool) -> usize {
    input
        .iter()
        .filter_map(|x| if is_nice(&x) { Some(x) } else { None })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_p1() {
        assert!(is_nice_p1("ugknbfddgicrmopn"));
        assert!(is_nice_p1("aaa"));
        assert!(!is_nice_p1("jchzalrnumimnmhp"));
        assert!(!is_nice_p1("haegwjzuvuyypxyu"));
        assert!(!is_nice_p1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice_p2() {
        assert!(is_nice_p2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_p2("xxyxx"));
        assert!(!is_nice_p2("uurcxstgmygtbstg"));
        assert!(!is_nice_p2("ieodomkazucvgmuy"));
        assert!(!is_nice_p2("aaa"));
    }
}
