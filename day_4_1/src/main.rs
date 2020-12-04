use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

impl From<Vec<String>> for Passport {
    fn from(blob: Vec<String>) -> Self {
        let mut m: std::collections::HashMap<_, _> = blob
            .join(" ")
            .split_whitespace()
            .into_iter()
            .map(|e| e.splitn(2, ':').map(|s| s.to_string()).collect_tuple::<(String, String)>().unwrap())
            .collect();
        Self {
            byr: m.remove("byr"),
            iyr: m.remove("iyr"),
            eyr: m.remove("eyr"),
            hgt: m.remove("hgt"),
            hcl: m.remove("hcl"),
            ecl: m.remove("ecl"),
            pid: m.remove("pid"),
            cid: m.remove("cid")
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut all: Vec<Passport> = vec![];
    let mut chunk = vec![];
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            all.push(chunk.into());
            chunk = vec![];
        } else {
            chunk.push(line)
        }
    }
    all.push(chunk.into());
    dbg!(all.iter().filter(|e| e.is_valid()).count());
    Ok(())
}
