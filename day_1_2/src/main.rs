use std::env;
use std::fs;

use anyhow::bail;

const NUMBERS: [(u32, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn main() -> anyhow::Result<()> {
    let Some(path) = env::args().nth(1) else {
        bail!("Missing argument");
    };

    let contents = fs::read_to_string(path)?;

    let result: u32 = contents
        .lines()
        .map(|line| {
            let mut nums: Vec<u32> = Vec::new();

            let mut buf = String::new();
            for c in line.chars() {
                if let Some(digit) = c.to_digit(10) {
                    nums.push(digit);
                    buf.clear();
                } else {
                    buf.push(c);

                    for (n, s) in NUMBERS {
                        if buf.contains(s) {
                            nums.push(n);
                            buf.clear();

                            // words for numbers can overlap by at most one character
                            buf.push(s.chars().last().unwrap());
                        }
                    }
                }
            }

            let value = nums.first().unwrap() * 10 + nums.last().unwrap();

            println!("{}: {:?} -> {}", line, nums, value);

            value
        })
        .sum();

    println!("Result: {}", result);

    Ok(())
}
