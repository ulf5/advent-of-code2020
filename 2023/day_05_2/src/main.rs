fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, s)| s.split_whitespace())
        .unwrap()
        .map(str::trim)
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

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

    for i in 0.. {
        let s = maps.iter().rev().fold(i, |acc, v| translate(acc, v));
        if is_seed(s, &seeds) {
            dbg!(i);
            break;
        }
    }
}

fn is_seed(s: usize, seeds: &[usize]) -> bool {
    seeds
        .chunks(2)
        .find(|c| s >= c[0] && s < c[0] + c[1])
        .is_some()
}

fn translate(from: usize, v: &[(usize, usize, usize)]) -> usize {
    v.iter()
        .find(|(src, _, rng)| from >= *src && from < src + rng)
        .map(|(src, dst, _)| dst + (from - src))
        .unwrap_or(from)
}
