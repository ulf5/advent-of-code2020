use std::fs::File;
use std::io::{prelude::*, BufReader};

fn count(slope: &Vec<Vec<bool>>, right: usize, down: usize) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    loop {
        if slope[x][y] {
            count += 1;
        }

        x += down;
        y += right;
        if x >= slope.len() {
            break;
        }
        if y >= slope[x].len() {
            y -= slope[x].len();
        }
    }
    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut all = vec![];
    for line in reader.lines() {
        let mut row = vec![];
        for c in line?.chars() {
            row.push(c == '#');
        }
        all.push(row);
    }
    dbg!(
        count(&all, 1, 1)
            * count(&all, 3, 1)
            * count(&all, 5, 1)
            * count(&all, 7, 1)
            * count(&all, 1, 2)
    );
    Ok(())
}
