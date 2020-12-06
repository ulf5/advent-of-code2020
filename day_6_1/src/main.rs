use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut all: Vec<HashSet<char>> = vec![];
    let mut chunk: Vec<String> = vec![];
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            all.push(chunk.iter().flat_map(|x| x.chars()).collect());
            chunk = vec![];
        } else {
            chunk.push(line)
        }
    }
    all.push(chunk.iter().flat_map(|x| x.chars()).collect());
    dbg!(all.iter().map(|x| x.len()).sum::<usize>());
    Ok(())
}
