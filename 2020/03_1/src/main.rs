use std::fs::File;
use std::io::{prelude::*, BufReader};

fn count(slope: Vec<Vec<bool>>) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    loop {
        if slope[x][y] {
            count += 1;
        }

        x += 1;
        y += 3;
        if x == slope.len() {
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
    dbg!(count(all));
    Ok(())
}
