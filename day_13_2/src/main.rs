fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = include_str!("../input.txt").lines();
    let _ignore = lines.next().unwrap();
    let line = lines.next().unwrap();
    dbg!(xmas_run(&line));
    Ok(())
}

fn xmas_run(line: &str) -> i64 {
    let t: (Vec<i64>, Vec<i64>) = line
        .split(",")
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|(i, x)| (i as i64, x.parse::<i64>().unwrap()))
        .unzip();

    ring_algorithm::chinese_remainder_theorem(&t.0, &t.1)
        .unwrap()
        .abs()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example() {
        let line = "7,13,x,x,59,x,31,19";
        assert_eq!(xmas_run(line), 1068781);
    }
}
