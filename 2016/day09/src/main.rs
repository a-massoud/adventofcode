// Again, nice and simple

use std::{env, fs};

use anyhow::{Context, anyhow, bail};

fn main() -> anyhow::Result<()> {
    let input_path = env::args().nth(1).ok_or(anyhow!("no input provided"))?;
    let input = fs::read_to_string(&input_path)?;
    let input = input.trim();

    println!("===Part 1===");
    let len = get_decompressed_len(input.as_bytes(), false)?;
    println!("Decompressed length: {}", len);
    println!();

    println!("===Part 2===");
    let len = get_decompressed_len(input.as_bytes(), true)?;
    println!("Decompressed length: {}", len);

    Ok(())
}
fn get_decompressed_len(input: &[u8], recurse: bool) -> anyhow::Result<usize> {
    let mut len = 0;

    let mut idx = 0;
    while idx < input.len() {
        if input[idx] == b'(' {
            let x_pos = input[idx..]
                .iter()
                .position(|&b| b == b'x')
                .ok_or_else(|| anyhow!("no x found starting at {}", idx))?
                + idx;
            let close_pos = input[idx..]
                .iter()
                .position(|&b| b == b')')
                .ok_or_else(|| anyhow!("no ) found starting at {}", idx))?
                + idx;
            if x_pos <= idx || close_pos <= x_pos || close_pos + 1 >= input.len() {
                bail!("open {}, x {}, close {} invalid", idx, x_pos, close_pos);
            }

            let take: usize = str::from_utf8(&input[idx + 1..x_pos])
                .expect("utf-8 bounded by ascii should be utf-8")
                .parse()
                .with_context(|| format!("parsing take from {} to {}", idx + 1, x_pos))?;
            let repeat: usize = str::from_utf8(&input[x_pos + 1..close_pos])
                .expect("utf-8 bounded by ascii should be utf-8")
                .parse()
                .with_context(|| format!("parsing repeat from {} to {}", idx + 1, x_pos))?;

            if close_pos + take + 1 > input.len() {
                bail!(
                    "not enough to take from (open {}, x {}, close {})",
                    idx,
                    x_pos,
                    close_pos
                );
            }

            let sub_len = if recurse {
                get_decompressed_len(&input[close_pos + 1..close_pos + 1 + take], true)
                    .with_context(|| format!("unraveling {}", idx))?
            } else {
                for &ch in &input[close_pos + 1..close_pos + 1 + take] {
                    if !(ch.is_ascii_uppercase()
                        || ch.is_ascii_digit()
                        || ch == b'('
                        || ch == b')'
                        || ch == b'x')
                    {
                        bail!("invalid character {}", ch);
                    }
                }
                take
            };
            len += sub_len * repeat;
            idx = close_pos + take + 1;
        } else if input[idx].is_ascii_uppercase() {
            len += 1;
            idx += 1;
        } else {
            bail!("invalid character {}", input[idx]);
        }
    }

    Ok(len)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_input_p1() {
        assert_eq!(
            get_decompressed_len(b"ADVENT", false).unwrap(),
            "ADVENT".len()
        );
        assert_eq!(
            get_decompressed_len(b"A(1x5)BC", false).unwrap(),
            "ABBBBBC".len()
        );
        assert_eq!(
            get_decompressed_len(b"(3x3)XYZ", false).unwrap(),
            "XYZXYZXYZ".len()
        );
        assert_eq!(
            get_decompressed_len(b"A(2x2)BCD(2x2)EFG", false).unwrap(),
            "ABCBCDEFEFG".len()
        );
        assert_eq!(
            get_decompressed_len(b"(6x1)(1x3)A", false).unwrap(),
            "(1x3)A".len()
        );
        assert_eq!(
            get_decompressed_len(b"X(8x2)(3x3)ABCY", false).unwrap(),
            "X(3x3)ABC(3x3)ABCY".len()
        );
    }

    #[test]
    fn sample_input_p2() {
        assert_eq!(get_decompressed_len(b"(3x3)XYZ", true).unwrap(), 9);
        assert_eq!(get_decompressed_len(b"X(8x2)(3x3)ABCY", true).unwrap(), 20);
        assert_eq!(
            get_decompressed_len(b"(27x12)(20x12)(13x14)(7x10)(1x12)A", true).unwrap(),
            241920
        );
        assert_eq!(
            get_decompressed_len(
                b"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                true
            )
            .unwrap(),
            445
        );
    }
}
