use std::collections::BTreeMap;

fn main() {
    let lines = include_str!("../input.txt").lines();
    let length = lines.clone().peekable().peek().unwrap().len();

    let pos_cnt: BTreeMap<(usize, bool), u32> = lines
        .map(|l| {
            u32::from_str_radix(l, 2).unwrap()
        })
        .fold(BTreeMap::new(), |mut acc, elem| {
            (0..length).for_each(|x| {
                let cnt = acc.entry((x, elem & 1 << x != 0)).or_insert(0);
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
