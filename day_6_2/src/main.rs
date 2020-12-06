#![feature(iterator_fold_self)]

use bit_vec::BitVec;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut all: Vec<usize> = vec![];
    let mut chunk: Vec<BitVec> = vec![];
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            let c: BitVec = chunk
                .into_iter()
                .fold_first(|mut a, b| {
                    a.and(&b);
                    a
                })
                .unwrap();
            let p = c.iter().filter(|x| *x).count();
            all.push(p);
            chunk = vec![];
        } else {
            let person: BitVec = line
                .chars()
                .fold(BitVec::from_elem(26, false), |mut acc, x| {
                    acc.set((x as usize % 32usize) - 1usize, true);
                    acc
                });
            chunk.push(person);
        }
    }
    dbg!(all.iter().sum::<usize>());
    Ok(())
}
