use std::collections::HashSet;

fn main() {
    let sum: u32 = include_str!("../input.txt")
        .lines()
        .filter_map(|l| l.split_once(":"))
        .filter_map(|(_, r)| r.split_once("|"))
        .map(|(w, n)| (to_set(w), to_set(n)))
        .map(|(w, n)| w.intersection(&n).count() as u32)
        .map(|c| match c {
            0 => 0,
            c => 2u32.pow(c - 1),
        })
        .sum();
    dbg!(sum);
}

fn to_set(n_str: &str) -> HashSet<u32> {
    n_str
        .split(" ")
        .map(str::parse::<u32>)
        .filter_map(Result::ok)
        .collect()
}
