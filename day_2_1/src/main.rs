use std::env;
use std::fs;

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let Some(path) = env::args().nth(1) else {
        bail!("Missing argument");
    };

    let contents = fs::read_to_string(path)?;

    let result: u32 = contents
        .lines()
        .map(|line| {
            let (gameno, game): (u32, &str) = {
                let mut parts = line.strip_prefix("Game ").unwrap().splitn(2, ": ");
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap(),
                )
            };

            let possible = game.trim().split(';').all(|show| {
                show.trim().split(',').all(|item| {
                    let mut parts = item.trim().splitn(2, ' ');
                    let no: u32 = parts.next().unwrap().parse().unwrap();
                    let color = parts.next().unwrap();

                    match color {
                        "red" if no <= 12 => true,
                        "green" if no <= 13 => true,
                        "blue" if no <= 14 => true,
                        _ => false,
                    }
                })
            });

            if possible {
                gameno
            } else {
                0
            }
        })
        .sum();

    println!("Result: {result}");

    Ok(())
}
