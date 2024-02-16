use std::io::{self, stdin, Read, Write};

use anyhow::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    part1(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut freq = 0;
    for line in input.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }
    println!("{}", freq);
    // writeln!(io::stdout(), "{}", freq)?;
    Ok(())
}
