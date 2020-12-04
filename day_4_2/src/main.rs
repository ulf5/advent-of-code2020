use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

lazy_static! {
    static ref HCL_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref ECL_REGEX: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_REGEX: Regex = Regex::new("^[0-9]{9}$").unwrap();
}
struct Passport {
    byr: Option<u64>,
    iyr: Option<u64>,
    eyr: Option<u64>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.map_or(false, |byr| byr >= 1920 && byr <= 2002)
            && self.iyr.map_or(false, |iyr| iyr >= 2010 && iyr <= 2020)
            && self.eyr.map_or(false, |eyr| eyr >= 2020 && eyr <= 2030)
            && self.hgt.as_ref().map_or(false, |hgt| _val_hgt(&hgt))
            && self
                .hcl
                .as_ref()
                .map_or(false, |hcl| HCL_REGEX.is_match(&hcl))
            && self
                .ecl
                .as_ref()
                .map_or(false, |ecl| ECL_REGEX.is_match(&ecl))
            && self
                .pid
                .as_ref()
                .map_or(false, |pid| PID_REGEX.is_match(&pid))
    }
}
fn _val_hgt(hgt: &String) -> bool {
    let len = hgt.len();
    match &hgt[len - 2..] {
        "cm" => {
            if len != 5 {
                false
            } else {
                let num: Option<u64> = hgt[..3].parse().ok();
                num.map_or(false, |n| n >= 150 && n <= 193)
            }
        }
        "in" => {
            if len != 4 {
                false
            } else {
                let num: Option<u64> = hgt[..2].parse().ok();
                num.map_or(false, |n| n >= 59 && n <= 76)
            }
        }
        _ => false,
    }
}

impl From<Vec<String>> for Passport {
    fn from(blob: Vec<String>) -> Self {
        let mut m: std::collections::HashMap<_, _> = blob
            .join(" ")
            .split_whitespace()
            .into_iter()
            .map(|e| {
                e.splitn(2, ':')
                    .map(|s| s.to_string())
                    .collect_tuple::<(String, String)>()
                    .unwrap()
            })
            .collect();
        Self {
            byr: m.remove("byr").and_then(|v| v.parse().ok()),
            iyr: m.remove("iyr").and_then(|v| v.parse().ok()),
            eyr: m.remove("eyr").and_then(|v| v.parse().ok()),
            hgt: m.remove("hgt"),
            hcl: m.remove("hcl"),
            ecl: m.remove("ecl"),
            pid: m.remove("pid"),
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
