use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines: Vec<(char, i64)> = vec![];
    for line in include_str!("../input.txt").lines() {
        lines.push((line.chars().nth(0).unwrap(), line[1..].parse().unwrap()));
    }

    dbg!(xmas_run(lines));
    Ok(())
}

fn xmas_run(lines: Vec<(char, i64)>) -> i64 {
    let mut dir = (0, 1);
    let (mut x, mut y) = (0, 0);
    for (cmd, val) in lines.iter() {
        match cmd {
            'N' => {
                x -= val;
            },
            'E' => {
                y += val;
            },
            'S' => {
                x += val;
            },
            'W' => {
                y -= val;
            },
            'F' => {
                x += dir.0 * val;
                y += dir.1 * val;
            },
            'R' => {
                dir = new_dir(dir, *val);
            },
            'L' => {
                dir = new_dir(dir, 0 - val);
            },
            _ => {
                dbg!(cmd, val);
                unreachable!();
            }
        }
    }
    x.abs() + y.abs()
}

fn new_dir(cur_dir: (i64, i64), deg: i64) -> (i64, i64) {
    let dir_to_num: HashMap<(i64, i64), i64> = [((0, 1), 0), ((1, 0), 1), ((0, -1), 2), ((-1, 0), 3)].iter().cloned().collect();
    let num_to_dir: HashMap<i64, (i64, i64)>  = [(0, (0, 1)), (1, (1, 0)), (2, (0, -1)), (3, (-1, 0))].iter().cloned().collect();
    let cnt_from_cur = ((deg / 90) + dir_to_num.get(&cur_dir).unwrap()).rem_euclid(4);
    num_to_dir[&cnt_from_cur]
}
