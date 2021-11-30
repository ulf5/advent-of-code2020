use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Loc {
    Wall,
    Floor,
    Occ,
    Unocc,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map: HashMap<(i64, i64), Loc> = HashMap::new();
    for (i, line) in include_str!("../input.txt").lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'L' => map.insert((i as i64, j as i64), Loc::Unocc),
                '.' => map.insert((i as i64, j as i64), Loc::Floor),
                _ => unreachable!(),
            };
        }
    }

    dbg!(xmas_run(map));
    Ok(())
}
fn xmas_run(mut map: HashMap<(i64, i64), Loc>) -> usize {
    loop {
        let (changed, new_map) = pass(map);
        map = new_map;
        if !changed {
            break;
        }
    }
    map.values().filter(|x| **x == Loc::Occ).count()
}

fn pass(map: HashMap<(i64, i64), Loc>) -> (bool, HashMap<(i64, i64), Loc>) {
    let mut new_map = HashMap::new();
    let mut changed = false;
    map.iter().for_each(|(k, v)| {
        let new_pos = new_pos(&map, *k);
        if new_pos != *v {
            changed = true;
        }
        new_map.insert(*k, new_pos);
    });

    (changed, new_map)
}

fn new_pos(map: &HashMap<(i64, i64), Loc>, (x, y): (i64, i64)) -> Loc {
    let mut cnt_occ = 0;
    let dirs = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for dir in dirs {
        let (mut cx, mut cy) = (x, y);
        loop {
            cx += dir.0;
            cy += dir.1;
            match map.get(&(cx, cy)).unwrap_or(&Loc::Wall) {
                Loc::Occ => {
                    cnt_occ += 1;
                    break;
                }
                Loc::Floor => {}
                _ => break,
            }
        }
    }
    let cur = map.get(&(x, y)).unwrap();
    match cur {
        Loc::Occ => {
            if cnt_occ > 4 {
                return Loc::Unocc;
            }
            *cur
        }
        Loc::Unocc => {
            if cnt_occ == 0 {
                return Loc::Occ;
            }
            *cur
        }
        _ => *cur,
    }
}
