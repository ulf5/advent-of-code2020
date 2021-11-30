use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r"^(?P<color>[a-z\s]+?) bags contain (?P<contains>(?:,? ?(?:\d|no) (?:[a-z\s]+) bags?)+)\.\n?$").unwrap();
    static ref CONTAINS_REGEX: Regex = Regex::new(r"(?P<quantity>\d) (?P<color>[a-z\s]+) bag").unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut index: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for line in reader.lines() {
        let s = line?;
        let cap = BAG_REGEX.captures(&s).unwrap();
        let color = cap.name("color").unwrap().as_str();
        let mut cmap = HashMap::new();

        for c in CONTAINS_REGEX.captures_iter(cap.name("contains").unwrap().as_str()) {
            cmap.insert(
                c["color"].to_string(),
                c["quantity"].parse::<u32>().unwrap(),
            );
        }
        index.insert(color.to_string(), cmap);
    }
    let mut memo_map = HashMap::new();

    dbg!(&index);
    dbg!(index["shiny gold"]
        .iter()
        .map(|(x, v)| calc_count(&mut memo_map, &index, x, *v))
        .sum::<u32>());

    Ok(())
}

fn calc_count(
    memo_map: &mut HashMap<String, u32>,
    index: &HashMap<String, HashMap<String, u32>>,
    s: &str,
    multiplier: u32,
) -> u32 {
    if let Some(v) = memo_map.get(s) {
        return multiplier * v;
    } else {
        let next = &index[s];
        let mut acc = 1;
        for (k, v) in next.iter() {
            acc += calc_count(memo_map, index, k, *v);
        }
        memo_map.insert(s.to_string(), acc);
        return acc * multiplier;
    }
}
