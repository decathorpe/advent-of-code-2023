use std::env;
use std::fs;

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let Some(path) = env::args().skip(1).next() else {
        bail!("Missing argument");
    };

    let contents = fs::read_to_string(path)?;

    let result: u32 = contents
        .lines()
        .map(|line| {
            let chars = line
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();

            [chars.first().unwrap(), chars.last().unwrap()]
                .into_iter()
                .collect::<String>()
                .parse::<u32>()
        })
        .collect::<Result<Vec<u32>, _>>()?
        .into_iter()
        .sum();

    println!("Result: {}", result);

    Ok(())
}
