
fn main() {
    let lines = include_str!("../input.txt").lines();
    let sum: u32 = lines
        .map(|s| {
            s.replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
        })
        .map(|l| {
            let mut digits = l.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum();
    dbg!(sum);
}
