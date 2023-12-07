fn main() {
    let lines: Vec<_> = include_str!("../input.txt").lines()
        .map(str::as_bytes)
        .collect();

    let mut nums = vec![];
    for (i, l) in lines.iter().enumerate() {
        let mut buf = String::new();
        for (j, b) in l.iter().enumerate() {
            match b {
                b if b.is_ascii_digit() => {buf.push(*b as char)}
                _ => {
                    if buf.is_empty() {
                        continue;
                    }
                    check_number(j, &buf, i, &lines, &mut nums);
                    buf = String::new();
                }
            }
        }

        if !buf.is_empty() {
            check_number(l.len()-1, &buf, i, &lines, &mut nums);
        }
    }

    dbg!(&nums.iter().sum::<u32>());
}

fn check_number(j: usize, buf: &str, i: usize, lines: &[&[u8]], nums: &mut Vec<u32>) {
    for k in j.saturating_sub(buf.len()+1)..j+1 {
        if k >= lines[0].len() {
            continue;
        }
        for n in i.saturating_sub(1)..=i+1 {
            if n >= lines.len() {
                continue;
            }
            match lines[n][k] {
                d if d.is_ascii_digit() => continue,
                b'.' => continue,
                _ => {
                    nums.push(buf.parse().unwrap());
                    return;
                }
            }
        }
    }
}
