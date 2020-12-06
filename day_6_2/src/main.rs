use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    dbg!(input
        .split("\n\n")
        .map(|group| group
            .split_whitespace()
            .map(|x| x.bytes().fold(0u32, |x, b| x | 1 << (b - b'a')))
            .fold(!0, |x, g| x & g)
            .count_ones())
        .sum::<u32>());
    Ok(())
}
