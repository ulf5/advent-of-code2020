use std::cmp::min;

fn main() {
    let lines: Vec<_> = include_str!("../input.txt").lines()
        .map(str::as_bytes)
        .collect();

    let mut nums = vec![];
    for (i, l) in lines.iter().enumerate() {
        for (j, b) in l.iter().enumerate() {
            match b {
                b'*' => {
                    check_gear(j, i, &lines, &mut nums);
                }
                _ => {}
            }
        }
    }

    dbg!(&nums.iter().sum::<u32>());
}

fn check_gear(j: usize, i: usize, lines: &Vec<&[u8]>, nums: &mut Vec<u32>) {
    let mut nums_found = vec![];
    let mut found_cnt = 0;

    for n in i.saturating_sub(1)..=i+1 {
        if n >= lines.len() {
                continue;
        }
        let mut same = false;
        for k in j.saturating_sub(1)..=j+1 {
            if k >= lines[0].len() {
                continue;
            }
            match lines[n][k] {
                d if d.is_ascii_digit() => {
                    same = true;
                },
                _ => {
                    if same {
                        same = false;
                        found_cnt += 1;
                    }
                }
            }
        }
        if same {
            found_cnt += 1;
        }
    }
    if found_cnt == 2 {
        'outer: for n in i.saturating_sub(1)..=i+1 {
            if n >= lines.len() {
                continue;
            }
            for k in j.saturating_sub(1)..=j+1 {
                if k >= lines[0].len() {
                    continue;
                }
                if lines[n][k].is_ascii_digit() {
                    let mut buf = String::new();
                    let mut x = k;
                    while lines[n][x].is_ascii_digit() {
                        buf.insert(0, lines[n][x] as char);
                        if x == 0 {
                            break;
                        }
                        x -= 1;
                    }
                    let mut y = k+1;
                    while y < lines[0].len() && lines[n][y].is_ascii_digit() {
                        buf.push(lines[n][y] as char);
                        y += 1;
                    }
                    nums_found.push(buf.parse().unwrap());
                    if y > min(lines[0].len(), j) {
                        continue 'outer;
                    }
                }
            }
        }
    }
    if nums_found.len() == 2 {
        nums.push(nums_found.iter().product());
    }
}
