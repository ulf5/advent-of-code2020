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
    lines.insert(0, 0);
    let mut ones = 0;
    let mut threes = 1;
    lines.windows(2).for_each(|x| {
        let diff = x[1] - x[0];
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    });
    threes * ones
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example() {
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
        assert_eq!(xmas_run(lines), 220);
    }
}
