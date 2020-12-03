use std::fs::File;
use std::io::{prelude::*, BufReader};

fn calc(all: Vec<u64>) -> (u64, u64, u64) {
    for i in 0..all.len() {
        for j in i+1..all.len() {
            for k in j+1..all.len() {
                if all[i] + all[j] + all[k] == 2020 {
                    return (all[i], all[j], all[k]);
                }
            }
        }
    }
    panic!("sumting wong");
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut all = vec![];
    for line in reader.lines() {
        let num: u64 = line?.parse()?;

        all.push(num);
    }
    let (one, two, three) = calc(all);

    println!("{} {} {}", one, two, three);
    println!("{}", one * two * three);

    Ok(())
}
