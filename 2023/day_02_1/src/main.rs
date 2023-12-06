fn main() {
    let lines = include_str!("../input.txt").lines();
    let sum: u32 = lines.filter_map(do_stuff).sum();
    dbg!(sum);
}

fn do_stuff(l: &str) -> Option<u32> {
    let mut parts = l.split([':', ';']);
    let gid = Some(parts.next().unwrap()[5..].parse().unwrap());

    for part in parts {
        for color in part.split(',') {
            match color {
                green if green.ends_with(" green") => {
                    if green.trim().replace(" green", "").parse::<u32>().unwrap() > 13 {
                        return None;
                    }
                }
                blue if blue.ends_with(" blue") => {
                    if blue.trim().replace(" blue", "").parse::<u32>().unwrap() > 14 {
                        return None;
                    }
                }
                red if red.ends_with(" red") => {
                    if red.trim().replace(" red", "").parse::<u32>().unwrap() > 12 {
                        return None;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    gid
}
