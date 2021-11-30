use std::fs::File;
use std::io::{prelude::*, BufReader};

const TARGET: u64 = 29221323;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<u64> = vec![];
    for line in reader.lines() {
        lines.push(line?.parse().unwrap());
    }

    dbg!(xmas_run(lines, TARGET));
    Ok(())
}

fn xmas_run(lines: Vec<u64>, target: u64) -> u64 {
    let mut cnt = 2;
    for i in 0..lines.len() {
        loop {
            let res: u64 = lines[i..i + cnt].iter().sum();
            if res > target {
                cnt = 2;
                break;
            } else if res == target {
                let sl = &lines[i..i + cnt];
                return sl.iter().min().unwrap() + sl.iter().max().unwrap();
            }
            cnt += 1;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example() {
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
        assert_eq!(xmas_run(lines, 127), 62);
    }
}
