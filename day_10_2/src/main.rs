use std::collections::HashMap;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines: Vec<u64> = vec![];
    for line in include_str!("../input.txt").lines() {
        lines.push(line.parse().unwrap());
    }

    dbg!(xmas_run(lines));
    Ok(())
}

fn xmas_run(mut lines: Vec<u64>) -> u64 {
    lines.sort();
    let mut p = HashMap::new();
    p.insert(0, 1);
    for i in &lines {
        let c = p.get(&(i.wrapping_sub(1))).unwrap_or(&0)
            + p.get(&(i.wrapping_sub(2))).unwrap_or(&0)
            + p.get(&(i.wrapping_sub(3))).unwrap_or(&0);
        p.insert(*i, c);
    }
    p[lines.last().unwrap()]
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example() {
        let lines: Vec<u64> = "16
10
15
5
1
11
7
19
6
12
4"
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();
        assert_eq!(xmas_run(lines), 8);
    }

    #[test]
    fn test_example_2() {
        let lines: Vec<u64> = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();
        assert_eq!(xmas_run(lines), 19208);
    }
}
