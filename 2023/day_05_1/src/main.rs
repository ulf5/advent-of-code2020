fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, s)| s.split_whitespace())
        .unwrap()
        .map(str::trim)
        .map(str::parse::<usize>)
        .map(Result::unwrap);

    let mut maps = vec![];

    let mut acc = vec![];
    for l in lines.filter(|s| !s.trim().is_empty()).skip(1) {
        if l.contains(':') {
            maps.push(acc);
            acc = vec![];
        } else {
            let mut spl = l.split_whitespace();
            let dst: usize = spl.next().unwrap().parse().unwrap();
            let src: usize = spl.next().unwrap().parse().unwrap();
            let rng: usize = spl.next().unwrap().parse().unwrap();
            acc.push((dst, src, rng));
        }
    }
    maps.push(acc);
    let min_loc = seeds
        .map(|seed| maps.iter().fold(seed, |acc, v| translate(acc, v)))
        .min();
    dbg!(min_loc);
}

fn translate(from: usize, v: &[(usize, usize, usize)]) -> usize {
    v.iter()
        .find(|(_, src, rng)| from >= *src && from < src + rng)
        .map(|(dst, src, _)| dst + (from - src))
        .unwrap_or(from)
}
