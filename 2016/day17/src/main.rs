// Nice and simple searches

use std::{collections::VecDeque, env};

use anyhow::{Result, anyhow};
use md5::{Digest, Md5, digest};

fn main() -> Result<()> {
    let passcode = env::args().nth(1).ok_or(anyhow!("no input provided"))?;

    println!("===Part 1===");
    let path = find_shortest_path(&passcode).ok_or(anyhow!("no shortest path found"))?;
    println!(
        "Path: {}",
        str::from_utf8(&path).expect("path only consists of `U` `D` `L` and `R`")
    );
    println!();

    println!("===Part 2===");
    let len = find_longest_path_length(&passcode).ok_or(anyhow!("no paths found"))?;
    println!("Length: {}", len);

    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

const START: Coord = Coord { x: 0, y: 0 };
const END: Coord = Coord { x: 3, y: 3 };

fn open_doors(hash: &digest::Output<Md5>) -> impl Iterator<Item = (u8, i64, i64)> {
    const DIRECTIONS: [(u8, i64, i64); 4] =
        [(b'U', 0, -1), (b'D', 0, 1), (b'L', -1, 0), (b'R', 1, 0)];
    let nibbles = [hash[0] >> 4, hash[0] & 0xF, hash[1] >> 4, hash[1] & 0xF];

    DIRECTIONS
        .into_iter()
        .zip(nibbles)
        .filter(|(_, n)| *n > 0xA)
        .map(|(m, _)| m)
}

fn find_shortest_path(passcode: &str) -> Option<Vec<u8>> {
    let mut hash_buf = Vec::from(passcode.as_bytes());
    let mut hasher = Md5::new();

    let mut queue = VecDeque::from([(START, Vec::new())]);

    while let Some((pos, hist)) = queue.pop_front() {
        hash_buf.truncate(passcode.len());
        hash_buf.extend(hist.iter());

        hasher.update(&hash_buf);
        let hash = hasher.finalize_reset();

        for (dir, dx, dy) in open_doors(&hash) {
            let new_pos = Coord {
                x: pos.x + dx,
                y: pos.y + dy,
            };

            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x > END.x || new_pos.y > END.y {
                continue;
            }

            let new_hist = {
                let mut new_hist = hist.clone();
                new_hist.push(dir);
                new_hist
            };

            if new_pos == END {
                return Some(new_hist);
            }

            queue.push_back((new_pos, new_hist));
        }
    }

    None
}

fn find_longest_path_length(passcode: &str) -> Option<usize> {
    let mut hash_buf = Vec::from(passcode.as_bytes());
    let mut hasher = Md5::new();

    let mut stack = vec![(START, Vec::new())];
    let mut longest = None;

    while let Some((pos, hist)) = stack.pop() {
        hash_buf.truncate(passcode.len());
        hash_buf.extend(hist.iter());

        hasher.update(&hash_buf);
        let hash = hasher.finalize_reset();

        for (dir, dx, dy) in open_doors(&hash) {
            let new_pos = Coord {
                x: pos.x + dx,
                y: pos.y + dy,
            };

            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x > END.x || new_pos.y > END.y {
                continue;
            }

            let new_hist = {
                let mut new_hist = hist.clone();
                new_hist.push(dir);
                new_hist
            };

            if new_pos == END {
                if longest.is_none_or(|longest| longest < new_hist.len()) {
                    longest = Some(new_hist.len());
                }
                continue;
            }

            stack.push((new_pos, new_hist));
        }
    }

    longest
}

#[cfg(test)]
mod test {
    use crate::{find_longest_path_length, find_shortest_path};

    #[test]
    fn sample_input_p1() {
        let path = find_shortest_path("ihgpwlah");
        assert_eq!(path.as_deref(), Some("DDRRRD".as_bytes()));

        let path = find_shortest_path("kglvqrro");
        assert_eq!(path.as_deref(), Some("DDUDRLRRUDRD".as_bytes()));

        let path = find_shortest_path("ulqzkmiv");
        assert_eq!(
            path.as_deref(),
            Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".as_bytes())
        );
    }

    #[test]
    fn sample_input_p2() {
        assert_eq!(find_longest_path_length("ihgpwlah"), Some(370));
        assert_eq!(find_longest_path_length("kglvqrro"), Some(492));
        assert_eq!(find_longest_path_length("ulqzkmiv"), Some(830));
    }
}
