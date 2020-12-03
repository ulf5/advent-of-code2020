use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Policy {
    ch: char,
    min: u64,
    max: u64,
}

#[derive(Debug)]
struct Line {
    policy: Policy,
    password: String,
}

impl std::str::FromStr for Line {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<&str> = s.split_whitespace().collect();

        let min_max: Vec<&str> = sp[0].split('-').collect();
        let min = min_max[0].parse()?;
        let max = min_max[1].parse()?;
        let chs: Vec<char> = sp[1].chars().collect();

        Ok(Self {
            policy: Policy {
                min,
                max,
                ch: chs[0],
            },
            password: sp[2].to_string(),
        })
    }
}
impl Line {
    fn valid(&self) -> bool {
        let chrs: Vec<char> = self.password.chars().collect();
        let mindex: usize = self.policy.min as usize - 1usize;
        let maxdex: usize = self.policy.max as usize - 1usize;

        (chrs[mindex] == self.policy.ch) ^ (chrs[maxdex] == self.policy.ch)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut all = vec![];
    for line in reader.lines() {
        let ele: Line = line?.parse()?;

        all.push(ele);
    }
    println!("{}", all.iter().filter(|a| a.valid()).count());
    Ok(())
}
