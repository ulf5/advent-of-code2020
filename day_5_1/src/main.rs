use std::fs::File;
use std::io::{prelude::*, BufReader};


#[derive(Debug)]
struct BoardingPass {
    row: u64,
    col: u64,
    seat_id: u64,
}

impl From<String> for BoardingPass {
    fn from(s: String) -> Self {
        let mut row = 0;
        let mut row_range = 128;
        let mut col_range = 8;
        let mut col = 0;
        for c in s.chars() {
            match c {
                'F' => {
                    row_range = row_range / 2 as u64;
                }
                'B' => {
                    row_range = row_range / 2 as u64;
                    row += row_range;
                }
                'L' => {
                    col_range = col_range / 2 as u64;
                }
                'R' => {
                    col_range = col_range / 2 as u64;
                    col += col_range;
                }
                _ => panic!("nooo"),
            }
        }
        Self {
            row,
            col,
            seat_id: row * 8 + col,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut all: Vec<BoardingPass> = vec![];
    for line in reader.lines() {
        all.push(line?.into());
    }
    dbg!(all.iter().max_by_key(|e| e.seat_id).unwrap());
    Ok(())
}
