use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let Some(path) = env::args().nth(1) else {
        bail!("Missing argument");
    };

    let contents = fs::read_to_string(path)?;
    let lines = contents.lines();

    let mut spans = Vec::new();
    let mut symbols = HashSet::new();

    for (lineno, line) in lines.enumerate() {
        let mut buf = String::new();

        let mut start = None;
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                buf.push(c);
                if start.is_none() {
                    start = Some(i);
                }
            } else {
                if !buf.is_empty() {
                    let num: u32 = buf.parse().unwrap();
                    buf.clear();
                    spans.push((lineno, (start.take().unwrap(), i - 1), num));
                }
                if c == '*' {
                    symbols.insert((lineno, i));
                }
            }
        }

        if !buf.is_empty() {
            let num: u32 = buf.parse().unwrap();
            buf.clear();
            spans.push((lineno, (start.take().unwrap(), line.len()), num));
        }
    }

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (lineno, (l, r), num) in spans {
        for i in l.saturating_sub(1)..=r.saturating_add(1) {
            // above
            if symbols.contains(&(lineno.saturating_add(1), i)) {
                gears
                    .entry((lineno.saturating_add(1), i))
                    .and_modify(|v| v.push(num))
                    .or_insert(vec![num]);
            }
            // below
            if symbols.contains(&(lineno.saturating_sub(1), i)) {
                gears
                    .entry((lineno.saturating_sub(1), i))
                    .and_modify(|v| v.push(num))
                    .or_insert(vec![num]);
            }
        }

        // left
        if symbols.contains(&(lineno, r.saturating_add(1))) {
            gears
                .entry((lineno, r.saturating_add(1)))
                .and_modify(|v| v.push(num))
                .or_insert(vec![num]);
        }
        // right
        if symbols.contains(&(lineno, l.saturating_sub(1))) {
            gears
                .entry((lineno, l.saturating_sub(1)))
                .and_modify(|v| v.push(num))
                .or_insert(vec![num]);
        }
    }

    let result: u32 = gears
        .into_iter()
        .filter(|(_k, v)| v.len() == 2)
        .map(|(_k, v)| v.into_iter().product::<u32>())
        .sum();

    println!("Result: {result}");

    Ok(())
}
