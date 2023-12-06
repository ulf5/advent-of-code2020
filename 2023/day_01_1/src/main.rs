fn main() {
    let lines = include_str!("../input.txt").lines();
    let sum: u32 = lines
        .map(|l| {
            let mut digits = l.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum();
    dbg!(&sum);
}
