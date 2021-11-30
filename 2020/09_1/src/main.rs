use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<u64> = vec![];
    for line in reader.lines() {
        lines.push(line?.parse().unwrap());
    }

    dbg!(xmas_run(lines, 25));
    Ok(())
}

fn xmas_run(lines: Vec<u64>, prel: usize) -> u64 {
    for i in prel..lines.len() {
        if !find(&lines[i - prel as usize..i], lines[i]) {
            return lines[i];
        }
    }
    unreachable!();
}

fn find(sl: &[u64], target: u64) -> bool {
    sl.iter()
        .tuple_combinations()
        .map(|(x, y)| x + y)
        .find(|x| *x == target)
        .is_some()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example() {
        let lines: Vec<u64> = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();
        assert_eq!(xmas_run(lines, 5), 127);
    }
}
