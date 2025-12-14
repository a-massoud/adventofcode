// I was optimistic I wouldn't have to actually parse the json...

use anyhow::anyhow;
use serde_json::Value;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).ok_or(anyhow!("no argument"))?)?;
    let input: Value = serde_json::from_str(&input)?;

    println!("Part 1: {}", sum_numbers(&input, true));
    println!("Part 2: {}", sum_numbers(&input, false));

    Ok(())
}

fn sum_numbers(input: &Value, allow_red: bool) -> i64 {
    let mut r = 0;
    let mut stack = Vec::new();
    stack.push(input);

    while let Some(val) = stack.pop() {
        match val {
            Value::Number(number) => r += number.as_i64().unwrap_or(0),
            Value::Array(values) => {
                for val in values {
                    stack.push(val)
                }
            }
            Value::Object(map) => {
                if !allow_red
                    && map
                        .values()
                        .any(|val| val.as_str().is_some_and(|v| v == "red"))
                {
                    continue;
                }
                for (_, val) in map {
                    stack.push(val);
                }
            }
            _ => (),
        }
    }

    r
}
