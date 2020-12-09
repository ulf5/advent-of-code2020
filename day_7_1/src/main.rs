use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r"^(?P<color>[a-z\s]+?) bags contain (?P<contains>(?:,? ?(?:\d|no) (?:[a-z\s]+) bags?)+)\.\n?$").unwrap();
    static ref CONTAINS_REGEX: Regex = Regex::new(r"(?P<quantity>\d) (?P<color>[a-z\s]+) bag").unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut inverted_index: HashMap<String, HashSet<String>> = HashMap::new();
    for line in reader.lines() {
        let s = line?;
        let cap = BAG_REGEX.captures(&s).unwrap();
        let color = cap.name("color").unwrap().as_str();

        for c in CONTAINS_REGEX.captures_iter(cap.name("contains").unwrap().as_str()) {
            let color_name = c.name("color").unwrap().as_str();
            let c_set = inverted_index
                .entry(color_name.to_string())
                .or_insert(HashSet::new());
            c_set.insert(color.to_string());
        }
    }

    let mut h: HashSet<String> = HashSet::new();
    for c in inverted_index.get("shiny gold").unwrap().clone() {
        rec_search(&mut h, &mut inverted_index, &c);
    }
    dbg!(h.len());
    Ok(())
}

fn rec_search(h: &mut HashSet<String>, search: &mut HashMap<String, HashSet<String>>, c: &str) {
    h.insert(c.to_string());
    let mut to_search = HashSet::new();
    if let Some(ci) = search.get_mut(c) {
        for i in ci.iter() {
            if !h.contains(i) {
                to_search.insert(i.to_string());
            }
        }
    }

    for i in to_search.iter() {
        rec_search(h, search, i);
    }
}
