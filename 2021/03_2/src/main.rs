use std::collections::BTreeMap;

fn main() {
    let lines = include_str!("../input.txt").lines();
    let length = lines.clone().peekable().peek().unwrap().len();

    let mut oxy_cands: Vec<_> = lines.clone().collect();
    let mut c02_cands: Vec<_> = lines.clone().collect();
    for i in 0..length {
        let pos_cnt_oxy = position_count(&oxy_cands);
        if oxy_cands.len() > 1 {
            oxy_cands.retain(|e| {
                let greater = pos_cnt_oxy.get(&(i, true)).unwrap_or(&0)
                    >= pos_cnt_oxy.get(&(i, false)).unwrap_or(&0);
                e.chars().nth(i).unwrap().to_digit(2).unwrap() == greater as u32
            })
        }
        let pos_cnt_c02 = position_count(&c02_cands);
        if c02_cands.len() > 1 {
            c02_cands.retain(|e| {
                let less = pos_cnt_c02.get(&(i, true)).unwrap_or(&0)
                    < pos_cnt_c02.get(&(i, false)).unwrap_or(&0);
                e.chars().nth(i).unwrap().to_digit(2).unwrap() == less as u32
            })
        }
    }

    let oxy = u32::from_str_radix(oxy_cands[0], 2).unwrap();
    let c02 = u32::from_str_radix(c02_cands[0], 2).unwrap();
    dbg!(oxy * c02);
}

fn position_count(lines: &Vec<&str>) -> BTreeMap<(usize, bool), u32> {
    let pos_cnt: BTreeMap<(usize, bool), u32> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(2).unwrap() == 1)
                .collect::<Vec<bool>>()
        })
        .fold(BTreeMap::new(), |mut acc, elem| {
            elem.iter().enumerate().for_each(|(x, &v)| {
                let cnt = acc.entry((x, v)).or_insert(0);
                *cnt += 1;
            });
            acc
        });
    pos_cnt
}
