fn main() {
    let num: usize = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split('|').collect::<Vec<&str>>()[1]
                .split_whitespace()
                .collect::<Vec<&str>>()
        })
        .flatten()
        .filter(|x| [2, 3, 4, 7].contains(&x.len()))
        .count();
    dbg!(num);
}
