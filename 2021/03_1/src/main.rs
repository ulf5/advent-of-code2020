use std::collections::BTreeMap;

fn main() {
    let lines = include_str!("../input.txt").lines();
    let length = lines.clone().peekable().peek().unwrap().len();

    let pos_cnt: BTreeMap<(usize, bool), u32> = lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(2).unwrap() == 1)
                .collect::<Vec<bool>>()
        })
        .fold(BTreeMap::new(), |mut acc, elem| {
            elem.iter().rev().enumerate().for_each(|(x, &v)| {
                let cnt = acc.entry((x, v)).or_insert(0);
                *cnt += 1;
            });
            acc
        });
    let gamma = pos_cnt.keys().fold(0, |mut acc, (k, _)| {
        let one_or_zero =
            pos_cnt.get(&(*k, true)).unwrap_or(&0) > pos_cnt.get(&(*k, false)).unwrap_or(&0);
        acc |= (one_or_zero as u32) << k;
        acc
    });
    let mask = (1 << length) - 1;
    let epsilon = !gamma & mask;
    dbg!(epsilon * gamma);
}
