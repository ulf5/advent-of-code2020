use std::{
    cmp::min,
    collections::{BTreeMap, HashSet},
};

fn main() {
    let mut acc: BTreeMap<usize, usize> = BTreeMap::new();

    let lines: Vec<_> = include_str!("../input.txt").lines().collect();
    lines
        .iter()
        .filter_map(|l| l.split_once(":"))
        .enumerate()
        .for_each(|(i, (_, r))| {
            let cur = *acc.get(&i).unwrap_or(&0) + 1;
            let score = score_game(r);
            for n in i + 1..=min(i + score, lines.len()) {
                *acc.entry(n).or_insert(0) += cur;
            }
        });
    dbg!(acc.values().sum::<usize>() + lines.len());
}

fn score_game(game_str: &str) -> usize {
    game_str
        .split_once("|")
        .map(|(w, n)| (to_set(w), to_set(n)))
        .map(|(w, n)| w.intersection(&n).count())
        .unwrap()
}

fn to_set(n_str: &str) -> HashSet<u32> {
    n_str
        .split(" ")
        .map(str::parse::<u32>)
        .filter_map(Result::ok)
        .collect()
}
