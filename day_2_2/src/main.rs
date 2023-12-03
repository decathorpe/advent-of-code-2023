use std::env;
use std::fs;

use anyhow::bail;

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn main() -> anyhow::Result<()> {
    let Some(path) = env::args().nth(1) else {
        bail!("Missing argument");
    };

    let contents = fs::read_to_string(path)?;

    let result: u32 = contents
        .lines()
        .map(|line| {
            let (_gameno, game): (u32, &str) = {
                let mut parts = line.strip_prefix("Game ").unwrap().splitn(2, ": ");
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap(),
                )
            };

            let colors = game
                .trim()
                .split(';')
                .map(|show| {
                    show.trim().split(',').map(|item| {
                        let mut parts = item.trim().splitn(2, ' ');
                        let no: u32 = parts.next().unwrap().parse().unwrap();
                        let color = parts.next().unwrap();

                        match color {
                            "red" => Color::Red(no),
                            "green" => Color::Green(no),
                            "blue" => Color::Blue(no),
                            _ => unreachable!(),
                        }
                    })
                })
                .flatten();

            let r_min = colors
                .clone()
                .map(|color| if let Color::Red(r) = color { r } else { 0 })
                .max()
                .unwrap();
            let g_min = colors
                .clone()
                .map(|color| if let Color::Green(r) = color { r } else { 0 })
                .max()
                .unwrap();
            let b_min = colors
                .map(|color| if let Color::Blue(r) = color { r } else { 0 })
                .max()
                .unwrap();

            r_min * g_min * b_min
        })
        .sum();

    println!("Result: {result}");

    Ok(())
}
